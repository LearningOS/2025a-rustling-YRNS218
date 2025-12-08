// 1. 引入依赖（需在 Cargo.toml 中配置）
use std::process::Command;
use indicatif::ProgressBar;
use anyhow::{Context, Result}; // 用 anyhow 简化错误处理


// 2. 定义模块与结构体（模拟 `crate::exercise` 模块）
mod exercise {
    use super::Result;

    #[derive(Debug)]
    pub enum Mode {
        Test,
        Compile,
        Clippy,
        BuildScript,
    }

    #[derive(Debug)]
    pub struct Exercise {
        pub path: String,
        pub mode: Mode,
    }

    impl Exercise {
        // 模拟 `compile` 方法：调用 rustc 编译当前 exercise
        pub fn compile(&self) -> Result<Compilation> {
            let output = Command::new("rustc")
                .arg(&self.path)
                .output()
                .context("Failed to start rustc")?;

            if output.status.success() {
                Ok(Compilation {})
            } else {
                Err(anyhow::anyhow!(
                    "Compilation failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ))
            }
        }
    }

    // 模拟编译结果（用于后续运行）
    pub struct Compilation;
    impl Compilation {
        // 模拟 `run` 方法：运行编译后的二进制文件
        pub fn run(&self) -> Result<RunOutput> {
            // 简化：实际应运行编译后的二进制（这里用 echo 模拟）
            let output = Command::new("echo")
                .arg("Hello from compiled binary!")
                .output()
                .context("Failed to run binary")?;

            Ok(RunOutput {
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            })
        }
    }

    // 模拟运行结果的输出结构
    pub struct RunOutput {
        pub stdout: String,
        pub stderr: String,
    }
}
use exercise::{Exercise, Mode, Compilation, RunOutput};


// 3. 模拟 `crate::verify::test` 函数
mod verify {
    use super::{Exercise, Result};

    pub fn test(_exercise: &Exercise, _verbose: bool) -> Result<()> {
        // 模拟测试逻辑（实际应执行测试用例）
        Ok(())
    }
}
use verify::test;


// 4. 实现 `run` 函数（你的片段）
pub fn run(exercise: &Exercise, verbose: bool) -> Result<()> {
    match exercise.mode {
        Mode::Test => test(exercise, verbose),
        Mode::Compile => compile_and_run(exercise),
        Mode::Clippy => compile_and_run(exercise), // 实际 Clippy 应调用 cargo clippy
        Mode::BuildScript => test(exercise, verbose),
    }
}


// 5. 实现 `reset` 函数（你的片段）
pub fn reset(exercise: &Exercise) -> Result<()> {
    let command = Command::new("git")
        .args(["stash", "--"])
        .arg(&exercise.path)
        .spawn()
        .context("Failed to spawn git stash")?;

    let status = command.wait().context("Failed to wait for git stash")?;
    if status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Git stash failed"))
    }
}


// 6. 实现 `compile_and_run` 函数（你的片段）
fn compile_and_run(exercise: &Exercise) -> Result<()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("Compiling {exercise:?}..."));
    progress_bar.enable_steady_tick(100);

    // 编译逻辑
    let compilation = match exercise.compile() {
        Ok(compilation) => compilation,
        Err(err) => {
            progress_bar.finish_and_clear();
            eprintln!("Compilation of {:?} failed! Compiler error message:\n{}", exercise, err);
            return Err(err);
        }
    };

    // 运行逻辑
    progress_bar.set_message(format!("Running {exercise:?}..."));
    let result = compilation.run();
    progress_bar.finish_and_clear();

    // 处理运行结果
    match result {
        Ok(output) => {
            println!("{}", output.stdout);
            println!("Successfully ran {:?}", exercise);
            Ok(())
        }
        Err(err) => {
            // 简化：实际应从 err 中提取 stdout/stderr
            eprintln!("Ran {:?} with errors", exercise);
            Err(err)
        }
    }
}


// 7. Cargo.toml 依赖（需手动添加）
/*
[dependencies]
indicatif = "0.17"
anyhow = "1.0"
*/
