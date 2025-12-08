#![edition = "2021"]
#![deny(warnings)]

// 标准库导入
use std::ffi::OsStr;
use std::fs;
use std::io::{self, prelude::*};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

// 外部依赖导入（需在 Cargo.toml 中声明）
use argh::FromArgs;
use console::Emoji;
use notify::DebouncedEvent;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use tokio::task;

// ======================== 宏定义 ========================
#[macro_export]
macro_rules! println_success {
    ($($arg:tt)*) => {
        println!("{} {}", Emoji("✅", "✓"), format!($($arg)*));
    };
}

#[macro_export]
macro_rules! println_error {
    ($($arg:tt)*) => {
        println!("{} {}", Emoji("❌", "✗"), format!($($arg)*));
    };
}

// ======================== 常量定义 ========================
const WELCOME: &str = "Welcome to Rustlings!";
const DEFAULT_OUT: &str = "Please use a subcommand. Run `rustlings --help` for more information.";
const FINISH_LINE: &str = "You have completed all exercises! Great job!";
const VERSION: &str = "5.5.1";

// ======================== 命令行参数定义 ========================
#[derive(FromArgs, PartialEq, Debug)]
/// Rustlings: 轻量级 Rust 练习工具
struct Args {
    /// 显示测试/编译输出
    #[argh(switch)]
    nocapture: bool,
    /// 显示版本号
    #[argh(switch, short = 'v')]
    version: bool,
    /// 子命令
    #[argh(subcommand)]
    nested: Option<Subcommands>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Subcommands {
    Verify(VerifyArgs),
    Watch(WatchArgs),
    Run(RunArgs),
    Reset(ResetArgs),
    Hint(HintArgs),
    List(ListArgs),
    Lsp(LspArgs),
    CicvVerify(CicvVerifyArgs),
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "cicvverify", description = "批量验证所有练习并生成报告")]
struct CicvVerifyArgs {}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "verify")]
/// 按推荐顺序验证所有练习
struct VerifyArgs {}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "watch")]
/// 文件修改时自动重新验证
struct WatchArgs {
    /// 验证成功时显示提示
    #[argh(switch)]
    success_hints: bool,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "run")]
/// 运行/测试单个练习
struct RunArgs {
    #[argh(positional)]
    /// 练习名称（或 "next" 运行下一个未完成练习）
    name: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "reset")]
/// 重置单个练习（模拟 git stash）
struct ResetArgs {
    #[argh(positional)]
    /// 练习名称
    name: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "hint")]
/// 获取单个练习的提示
struct HintArgs {
    #[argh(positional)]
    /// 练习名称
    name: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "lsp")]
/// 生成 rust-analyzer 配置文件
struct LspArgs {}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "list")]
/// 列出所有可用练习
struct ListArgs {
    /// 仅显示练习路径
    #[argh(switch, short = 'p')]
    paths: bool,
    /// 仅显示练习名称
    #[argh(switch, short = 'n')]
    names: bool,
    /// 过滤练习名称（逗号分隔）
    #[argh(option, short = 'f')]
    filter: Option<String>,
    /// 仅显示未完成练习
    #[argh(switch, short = 'u')]
    unsolved: bool,
    /// 仅显示已完成练习
    #[argh(switch, short = 's')]
    solved: bool,
}

