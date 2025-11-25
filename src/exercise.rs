use regex::Regex;
use serde::Deserialize;
use std::env;
use std::fmt::{self, Display, Formatter};
use std::fs::{self, remove_file, File};
use std::io::Read;
use std::path::PathBuf;
use std::process::{self, Command, Output};

const RUSTC_COLOR_ARGS: &[&str] = &["--color", "always"];
const RUSTC_EDITION_ARGS: &[&str] = &["--edition", "2021"];
const I_AM_DONE_REGEX: &str = r"(?m)^\s*//?\s*I\s+AM\s+NOT\s+DONE";
const CONTEXT: usize = 2;
const CLIPPY_CARGO_TOML_PATH: &str = "./exercises/clippy/Cargo.toml";
const BUILD_SCRIPT_CARGO_TOML_PATH: &str = "./exercises/tests/Cargo.toml";

// 获取临时文件名（确保唯一性）
#[inline]
fn temp_file() -> String {
    let thread_id: String = format!("{:?}", std::thread::current().id())
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();
    format!("./temp_{}_{}", process::id(), thread_id)
}

// 练习模式枚举
#[derive(Deserialize, Copy, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Compile,    // 编译为二进制文件
    Test,       // 作为测试编译
    Clippy,     // 使用 clippy 检查
    BuildScript,// 使用构建脚本运行
}

// 练习列表结构体
#[derive(Deserialize)]
pub struct ExerciseList {
    pub exercises: Vec<Exercise>,
}

// 练习结构体（从 info.toml 反序列化）
#[derive(Deserialize, Debug)]
pub struct Exercise {
    pub name: String,           // 练习名称
    pub path: PathBuf,          // 源代码路径
    pub mode: Mode,             // 练习模式
    pub hint: String,           // 提示信息
}

// 练习状态枚举
#[derive(PartialEq, Debug)]
pub enum State {
    Done,                       // 已完成
    Pending(Vec<ContextLine>),  // 未完成（包含上下文信息）
}

// 未完成练习的上下文信息
#[derive(PartialEq, Debug)]
pub struct ContextLine {
    pub line: String,           // 待完成的源代码行
    pub number: usize,          // 行号
    pub important: bool,        // 是否为关键行
}

// 编译后的练习结构体
pub struct CompiledExercise<'a> {
    exercise: &'a Exercise,
    _handle: FileHandle,
}

// 练习输出结果
#[derive(Debug)]
pub struct ExerciseOutput {
    pub stdout: String,         // 标准输出内容
    pub stderr: String,         // 标准错误内容
}

// 文件句柄（用于自动清理临时文件）
struct FileHandle;

impl Drop for FileHandle {
    fn drop(&mut self) {
        clean();
    }
}

// 清理临时文件
#[inline]
fn clean() {
    let temp_path = temp_file();
    let _ = remove_file(&temp_path); // 忽略删除失败（文件可能已被清理）
}

