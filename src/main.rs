mod extractor;
mod ammender;

use extractor::extractor::{extract_commit_data, save_to_json};
use ammender::ammender::sequence_builder;
use std::env;
use std::path::Path;
use std::fs;
use std::process::Command;

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

fn run_rebaser(repo_path: &str, file_path: &str, new_author: &str, new_email: &str) -> Result<(), String> {
    // Build the sequence editor script
    sequence_builder(file_path, new_author, new_email)?;

    let git_dir = format!("--git-dir=\"{}\"", repo_path);
    
    // Configure Git to use the sequence_editor script
    Command::new("git")
        .args([&git_dir, "config", "sequence.editor", "sh sequence_editor.sh"])
        .status()
        .map_err(|e| e.to_string())?;

    // Start the interactive rebase
    let git_dir = format!("--git-dir=\"{}\"", repo_path);
    Command::new("git")
        .args([&git_dir, "rebase", "-i", "--root"]) // adjust as needed
        .status()
        .map_err(|e| e.to_string())?;

    Ok(())
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
            if let Err(e) = run_rebaser(&repo_path, &file_path, &new_author, &new_email) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
        "--extract-amend" | "-ea" => {
            if let Err(e) = run_extractor(repo_path, &file_path) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
            if let Err(e) = run_rebaser(&repo_path, &file_path, &new_author, &new_email) {
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

#[cfg(test)]
mod tests {
    use super::run_rebaser;
    use super::run_extractor;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_run_extract() {
        // Arrange
        let test_repo_path = "../test_repo"; // Update with actual test repo path
        // let temp_file = NamedTempFile::new().expect("Failed to create temporary file");
        let test_file_path = "test/data/test_repo_commit_data.json";
        // let temp_file_path = temp_file.path().to_str().unwrap();
        let new_author = "New Author";
        let new_email = "new_email@example.com";

        // Act
        let result = run_extractor(test_repo_path, test_file_path);

        // Assert
        assert!(result.is_ok());
    }   

    #[test]
    fn test_run_rebase() {
        // Arrange
        let test_repo_path = "../test_repo"; // Update with actual test repo path
        let test_file_path = "test/data/test_repo_commit_data.json";
        let new_author = "New Author";
        let new_email = "new_email@example.com";
        
        // Act
        let result = run_rebaser(test_repo_path, test_file_path, new_author, new_email);
        
        // Assert
        assert!(result.is_ok());
    }
}