// ======================== 数据结构定义 ========================
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ExerciseCheckList {
    pub exercises: Vec<ExerciseResult>,
    pub user_name: Option<String>,
    pub statistics: ExerciseStatistics,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ExerciseResult {
    pub name: String,
    pub result: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ExerciseStatistics {
    pub total_exercises: usize,
    pub total_succeeds: usize,
    pub total_failures: usize,
    pub total_time: u32,
}

// ======================== Exercise 模块 ========================
pub mod exercise {
    use super::*;
    use serde::Deserialize;
    use std::path::PathBuf;

    #[derive(Debug, Deserialize, Clone)]
    pub struct ExerciseList {
        pub exercises: Vec<Exercise>,
    }

    #[derive(Debug, Deserialize, Clone)]
    pub struct Exercise {
        pub name: String,
        pub path: PathBuf,
        pub mode: Mode,
        pub hint: String,
    }

    #[derive(Debug, Deserialize, Clone)]
    #[serde(rename_all = "lowercase")]
    pub enum Mode {
        Compile,
        Test,
        Clippy,
        BuildScript,
    }

    impl Exercise {
        /// 判断练习是否完成（简化实现：编译通过即完成）
        pub fn looks_done(&self) -> bool {
            match self.mode {
                Mode::Compile => self.compile_check(),
                Mode::Test => self.test_check(),
                Mode::Clippy => self.clippy_check(),
                Mode::BuildScript => self.compile_check(),
            }
        }

        /// 编译检查
        fn compile_check(&self) -> bool {
            Command::new("rustc")
                .arg(&self.path)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        }

        /// 测试检查
        fn test_check(&self) -> bool {
            Command::new("cargo")
                .arg("test")
                .arg("--manifest-path")
                .arg(self.path.parent().unwrap().join("Cargo.toml"))
                .arg("--")
                .arg(self.path.file_stem().unwrap())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        }

        /// Clippy 检查
        fn clippy_check(&self) -> bool {
            Command::new("cargo")
                .arg("clippy")
                .arg("--manifest-path")
                .arg(self.path.parent().unwrap().join("Cargo.toml"))
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        }
    }
}

// ======================== Project 模块（LSP 支持） ========================
pub mod project {
    use super::*;
    use std::path::PathBuf;

    pub struct RustAnalyzerProject {
        pub crates: Vec<Crate>,
    }

    #[derive(Debug, Clone)]
    pub struct Crate {
        root_module: PathBuf,
    }

    impl RustAnalyzerProject {
        pub fn new() -> Self {
            Self { crates: vec![] }
        }

        pub fn get_sysroot_src(&mut self) -> std::io::Result<()> {
            // 简化实现：实际需获取 Rust sysroot 路径
            Ok(())
        }

        pub fn exercises_to_json(&mut self) -> std::io::Result<()> {
            // 简化实现：实际需解析练习生成 rust-project.json 内容
            Ok(())
        }

        pub fn write_to_disk(&self) -> std::io::Result<()> {
            fs::write(
                "rust-project.json",
                r#"{
                    "roots": ["./exercises"],
                    "crates": []
                }"#,
            )
        }
    }
}

// ======================== Run 模块（运行/重置练习） ========================
pub mod run {
    use super::*;

    /// 运行单个练习
    pub fn run(exercise: &exercise::Exercise, verbose: bool) -> Result<(), ()> {
        match exercise.mode {
            exercise::Mode::Compile => run_compile(exercise, verbose),
            exercise::Mode::Test => run_test(exercise, verbose),
            exercise::Mode::Clippy => run_clippy(exercise, verbose),
            exercise::Mode::BuildScript => run_compile(exercise, verbose),
        }
    }

    /// 重置单个练习（模拟 git stash）
    pub fn reset(exercise: &exercise::Exercise) -> Result<(), ()> {
        println_success!("正在重置练习: {}", exercise.name);
        // 实际项目中需调用 git stash -- <file>
        Command::new("git")
            .arg("stash")
            .arg("--")
            .arg(&exercise.path)
            .status()
            .map_err(|e| {
                println_error!("重置失败: {}", e);
            })?;
        Ok(())
    }

    /// 运行编译型练习
    fn run_compile(exercise: &exercise::Exercise, verbose: bool) -> Result<(), ()> {
        let output = Command::new("rustc")
            .arg(&exercise.path)
            .output()
            .map_err(|e| {
                println_error!("编译失败: {}", e);
            })?;

        if verbose || !output.status.success() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }

        if output.status.success() {
            println_success!("练习 {} 编译成功!", exercise.name);
            Ok(())
        } else {
            println_error!("练习 {} 编译失败!", exercise.name);
            Err(())
        }
    }

    /// 运行测试型练习
    fn run_test(exercise: &exercise::Exercise, verbose: bool) -> Result<(), ()> {
        let output = Command::new("cargo")
            .arg("test")
            .arg("--manifest-path")
            .arg(exercise.path.parent().unwrap().join("Cargo.toml"))
            .arg("--")
            .arg(exercise.path.file_stem().unwrap())
            .output()
            .map_err(|e| {
                println_error!("测试失败: {}", e);
            })?;

        if verbose || !output.status.success() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }

        if output.status.success() {
            println_success!("练习 {} 测试成功!", exercise.name);
            Ok(())
        } else {
            println_error!("练习 {} 测试失败!", exercise.name);
            Err(())
        }
    }

    /// 运行 Clippy 型练习
    fn run_clippy(exercise: &exercise::Exercise, verbose: bool) -> Result<(), ()> {
        let output = Command::new("cargo")
            .arg("clippy")
            .arg("--manifest-path")
            .arg(exercise.path.parent().unwrap().join("Cargo.toml"))
            .output()
            .map_err(|e| {
                println_error!("Clippy 检查失败: {}", e);
            })?;

        if verbose || !output.status.success() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }

        if output.status.success() {
            println_success!("练习 {} Clippy 检查通过!", exercise.name);
            Ok(())
        } else {
            println_error!("练习 {} Clippy 检查失败!", exercise.name);
            Err(())
        }
    }
}

