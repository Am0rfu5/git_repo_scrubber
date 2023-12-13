use serde::{Serialize, Deserialize};
use git2::{Repository, Error};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct CommitData {
    sha: String,
    date: String,
}

// Extract commit data from the repository
fn extract_commit_data(repo_path: &str) -> Result<Vec<CommitData>, Error> {
    let repo = Repository::open(repo_path)?;
    let mut commit_data_list = Vec::new();

    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    for oid in revwalk {
        let commit = repo.find_commit(oid?)?;
        let sha = commit.id().to_string();
        let date = commit.time().seconds().to_string();

        commit_data_list.push(CommitData { sha, date });
    }

    Ok(commit_data_list)
}

// Save commit data to a JSON file
fn save_to_json(commit_data: &[CommitData], file_path: &str) -> Result<(), serde_json::Error> {
    let json = serde_json::to_string_pretty(commit_data)?;
    fs::write(file_path, json).expect("Unable to write to file");
    Ok(())
}

// Main function
fn main() {
    let repo_path = "."; // Specify your repo path here
    let file_path = "commit_data.json"; // Output file

    match extract_commit_data(repo_path) {
        Ok(data) => {
            save_to_json(&data, file_path).expect("Error saving JSON");
            println!("Commit data saved to {}", file_path);
        },
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_commit_data() {
        // Setup: path to the test repository
        let test_repo_path = "./test_repo/"; // Update with actual path

        // Act: Call the function with the test repository
        let result = extract_commit_data(test_repo_path);

        // Assert: Check if the result is as expected
        assert!(result.is_ok());
        let commits = result.unwrap();

        // Here, assert the specific expectations you have from the function,
        // such as the number of commits, specific commit SHAs, etc.
        // For example:
        // assert_eq!(commits.len(), expected_number_of_commits);
        // assert_eq!(commits[0].sha, expected_first_commit_sha);
    }
}