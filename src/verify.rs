use crate::exercise::{CompiledExercise, Exercise, Mode, State};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::env;

// 验证Exercise集合的编译/运行是否成功
pub fn verify<'a>(
    exercises: impl IntoIterator<Item = &'a Exercise>,
    progress: (usize, usize),
    verbose: bool,
    success_hints: bool,
) -> Result<(), &'a Exercise> {
    let (num_done, total) = progress;
    let bar = ProgressBar::new(total as u64);
    let mut percentage = (num_done as f32 / total as f32) * 100.0;
    bar.set_style(ProgressStyle::default_bar()
       .template("Progress: [{bar:60/green/red}] {pos}/{len} {msg}")
       .progress_chars("#-")
    );
    bar.set_position(num_done as u64);
    bar.set_message(format!("({:.1} %)", percentage));

    for exercise in exercises {
        let compile_result = match exercise.mode {
            Mode::Test => compile_and_test(exercise, RunMode::Interactive, verbose, success_hints),
            Mode::Compile => compile_and_run_interactively(exercise, success_hints),
            Mode::Clippy => compile_only(exercise, success_hints),
            Mode::BuildScript => compile_and_test(exercise, RunMode::Interactive, verbose, success_hints),
        };
        // 编译失败则返回错误
        if !compile_result.unwrap_or(false) {
            return Err(exercise);
        }
        // 更新进度条
        percentage += 100.0 / total as f32;
        bar.inc(1);
        bar.set_message(format!("({:.1} %)", percentage));
    }
    Ok(())
}

// 运行模式枚举
enum RunMode {
    Interactive,
    NonInteractive,
}

// 仅编译（不运行）
fn compile_only(exercise: &Exercise, success_hints: bool) -> Result<bool, ()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("Compiling {exercise}..."));
    progress_bar.enable_steady_tick(100);

    let _ = compile(exercise, &progress_bar);
    progress_bar.finish_and_clear();

    Ok(prompt_for_completion(exercise, None, success_hints))
}

// 交互式编译并运行
fn compile_and_run_interactively(exercise: &Exercise, success_hints: bool) -> Result<bool, ()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("Compiling {exercise}..."));
    progress_bar.enable_steady_tick(100);

    let compilation = compile(exercise, &progress_bar)?; // 编译失败则返回Err
    progress_bar.set_message(format!("Running {exercise}..."));
    let result = compilation.run();
    progress_bar.finish_and_clear();

    let output = match result {
        Ok(output) => output,
        Err(output) => {
            warn!("Ran {exercise} with errors");
            println!("{}", output.stdout);
            println!("{}", output.stderr);
            return Err(());
        }
    };

    Ok(prompt_for_completion(exercise, Some(output.stdout), success_hints))
}

// 编译并测试
fn compile_and_test(exercise: &Exercise, run_mode: RunMode, verbose: bool, success_hints: bool) -> Result<bool, ()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("Testing {exercise}..."));
    progress_bar.enable_steady_tick(100);

    let compilation = compile(exercise, &progress_bar)?; // 编译失败则返回Err
    let result = compilation.run();
    progress_bar.finish_and_clear();

    match result {
        Ok(output) => {
            if verbose {
                println!("{}", output.stdout);
            }
            // 根据运行模式返回不同结果
            if let RunMode::Interactive = run_mode {
                Ok(prompt_for_completion(exercise, None, success_hints))
            } else {
                Ok(true)
            }
        }
        Err(output) => {
            warn!(
                "Testing of {exercise} failed! Please try again. Here's the output:"
            );
            println!("{}", output.stdout);
            Err(())
        }
    }
}

// 编译Exercise并返回编译结果
fn compile<'a, 'b>(
    exercise: &'a Exercise,
    progress_bar: &'b ProgressBar,
) -> Result<CompiledExercise<'a>, ()> {
    let compilation_result = exercise.compile();
    match compilation_result {
        Ok(compilation) => Ok(compilation),
        Err(output) => {
            progress_bar.finish_and_clear();
            warn!(
                "Compiling of {exercise} failed! Please try again. Here's the output:"
            );
            println!("{}", output.stderr);
            Err(())
        }
    }
}

// 完成提示（返回是否继续）
fn prompt_for_completion(exercise: &Exercise, prompt_output: Option<String>, success_hints: bool) -> bool {
    let context = match exercise.state() {
        State::Done => return true,
        State::Pending(context) => context,
    };
    // 打印成功信息
    match exercise.mode {
        Mode::Compile => success!("Successfully ran {exercise}"),
        Mode::Test => success!("Successfully tested {exercise}"),
        Mode::Clippy => success!("Successfully compiled {exercise}"),
        Mode::BuildScript => success!("Successfully compiled {exercise}"),
    }

    let no_emoji = env::var("NO_EMOJI").is_ok();
    // Clippy成功消息
    let clippy_success_msg = if no_emoji {
        "The code is compiling, and Clippy is happy!"
    } else {
        "The code is compiling, and 🦀 Clippy 🦀 is happy!"
    };
    // 构造成功消息
    let success_msg = match exercise.mode {
        Mode::Compile => "The code is compiling!",
        Mode::Test => "The code is compiling, and the tests pass!",
        Mode::Clippy => clippy_success_msg,
        Mode::BuildScript => "Build script works!",
    };
    println!();
    // 打印带装饰的成功消息
    if no_emoji {
        println!("*** {success_msg} ***")
    } else {
        println!("🎉 🎉 {success_msg} 🎉 🎉")
    }
    println!();

    // 打印输出（如果有）
    if let Some(output) = prompt_output {
        println!("Output:");
        println!("{}", separator());
        println!("{output}");
        println!("{}", separator());
        println!();
    }

    // 打印提示（如果启用）
    if success_hints {
        println!("Hints:");
        println!("{}", separator());
        println!("{}", exercise.hint);
        println!("{}", separator());
        println!();
    }

    // 打印后续操作提示
    println!("You can keep working on this exercise,");
    println!(
        "or jump into the next one by removing the {} comment:",
        style("I AM NOT DONE").bold()
    );
    println!();

    // 打印上下文代码
    for context_line in context {
        let formatted_line = if context_line.important {
            format!("{}", style(context_line.line).bold())
        } else {
            context_line.line.to_string()
        };
        println!(
            ":{:>2} {} {}",
            style(context_line.number).blue().bold(),
            style("|").blue(),
            formatted_line
        );
    }

    false
}

// 生成分隔线样式
fn separator() -> console::StyledObject<&'static str> {
    style("====================").bold()
}

// 辅助宏（假设项目中已定义warn!/success!）
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        eprintln!("{} {}", style("[WARN]").yellow().bold(), format!($($arg)*));
    };
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {
        println!("{} {}", style("[SUCCESS]").green().bold(), format!($($arg)*));
    };
}