// ======================== Verify 模块（验证练习） ========================
pub mod verify {
    use super::*;

    /// 验证多个练习
    pub fn verify<I>(
        exercises: I,
        _range: (usize, usize),
        verbose: bool,
        _success_hints: bool,
    ) -> Result<(), &'static exercise::Exercise>
    where
        I: Iterator<Item = &'static exercise::Exercise>,
    {
        for exercise in exercises {
            if run::run(exercise, verbose).is_err() {
                return Err(exercise);
            }
        }
        Ok(())
    }
}

// ======================== 核心工具函数 ========================
/// 查找指定名称的练习
fn find_exercise<'a>(name: &str, exercises: &'a [exercise::Exercise]) -> &'a exercise::Exercise {
    if name.eq("next") {
        // 查找下一个未完成的练习
        exercises
            .iter()
            .find(|e| !e.looks_done())
            .unwrap_or_else(|| {
                println_success!("🎉 恭喜！所有练习已完成！");
                std::process::exit(0);
            })
    } else {
        // 按名称查找练习
        exercises
            .iter()
            .find(|e| e.name == name)
            .unwrap_or_else(|| {
                println_error!("未找到练习: {}", name);
                std::process::exit(1);
            })
    }
}

/// 检查 rustc 是否安装
fn rustc_exists() -> bool {
    Command::new("rustc")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_or(false, |s| s.success())
}

/// 启动 watch 模式的交互 shell
fn spawn_watch_shell(
    failed_exercise_hint: &Arc<Mutex<Option<String>>>,
    should_quit: Arc<AtomicBool>,
) {
    let failed_exercise_hint = Arc::clone(failed_exercise_hint);
    println!("📌 Watch 模式 - 输入 'help' 查看命令");

    thread::spawn(move || loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                match input {
                    "hint" => {
                        if let Some(hint) = &*failed_exercise_hint.lock().unwrap() {
                            println!("💡 提示: {}", hint);
                        } else {
                            println!("ℹ️  暂无失败练习的提示");
                        }
                    }
                    "clear" => println!("\x1B[2J\x1B[1;1H"),
                    "quit" => {
                        should_quit.store(true, Ordering::SeqCst);
                        println!("👋 再见！");
                        break;
                    }
                    "help" => {
                        println!("📋 可用命令：");
                        println!("  hint   - 显示当前失败练习的提示");
                        println!("  clear  - 清屏");
                        println!("  quit   - 退出 watch 模式");
                        println!("  !<cmd> - 执行系统命令（如 !rustc --explain E0381）");
                        println!("  help   - 显示此帮助");
                    }
                    cmd if cmd.starts_with('!') => {
                        let parts: Vec<&str> = cmd[1..].split_whitespace().collect();
                        if parts.is_empty() {
                            println_error!("请输入命令（如 !rustc --version）");
                        } else if let Err(e) = Command::new(parts[0])
                            .args(&parts[1..])
                            .status()
                        {
                            println_error!("命令执行失败: {}", e);
                        }
                    }
                    "" => (),
                    _ => println_error!("未知命令: {}", input),
                }
            }
            Err(e) => println_error!("读取输入失败: {}", e),
        }
    });
}

