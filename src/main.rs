#![allow(dead_code, unused_must_use, unused_assignments)]

use regex::Regex;

use self::packages::path;
mod packages;

fn main() {
    if path::TSCONFIG_FILE.exists() {
        let tsconfig = std::fs::read_to_string(path::TSCONFIG_FILE.display().to_string()).unwrap();
        let mut dist = Regex::new(r#""outDir"\s*:\s*"([^"]*)""#)
            .unwrap()
            .captures(&tsconfig)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();
        if dist.starts_with("./") {
            dist = dist[2..].to_string()
        }
        if path::BASE_DIR.join(&dist).exists() {
            packages::rt_file_control::delete_file_to_dist(
                packages::path::BASE_DIR.display().to_string(),
                dist,
            );
        } else {
            println!("'{}' file not found this file directory.", &dist);
        }
    } else {
        println!("'tsconfg.json' file not found this file directory.");
    }
}
