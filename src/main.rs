mod extractor;
mod ammender;

use extractor::extractor::{extract_commit_data, save_to_json};
use ammender::ammender::amend_commits;
use std::env;
use std::path::Path;
use std::fs;

fn run_extractor(repo_path: &str, file_path: &str) -> Result<(), String> {
    // Extract and save commit data
    match extract_commit_data(repo_path) {
        Ok(data) => {
            save_to_json(&data, file_path).map_err(|e| e.to_string())?;
        }
        Err(e) => return Err(e.to_string()),
    }
    Ok(())
}

fn run_ammender(repo_path: &str, file_path: &str, new_author: &str, new_email: &str) -> Result<(), String> {
    // Amend commits
    amend_commits(file_path, new_author, new_email)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} [--extract | -e | --amend | -a | --extract-amend | -ea] <path-to-repo> [options]", args[0]);
        std::process::exit(1);
    }

    let option = &args[1];
    let repo_path = &args[2];
    let mut file_path = String::new();
    let mut new_author = String::new();
    let mut new_email = String::new();

    // Determine file path
    if args.len() > 3 {
        file_path = args[3].clone();
    } else {
        if let Some(repo_name) = Path::new(repo_path).file_name().and_then(|n| n.to_str()) {
            let dir_path = ".commit_data";
            fs::create_dir_all(dir_path).expect("Failed to create directory");
            file_path = format!("{}/{}_commit_data.json", dir_path, repo_name);
        } else {
            eprintln!("Invalid repository path");
            std::process::exit(1);
        }
    }

    // Parse additional arguments
    for arg in args.iter().skip(4) {
        if arg.starts_with("--author=") {
            new_author = arg[9..].to_string();
        } else if arg.starts_with("--email=") {
            new_email = arg[8..].to_string();
        }
    }

    match option.as_str() {
        "--extract" | "-e" => {
            if let Err(e) = run_extractor(repo_path, &file_path) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
        "--amend" | "-a" => {
            if let Err(e) = run_ammender(repo_path, &file_path, &new_author, &new_email) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
        "--extract-amend" | "-ea" => {
            if let Err(e) = run_extractor(repo_path, &file_path) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
            if let Err(e) = run_ammender(repo_path, &file_path, &new_author, &new_email) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
        _ => {
            eprintln!("Invalid option: {}", option);
            std::process::exit(1);
        }
    }
}


// #[cfg(test)]
// mod tests {
//     use super::run;
//     use tempfile::NamedTempFile;
    
//     #[test]
//     fn test_run_extract() {
//         // Arrange
//         let test_repo_path = "../test_repo"; // Update with actual test repo path
//         let temp_file = NamedTempFile::new().expect("Failed to create temporary file");
//         let temp_file_path = temp_file.path().to_str().unwrap();
//         let new_author = "New Author";
//         let new_email = "new_email@example.com";

//         // Act
//         let result = run(test_repo_path, temp_file_path, new_author, new_email);

//         // Assert
//         assert!(result.is_ok());
//     }   
// }
