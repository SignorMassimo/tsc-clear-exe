use crate::packages::path;

pub fn read_all_files(start_dir: String, dist: String) -> Vec<String> {
    let mut files: Vec<String> = vec![];
    match std::fs::read_dir(start_dir) {
        Ok(entries) => {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    let dir_path = entry.path().to_string_lossy().to_string();
                    if dir_path.ends_with("node_modules") || dir_path.ends_with(&dist) {
                        continue;
                    };
                    files = [
                        &files[..],
                        &read_all_files(
                            path::BASE_DIR.join(entry.path()).display().to_string(),
                            dist.clone(),
                        ),
                    ]
                    .concat();
                } else {
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    if file_name.ends_with(".ts") || file_name.ends_with(".js") {
                        files.push(entry.path().to_string_lossy().to_string());
                    } else {
                        continue;
                    }
                }
            }
        }
        Err(e) => println!("{}", e.to_string()),
    }
    return files;
}

pub fn delete_file_to_dist(start_dir: String, dist: String) {
    let files = read_all_files(start_dir, dist.clone());
    let dist_files = read_all_files(
        path::BASE_DIR.join(&dist).to_string_lossy().to_string(),
        dist.clone(),
    );

    for dist_file in dist_files {
        if dist_file.ends_with(".js")
            || dist_file.ends_with(".d.ts")
            || dist_file.ends_with(".js.map")
        {
            let dist_base = path::BASE_DIR.join(&dist).to_string_lossy().to_string();
            let relative_dist_path = dist_file
                .replace(&format!("{}\\", dist_base), "")
                .replace(&format!("{}/", dist_base), "");

            let mut corresponding_ts_file = String::new();
            if relative_dist_path.ends_with(".d.ts") {
                corresponding_ts_file = relative_dist_path.replace(".d.ts", ".ts");
            } else if relative_dist_path.ends_with(".js") {
                corresponding_ts_file = relative_dist_path.replace(".js", ".ts");
            } else if relative_dist_path.ends_with(".js.map") {
                corresponding_ts_file = relative_dist_path.replace(".js.map", ".ts");
            }

            let ts_file_exists = files.iter().any(|file| {
                let base_dir = path::BASE_DIR.to_string_lossy().to_string();
                let relative_source_path = file
                    .replace(&format!("{}\\", base_dir), "")
                    .replace(&format!("{}/", base_dir), "");
                relative_source_path == corresponding_ts_file
            });

            if !ts_file_exists {
                match std::fs::remove_file(&dist_file) {
                    Ok(_) => {}
                    Err(e) => eprintln!("Failed to delete file {}: {}", dist_file, e),
                }
            }
        }
    }
}