// ======================== Watch 模式 ========================
enum WatchStatus {
    Finished,
    Unfinished,
}

/// 启动 watch 模式（文件修改时自动验证）
fn watch(
    exercises: &[exercise::Exercise],
    verbose: bool,
    success_hints: bool,
) -> notify::Result<WatchStatus> {
    fn clear_screen() {
        println!("\x1Bc");
    }

    // 创建通道监听文件变化
    let (tx, rx) = channel();
    let should_quit = Arc::new(AtomicBool::new(false));

    // 初始化文件监视器
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1))?;
    fs::create_dir_all("./exercises").ok(); // 确保目录存在
    watcher.watch(Path::new("./exercises"), RecursiveMode::Recursive)?;

    clear_screen();
    println_success!("Watch 模式已启动 - 编辑 exercises 目录下的文件自动验证");

    // 初始化失败练习提示
    let failed_exercise_hint = Arc::new(Mutex::new(None));
    spawn_watch_shell(&failed_exercise_hint, Arc::clone(&should_quit));

    // 转换为静态引用（测试场景，生产环境需优化生命周期）
    let static_exercises: Vec<&'static exercise::Exercise> = exercises
        .iter()
        .map(|e| unsafe { &*(e as *const _) })
        .collect();

    // 主循环
    loop {
        if should_quit.load(Ordering::SeqCst) {
            return Ok(WatchStatus::Unfinished);
        }

        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(event) => match event {
                DebouncedEvent::Write(path) | DebouncedEvent::Create(path) => {
                    if path.extension() == Some(OsStr::new("rs")) {
                        clear_screen();
                        println!("🔄 文件变更: {}", path.display());

                        // 重新验证所有练习
                        let result = verify::verify(
                            static_exercises.iter().copied(),
                            (0, exercises.len()),
                            verbose,
                            success_hints,
                        );

                        match result {
                            Ok(_) => {
                                if exercises.iter().all(|e| e.looks_done()) {
                                    return Ok(WatchStatus::Finished);
                                }
                            }
                            Err(ex) => {
                                *failed_exercise_hint.lock().unwrap() = Some(ex.hint.clone());
                                println_error!("练习 {} 验证失败 - 输入 'hint' 查看提示", ex.name);
                            }
                        }
                    }
                }
                DebouncedEvent::Remove(_) | DebouncedEvent::Rename(_, _) => {
                    clear_screen();
                    println!("🔄 文件变更，重新验证...");
                    let result = verify::verify(
                        static_exercises.iter().copied(),
                        (0, exercises.len()),
                        verbose,
                        success_hints,
                    );
                    if let Err(ex) = result {
                        *failed_exercise_hint.lock().unwrap() = Some(ex.hint.clone());
                    }
                }
                _ => {}
            },
            Err(RecvTimeoutError::Timeout) => continue,
            Err(RecvTimeoutError::Disconnected) => break,
        }
    }

    Ok(WatchStatus::Unfinished)
}