impl Exercise {
    // 编译练习
    pub fn compile(&self) -> Result<CompiledExercise, ExerciseOutput> {
        let cmd_output = match self.mode {
            Mode::Compile => Command::new("rustc")
                .arg(self.path.to_str().unwrap())
                .arg("-o")
                .arg(temp_file())
                .args(RUSTC_COLOR_ARGS)
                .args(RUSTC_EDITION_ARGS)
                .output()
                .expect("Failed to run rustc command"),

            Mode::Test => Command::new("rustc")
                .arg("--test")
                .arg(self.path.to_str().unwrap())
                .arg("-o")
                .arg(temp_file())
                .args(RUSTC_COLOR_ARGS)
                .args(RUSTC_EDITION_ARGS)
                .output()
                .expect("Failed to run rustc test command"),

            Mode::Clippy => {
                // 生成 Clippy 所需的 Cargo.toml
                let cargo_toml = format!(
                    r#"[package]
name = "{}"
version = "0.0.1"
edition = "2021"

[[bin]]
name = "{}"
path = "{}.rs"
"#,
                    self.name, self.name, self.name
                );

                let cargo_toml_error_msg = if env::var("NO_EMOJI").is_ok() {
                    "Failed to write Clippy Cargo.toml file."
                } else {
                    "Failed to write 📎 Clippy 📎 Cargo.toml file."
                };

                // 写入 Cargo.toml
                fs::write(CLIPPY_CARGO_TOML_PATH, cargo_toml)
                    .expect(cargo_toml_error_msg);

                // 预编译（确保可执行文件存在）
                let _ = Command::new("rustc")
                    .arg(self.path.to_str().unwrap())
                    .arg("-o")
                    .arg(temp_file())
                    .args(RUSTC_COLOR_ARGS)
                    .args(RUSTC_EDITION_ARGS)
                    .output()
                    .expect("Failed to compile for Clippy");

                // 清理缓存（解决 Clippy 历史问题）
                Command::new("cargo")
                    .arg("clean")
                    .arg("--manifest-path")
                    .arg(CLIPPY_CARGO_TOML_PATH)
                    .args(RUSTC_COLOR_ARGS)
                    .output()
                    .expect("Failed to run 'cargo clean'");

                // 运行 Clippy 检查
                Command::new("cargo")
                    .arg("clippy")
                    .arg("--manifest-path")
                    .arg(CLIPPY_CARGO_TOML_PATH)
                    .args(RUSTC_COLOR_ARGS)
                    .args(&["--", "-D", "warnings", "-D", "clippy::float_cmp"])
                    .output()
                    .expect("Failed to run clippy")
            }

            Mode::BuildScript => {
                // 生成构建脚本的 Cargo.toml
                let cargo_toml = format!(
                    r#"[package]
name = "{}"
version = "0.0.1"
edition = "2021"

[[bin]]
name = "{}"
path = "{}.rs"
"#,
                    self.name, self.name, self.name
                );

                let cargo_toml_error_msg = if env::var("NO_EMOJI").is_ok() {
                    "Failed to write build script Cargo.toml file."
                } else {
                    "Failed to write 🔨 Build Script 🔨 Cargo.toml file."
                };

                // 写入 Cargo.toml
                fs::write(BUILD_SCRIPT_CARGO_TOML_PATH, cargo_toml)
                    .expect(cargo_toml_error_msg);

                // 运行测试
                Command::new("cargo")
                    .arg("test")
                    .arg("--manifest-path")
                    .arg(BUILD_SCRIPT_CARGO_TOML_PATH)
                    .output()
                    .expect("Failed to run 'cargo test' for build script")
            }
        };

        // 根据编译结果返回对应值
        if cmd_output.status.success() {
            Ok(CompiledExercise {
                exercise: self,
                _handle: FileHandle,
            })
        } else {
            Err(ExerciseOutput {
                stdout: String::from_utf8_lossy(&cmd_output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&cmd_output.stderr).to_string(),
            })
        }
    }

    // 运行编译后的练习
    pub fn run(&self) -> Result<ExerciseOutput, ExerciseOutput> {
        let arg = match self.mode {
            Mode::Test => "--show-output",
            Mode::BuildScript => {
                return Ok(ExerciseOutput {
                    stdout: String::new(),
                    stderr: String::new(),
                })
            }
            _ => "",
        };

        let cmd_output = Command::new(temp_file())
            .arg(arg)
            .output()
            .expect("Failed to run compiled exercise");

        let output = ExerciseOutput {
            stdout: String::from_utf8_lossy(&cmd_output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&cmd_output.stderr).to_string(),
        };

        if cmd_output.status.success() {
            Ok(output)
        } else {
            Err(output)
        }
    }

    // 获取练习状态（已完成/未完成）
    pub fn state(&self) -> State {
        // 读取练习源代码
        let mut source_file = File::open(&self.path)
            .expect("Failed to open exercise file");
        let mut source = String::new();
        source_file.read_to_string(&mut source)
            .expect("Failed to read exercise file");

        // 检查是否包含 "I AM NOT DONE" 标记
        let re = Regex::new(I_AM_DONE_REGEX).unwrap();
        if !re.is_match(&source) {
            return State::Done;
        }

        // 提取未完成行的上下文
        let matched_line_index = source
            .lines()
            .enumerate()
            .find(|(_, line)| re.is_match(line))
            .map(|(i, _)| i)
            .expect("Matched line should exist");

        let min_line = (matched_line_index as i32 - CONTEXT as i32).max(0) as usize;
        let max_line = matched_line_index + CONTEXT;

        let context = source
            .lines()
            .enumerate()
            .filter(|(i, _)| *i >= min_line && *i <= max_line)
            .map(|(i, line)| ContextLine {
                line: line.to_string(),
                number: i + 1, // 行号从 1 开始
                important: i == matched_line_index,
            })
            .collect();

        State::Pending(context)
    }

    // 检查练习是否看起来已完成
    pub fn looks_done(&self) -> bool {
        self.state() == State::Done
    }
}

// 实现 CompiledExercise 的运行方法
impl<'a> CompiledExercise<'a> {
    pub fn run(&self) -> Result<ExerciseOutput, ExerciseOutput> {
        self.exercise.run()
    }
}

// 为 Exercise 实现 Display trait
impl Display for Exercise {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.path.to_str().unwrap())
    }
}
