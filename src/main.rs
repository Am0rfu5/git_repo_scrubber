mod extractor;
mod sequencer;

use extractor::extractor::{extract_commit_data, save_to_json};
use sequencer::sequencer::sequence_builder;
use std::path::Path;
use std::fs;
use std::process::Command;
use clap::Parser;
use std::env;
use log::SetLoggerError;
use log::{info, warn};
use simplelog::*;
use std::fs::File;

fn setup_logging() -> Result<(), SetLoggerError> {
    WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create("my_app.log").unwrap(),
    )
}

/// This struct defines the command-line interface of your application
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // The current application name
    /// The path to the repository
    #[arg(value_parser = validate_git_repo_path)]
    repo_path: String,

    /// The new author's name (optional)
    #[clap(short, long, value_parser)]
    author: Option<String>,

    /// The new author's email (optional)
    #[clap(short, long, value_parser)]
    email: Option<String>,

    /// Extract commit data
    #[clap(long, action)]
    extract: bool,

    /// Amend commit data
    #[clap(long, action)]
    amend: bool,

    /// Extract and then amend commit data
    #[clap(long, action)]
    extract_amend: bool,
    
    /// Path to the git-rebase-todo file for sequence editor mode
    #[clap(value_parser, required = false)]
    todo_file_path: Option<String>,
}

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

    let program_name = env::args()
        .next()
        .unwrap_or_else(|| "<unknown>"
        .to_string());

    // Configure Git to use the sequence_editor script
    Command::new("git")
        .args(["-C", repo_path, "config", "sequence.editor", &program_name])
        .status()
        .map_err(|e| e.to_string())?;

    // Start the interactive rebase
    Command::new("git")
    .args(["-C", repo_path, "rebase", "-i", "--root"])
    .status()
    .map_err(|e| e.to_string())?;

    Ok(())
}

fn handle_sequence_editor(todo_file_path: &str) -> Result<(), String> {
    // Read the git-rebase-todo file
    // let mut content = fs::read_to_string(todo_file_path)
    //     .map_err(|e| e.to_string())?;
    
    // Read the content from the sequence_editor file
    // TODO move the sequence_editor file to a struct?
    let sequence_editor_content = fs::read_to_string("data/rebase_todo.txt")
        .map_err(|e| e.to_string())?;

    info!("handling sequence editor");
    // Modify the content
    let content = sequence_editor_content;
        println!("Here");
    // Write the modified content back to the file
    fs::write(todo_file_path, &content)
        .map_err(|e| e.to_string())?;

    Ok(())
}

// Check if the repository path ends with ".git"
fn validate_git_repo_path(val: &str) -> Result<(), String> {
    if Path::new(val).extension().and_then(std::ffi::OsStr::to_str) == Some("git") {
        Ok(())
    } else {
        Err(String::from("The repository path must end with '.git'"))
    }
}

fn main() {
    setup_logging().expect("Failed to initialize logging");
    info!("Starting the application");
    let cli = Cli::parse();

    // Check if the application is run with git-rebase-todo file
    if let Some(todo_file_path) = cli.todo_file_path {
        if todo_file_path.ends_with("git-rebase-todo") {
            handle_sequence_editor(&todo_file_path)
                .expect("Failed to handle git-rebase-todo file");
            return;
        }
    }
    
    let file_path = if let Some(repo_name) = Path::new(&cli.repo_path).file_name().and_then(|n| n.to_str()) {
        let dir_path = ".commit_data";
        fs::create_dir_all(dir_path).expect("Failed to create directory");
        format!("{}/{}_commit_data.json", dir_path, repo_name)
    } else {
        eprintln!("Invalid repository path");
        std::process::exit(1);
    };

    if cli.extract {
        run_extractor(&cli.repo_path, &file_path).expect("Error running extractor");
    }

    if cli.amend {
        let new_author = cli.author.as_deref().unwrap_or("");
        let new_email = cli.email.as_deref().unwrap_or("");
        run_rebaser(&cli.repo_path, &file_path, &new_author, &new_email)
            .expect("Error running rebaser");
    }

    // if cli.extract_amend {
    //     run_extractor(&cli.repo_path, &file_path).expect("Error running extractor");
    //     run_rebaser(&cli.repo_path, &file_path, cli.author.as_deref().unwrap_or(""), cli.email.as_deref().unwrap_or("")).expect("Error running rebaser");
    // }
}

#[cfg(test)]
pub mod test_util {
    pub fn get_test_repo_path() -> String {
        dotenv::dotenv().ok();
        dotenv::var("TEST_REPO").expect("TEST_REPO not set")
    }
    
    pub fn get_test_rebase_todo() -> String {
        dotenv::dotenv().ok();
        dotenv::var("TEST_REBASE_TODO").expect("TEST_REBASE_TODO not set")
    }
    
    pub fn get_test_extracted_data() -> String {
        dotenv::dotenv().ok();
        dotenv::var("TEST_EXTRACTED_DATA").expect("TEST_EXTRACTED_DATA not set")
    }
    
}

#[cfg(test)]
mod tests {
    use super::run_rebaser;
    use super::run_extractor;
    use crate::test_util::*;
    use log::SetLoggerError;
    use log::{info, warn};
    use simplelog::*;
    use std::fs::File;
    
    fn setup_logging() -> Result<(), SetLoggerError> {
        WriteLogger::init(
            LevelFilter::Info,
            Config::default(),
            File::create("my_app.log").unwrap(),
        )
    }
    
    #[test]
    fn test_run_extract() {
        
        // TODO use test file location and repo_path
        let test_extracted_data= get_test_extracted_data();
        let test_extracted_data = &*test_extracted_data;        
        let test_repo_path = get_test_repo_path();
        let test_repo_path = &*test_repo_path;

        // Act
        let result = run_extractor(test_repo_path, test_extracted_data);

        // Assert
        assert!(result.is_ok());
    }   

    #[test]
    fn test_run_rebase() {
        
        setup_logging().expect("Failed to initialize logging");
        info!("test_run_rebase");
        
        let test_extracted_data = get_test_extracted_data();
        let test_extracted_data = &*test_extracted_data;        
        let test_repo_path = get_test_repo_path();
        let test_repo_path = &*test_repo_path;

        let new_author = "New Author";
        let new_email = "new_email@example.com";
        // Act
        let result = run_rebaser(test_repo_path, test_extracted_data, new_author, new_email);

        // Assert
        assert!(result.is_ok());
    }
}