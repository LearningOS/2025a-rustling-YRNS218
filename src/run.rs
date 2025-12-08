use std::process::{Command, Output};
use std::path::Path;

use crate::exercise::{Exercise, Mode};
use crate::verify::test;
use indicatif::ProgressBar;
use log::{warn, info}; // 假设使用log crate进行日志输出

// 成功日志宏定义，类似warn!
#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {
        info!("\x1b[32m{}\x1b[0m", format!($($arg)*));
    };
}

// Invoke the rust compiler on the path of the given exercise,
// and run the ensuing binary.
// The verbose argument helps determine whether or not to show
// the output from the test harnesses (if the mode of the exercise is test)
pub fn run(exercise: &Exercise, verbose: bool) -> Result<(), ()> {
    match exercise.mode {
        Mode::Test => test(exercise, verbose)?,
        Mode::Compile => compile_and_run(exercise)?,
        Mode::Clippy => compile_and_run(exercise)?,
        Mode::BuildScript => test(exercise, verbose)?,
    }
    Ok(())
}

// Resets the exercise by stashing the changes.
pub fn reset(exercise: &Exercise) -> Result<(), ()> {
    let command = Command::new("git")
        .args(["stash", "--"])
        .arg(&exercise.path)
        .spawn();

    match command {
        Ok(mut child) => {
            // 等待命令执行完成并检查状态
            if child.wait().map_err(|_| ())?.success() {
                Ok(())
            } else {
                Err(())
            }
        }
        Err(_) => Err(()),
    }
}

// Invoke the rust compiler on the path of the given exercise
// and run the ensuing binary.
// This is strictly for non-test binaries, so output is displayed
fn compile_and_run(exercise: &Exercise) -> Result<(), ()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("Compiling {exercise}..."));
    progress_bar.enable_steady_tick(100);

    let compilation_result = exercise.compile();
    let compilation = match compilation_result {
        Ok(compilation) => compilation,
        Err(output) => {
            progress_bar.finish_and_clear();
            warn!(
                "Compilation of {} failed!, Compiler error message:\n",
                exercise
            );
            println!("{}", output.stderr);
            return Err(());
        }
    };

    progress_bar.set_message(format!("Running {exercise}..."));
    let result = compilation.run();
    progress_bar.finish_and_clear();

    match result {
        Ok(output) => {
            println!("{}", output.stdout);
            success!("Successfully ran {}", exercise);
            Ok(())
        }
        Err(output) => {
            println!("{}", output.stdout);
            println!("{}", output.stderr);

            warn!("Ran {} with errors", exercise);
            Err(())
        }
    }
}

// 为Exercise实现Display trait以支持格式化输出
impl std::fmt::Display for Exercise {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.file_name().and_then(|n| n.to_str()).unwrap_or("exercise"))
    }
}

// 假设的Compilation结构体，用于表示编译结果
pub struct Compilation {
    // 编译相关的信息，如输出路径等
    output_path: String,
}

impl Compilation {
    // 运行编译后的二进制文件
    fn run(&self) -> Result<Output, Output> {
        Command::new(&self.output_path)
            .output()
            .map_err(|e| Output {
                status: std::process::ExitStatus::from_raw(1),
                stdout: Vec::new(),
                stderr: e.to_string().into_bytes(),
            })
            .and_then(|output| {
                if output.status.success() {
                    Ok(output)
                } else {
                    Err(output)
                }
            })
    }
}

// 为Exercise补充compile方法的默认实现
impl Exercise {
    fn compile(&self) -> Result<Compilation, Output> {
        // 简化的编译逻辑，实际实现可能更复杂
        let output_path = format!("./{}", self.path.file_stem().unwrap().to_str().unwrap());
        let status = Command::new("rustc")
            .arg(&self.path)
            .arg("-o")
            .arg(&output_path)
            .output()
            .map_err(|e| Output {
                status: std::process::ExitStatus::from_raw(1),
                stdout: Vec::new(),
                stderr: e.to_string().into_bytes(),
            })?;

        if status.status.success() {
            Ok(Compilation { output_path })
        } else {
            Err(status)
        }
    }
}