// ======================== 主函数 ========================
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    // 解析命令行参数
    let args: Args = argh::from_env();

    // 显示版本号
    if args.version {
        println!("rustlings v{}", VERSION);
        std::process::exit(0);
    }

    // 欢迎信息
    if args.nested.is_none() {
        println!("\n{}", WELCOME);
    }

    // 检查运行目录（需存在 info.toml）
    if !Path::new("info.toml").exists() {
        println_error!("必须在 rustlings 根目录运行（缺少 info.toml）");
        println!("💡 尝试: cd rustlings/");
        std::process::exit(1);
    }

    // 检查 rustc 是否安装
    if !rustc_exists() {
        println_error!("未找到 rustc - 请先安装 Rust");
        println!("💡 安装指南: https://www.rust-lang.org/tools/install");
        std::process::exit(1);
    }

    // 加载练习列表
    let toml_str = fs::read_to_string("info.toml").unwrap_or_else(|_| {
        r#"
        [exercises]
        exercises = []
        "#.to_string()
    });
    let exercise_list = toml::from_str::<exercise::ExerciseList>(&toml_str)
        .expect("解析 info.toml 失败");
    let exercises = exercise_list.exercises;
    let verbose = args.nocapture;

    // 处理子命令
    let command = args.nested.unwrap_or_else(|| {
        println!("{}", DEFAULT_OUT);
        std::process::exit(0);
    });

    match command {
        Subcommands::List(subargs) => {
            // 列出所有练习
            if !subargs.paths && !subargs.names {
                println!("{:<20}\t{:<50}\t{:<8}", "名称", "路径", "状态");
                println!("{}", "-".repeat(80));
            }

            let mut done_count = 0;
            let filters = subargs.filter.clone().unwrap_or_default().to_lowercase();
            let filter_parts: Vec<&str> = filters.split(',').map(|s| s.trim()).collect();

            for ex in &exercises {
                let path = ex.path.display().to_string();
                let name = ex.name.clone();

                // 过滤逻辑
                let filter_match = filter_parts
                    .iter()
                    .filter(|f| !f.is_empty())
                    .any(|f| name.contains(f) || path.contains(f))
                    || filter_parts.is_empty();

                // 完成状态过滤
                let is_done = ex.looks_done();
                let status_match = match (subargs.solved, subargs.unsolved) {
                    (true, false) => is_done,
                    (false, true) => !is_done,
                    _ => true,
                };

                if filter_match && status_match {
                    if is_done {
                        done_count += 1;
                    }

                    // 输出格式
                    let line = if subargs.paths {
                        format!("{}", path)
                    } else if subargs.names {
                        format!("{}", name)
                    } else {
                        format!(
                            "{:<20}\t{:<50}\t{}",
                            name,
                            path,
                            if is_done { "✅ 已完成" } else { "⏳ 未完成" }
                        )
                    };
                    println!("{}", line);
                }
            }

            // 输出进度
            let total = exercises.len();
            let progress = (done_count as f32 / total as f32) * 100.0;
            println!("\n📊 进度: {}/{} 练习已完成 ({:.1}%)", done_count, total, progress);
        }

        Subcommands::Run(subargs) => {
            // 运行单个练习
            let ex = find_exercise(&subargs.name, &exercises);
            run::run(ex, verbose).unwrap_or_else(|_| std::process::exit(1));
        }

        Subcommands::Reset(subargs) => {
            // 重置单个练习
            let ex = find_exercise(&subargs.name, &exercises);
            run::reset(ex).unwrap_or_else(|_| std::process::exit(1));
        }

        Subcommands::Hint(subargs) => {
            // 显示练习提示
            let ex = find_exercise(&subargs.name, &exercises);
            println!("💡 {} 的提示: {}", ex.name, ex.hint);
        }

        Subcommands::Verify(_subargs) => {
            // 验证所有练习
            let static_exs: Vec<&'static exercise::Exercise> = exercises
                .iter()
                .map(|e| unsafe { &*(e as *const _) })
                .collect();

            verify::verify(
                static_exs.iter().copied(),
                (0, exercises.len()),
                verbose,
                false,
            )
            .unwrap_or_else(|ex| {
                println_error!("验证失败 - 练习 {} 未通过", ex.name);
                std::process::exit(1);
            });

            println_success!("所有练习验证通过！");
        }

        Subcommands::CicvVerify(_subargs) => {
            // 批量验证并生成报告
            let start_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let success_count = Arc::new(Mutex::new(0));
            let total = exercises.len();

            // 初始化报告
            let report = Arc::new(Mutex::new(ExerciseCheckList {
                exercises: vec![],
                user_name: None,
                statistics: ExerciseStatistics {
                    total_exercises: total,
                    total_succeeds: 0,
                    total_failures: 0,
                    total_time: 0,
                },
            }));

            // 并发验证所有练习
            let mut tasks = vec![];
            for ex in exercises {
                let success_clone = Arc::clone(&success_count);
                let report_clone = Arc::clone(&report);
                let verbose = verbose;

                let task = task::spawn(async move {
                    let ex_start = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    let result = run::run(&ex, verbose).is_ok();
                    if result {
                        *success_clone.lock().unwrap() += 1;
                        println_success!("{} ✅", ex.name);
                    } else {
                        println_error!("{} ❌", ex.name);
                    }

                    let ex_end = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    println!("⏱️ {} 耗时: {}s", ex.name, ex_end - ex_start);

                    // 更新报告
                    let mut report = report_clone.lock().unwrap();
                    report.exercises.push(ExerciseResult {
                        name: ex.name,
                        result,
                    });

                    if result {
                        report.statistics.total_succeeds += 1;
                    } else {
                        report.statistics.total_failures += 1;
                    }
                });

                tasks.push(task);
            }

            // 等待所有任务完成
            for task in tasks {
                task.await.unwrap();
            }

            // 生成最终报告
            let end_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let total_time = (end_time - start_time) as u32;

            let mut report = report.lock().unwrap();
            report.statistics.total_time = total_time;

            // 保存报告
            fs::create_dir_all(".github/result").ok();
            let report_json = serde_json::to_string_pretty(&*report).unwrap();
            fs::write(".github/result/check_result.json", report_json).unwrap();

            // 输出汇总
            println!("\n{}", "=".repeat(80));
            println_success!("批量验证完成！");
            println!("📊 总计: {} 练习", total);
            println!("✅ 成功: {}", report.statistics.total_succeeds);
            println!("❌ 失败: {}", report.statistics.total_failures);
            println!("⏱️ 总耗时: {}s", total_time);
            println!("📄 报告已保存至: .github/result/check_result.json");
            println!("{}", "=".repeat(80));
        }

        Subcommands::Lsp(_subargs) => {
            // 生成 rust-analyzer 配置
            let mut project = project::RustAnalyzerProject::new();
            project.get_sysroot_src().expect("获取 sysroot 失败");
            project.exercises_to_json().expect("解析练习失败");

            if project.crates.is_empty() {
                println_warning!("未找到练习 - 请确认在 rustlings 目录运行");
            } else if project.write_to_disk().is_err() {
                println_error!("生成 rust-project.json 失败");
            } else {
                println_success!("成功生成 rust-project.json");
                println!("💡 重启 rust-analyzer 以加载练习配置");
            }
        }

        Subcommands::Watch(subargs) => {
            // 启动 watch 模式
            match watch(&exercises, verbose, subargs.success_hints) {
                Ok(WatchStatus::Finished) => {
                    println_success!("{}", FINISH_LINE);
                }
                Ok(WatchStatus::Unfinished) => {
                    println!("👋 Watch 模式已退出");
                }
                Err(e) => {
                    println_error!("Watch 模式失败: {:?}", e);
                    println!("💡 可能原因：磁盘空间不足 / inotify 限制达到");
                    std::process::exit(1);
                }
            }
        }
    }
}
