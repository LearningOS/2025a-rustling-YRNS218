use glob::glob;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Contains the structure of resulting rust-project.json file
/// and functions to build the data required to create the file
#[derive(Serialize, Deserialize, Debug)]
pub struct RustAnalyzerProject {
    sysroot_src: String,
    pub crates: Vec<Crate>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Crate {
    root_module: String,
    edition: String,
    deps: Vec<String>,
    cfg: Vec<String>,
}

impl RustAnalyzerProject {
    pub fn new() -> Self {
        RustAnalyzerProject {
            sysroot_src: String::new(),
            crates: Vec::new(),
        }
    }

    /// Write rust-project.json to disk
    pub fn write_to_disk(&self) -> Result<(), std::io::Error> {
        let json_data = serde_json::to_vec_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        
        std::fs::write("./rust-project.json", json_data)?;
        println!("Successfully wrote rust-project.json");
        Ok(())
    }

    /// If path contains .rs extension, add a crate to `rust-project.json`
    fn path_to_json(&mut self, path: PathBuf) -> Result<(), Box<dyn Error>> {
        // 仅处理文件（跳过目录）
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "rs" {
                    // 将路径转换为绝对路径（确保rust-analyzer能识别）
                    let abs_path = path.canonicalize()?;
                    self.crates.push(Crate {
                        root_module: abs_path.display().to_string(),
                        edition: "2021".to_string(),
                        deps: Vec::new(),
                        // 支持测试模块解析
                        cfg: vec!["test".to_string()],
                    });
                }
            }
        }
        Ok(())
    }

    /// Parse the exercises folder for .rs files, any matches will create
    /// a new `crate` in rust-project.json which allows rust-analyzer to
    /// treat it like a normal binary
    pub fn exercises_to_json(&mut self) -> Result<(), Box<dyn Error>> {
        let pattern = if cfg!(windows) {
            "./exercises/**/*.rs"
        } else {
            "./exercises/**/*"
        };

        for entry in glob(pattern)? {
            match entry {
                Ok(path) => self.path_to_json(path)?,
                Err(e) => eprintln!("Warning: Failed to read path: {}", e),
            }
        }
        
        if self.crates.is_empty() {
            eprintln!("Warning: No .rs files found in ./exercises");
        } else {
            println!("Found {} Rust files", self.crates.len());
        }
        Ok(())
    }

    /// Use `rustc` to determine the default toolchain
    pub fn get_sysroot_src(&mut self) -> Result<(), Box<dyn Error>> {
        // 优先使用环境变量RUST_SRC_PATH
        if let Ok(path) = env::var("RUST_SRC_PATH") {
            if Path::new(&path).exists() {
                self.sysroot_src = path;
                println!("Using RUST_SRC_PATH: {}", self.sysroot_src);
                return Ok(());
            } else {
                eprintln!("Warning: RUST_SRC_PATH is set but path does not exist");
            }
        }

        // 通过rustc获取sysroot
        let output = Command::new("rustc")
            .arg("--print")
            .arg("sysroot")
            .output()?;

        if !output.status.success() {
            return Err("Failed to execute `rustc --print sysroot`".into());
        }

        let toolchain = String::from_utf8(output.stdout)?
            .trim()
            .to_string();

        println!("Determined toolchain path: {}", toolchain);

        // 构建标准库源码路径
        let sysroot_src = Path::new(&toolchain)
            .join("lib")
            .join("rustlib")
            .join("src")
            .join("rust")
            .join("library");

        if !sysroot_src.exists() {
            return Err(format!(
                "Standard library sources not found at: {}",
                sysroot_src.display()
            )
            .into());
        }

        self.sysroot_src = sysroot_src.display().to_string();
        println!("Found sysroot src: {}", self.sysroot_src);
        Ok(())
    }
}

// 主函数示例（用于测试）
fn main() -> Result<(), Box<dyn Error>> {
    let mut project = RustAnalyzerProject::new();
    
    // 获取sysroot路径
    project.get_sysroot_src()?;
    
    // 扫描练习文件
    project.exercises_to_json()?;
    
    // 生成rust-project.json
    project.write_to_disk()?;
    
    Ok(())
}
