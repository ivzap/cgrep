

use std::fs;

/*
Recursively walks a directory and returns
a list of code files to 
*/
pub fn walk_directory(directory: &str, filters: &Vec<String>) -> Vec<String> {
    // TODO: implement
    // make a mutable vector of strings
    let mut files: Vec<String> = Vec::new();
    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                if let Some(subdir) = entry_path.to_str() {
                    files.extend(walk_directory(subdir, filters));
                }
            } else {
                if let Some(ext) = entry_path.extension().and_then(|e| e.to_str()) {
                    if filters.contains(&ext.to_string()) {
                        if let Some(path_str) = entry_path.to_str() {
                            files.push(path_str.to_string());
                        }
                    }
                }
            }
        }
    }
    return files;
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;

    fn setup_test_dir() -> String {
        let test_dir = "test_dir";
        let _ = fs::remove_dir_all(test_dir); // Clean up before test
        match fs::create_dir(test_dir) {
            Ok(_) => {},
            Err(e) => {
                
            }
        }
        // Create files
        let mut file1 = File::create(format!("{}/file1.rs", test_dir)).unwrap();
        writeln!(file1, "// Rust file").unwrap();
        let mut file2 = File::create(format!("{}/file2.py", test_dir)).unwrap();
        writeln!(file2, "# Python file").unwrap();
        // Create subdirectory
        let sub_dir = format!("{}/sub", test_dir);
        match fs::create_dir(&sub_dir) {
            Ok(_) => {},
            Err(e) => {
            }
        }
        let mut file3 = File::create(format!("{}/file3.rs", sub_dir)).unwrap();
        writeln!(file3, "// Rust file in subdir").unwrap();
        test_dir.to_string()
    }

    #[test]
    fn test_walk_directory_filters_rs() {
        let test_dir = setup_test_dir();
        let filters = vec!["rs".to_string()];
        let files = walk_directory(&test_dir, &filters);
        println!("{:?}", files);
        assert!(files.contains(&"file1.rs".to_string()));
        assert!(files.contains(&"file3.rs".to_string()));
        let _ = fs::remove_dir_all(&test_dir); // Clean up
    }

    #[test]
    fn test_walk_directory_filters_py() {
        let test_dir = setup_test_dir();
        let filters = vec!["py".to_string()];
        let files = walk_directory(&test_dir, &filters);
        println!("{:?}", files);  assert!(files.contains(&"file2.py".to_string()));
        assert!(!files.contains(&"file1.rs".to_string()));
        let _ = fs::remove_dir_all(&test_dir); // Clean up
    }
}
