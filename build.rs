extern crate gcc;

use std::string::String;
use std::env;
use std::path::{Path, PathBuf};
use std::fs;


fn main() {
    let cpu_features = env::vars().filter(|kv| -> bool {
        let &(ref key, _) = kv;
        key.starts_with("CARGO_FEATURE_CPU_")
    }).map(|kv| -> String {
        let (ref key, _) = kv;
        key.chars().skip(18).collect()
    });
    // Assume cpu features are correctly set and we won't get
    // collisions between functions defined in separate .s or .S files.
    let mut asm_files: Vec<String> = Vec::new();
    for cpu_name in cpu_features {
        let mut cpu_dir = PathBuf::from("src");
        cpu_dir.push("cpu");
        cpu_dir.push(cpu_name.to_lowercase());
        println!("read_dir {}", cpu_dir.as_path().to_str().unwrap());
        asm_files.extend(list_asm_files(cpu_dir.as_path()));
    }

    let mut gcc_cfg = gcc::Config::new();
    gcc_cfg.compiler("arm-none-eabi-gcc");

    for f in asm_files {
        gcc_cfg.file(f);
    }
    gcc_cfg.compile("libcpu.a");
}

fn list_asm_files(dir: &Path) -> Vec<String> {
    fs::read_dir(dir).unwrap()
        .filter(|res_de| res_de.is_ok())
        .map(|res_de| res_de.unwrap().path())
        .filter(|f| f.extension().map_or(false, |v| v == "s" || v == "S"))
        .map(|f| f.to_str().unwrap().to_string())
        .collect()
}
