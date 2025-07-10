use std::{env, path::PathBuf};

use once_cell::sync::Lazy;

pub static CLI_EXE_DIR: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::new().join(env::current_exe().unwrap()));
pub static CLI_PATH: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::new().join(env::current_exe().unwrap().parent().unwrap().to_path_buf()));
pub static BASE_DIR: Lazy<PathBuf> = Lazy::new(|| PathBuf::new().join(env::current_dir().unwrap()));
pub static TSCONFIG_FILE: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("tsconfig.json"));

pub struct Path {}

impl Path {
    pub fn cli_exe_path() -> String {
        CLI_PATH.clone().display().to_string()
    }
    pub fn cli_exe_dir() -> String {
        CLI_EXE_DIR.clone().display().to_string()
    }
    pub fn current_dir() -> String {
        BASE_DIR.clone().display().to_string()
    }
    pub fn conf_path() -> String {
        TSCONFIG_FILE.clone().display().to_string()
    }
}
