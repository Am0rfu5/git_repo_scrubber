use serde::{Serialize, Deserialize};
use git2::{Repository, Error};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitData {
    sha: String,
    comment: String,
    date: String,
}

// Extract commit data from the repository
pub fn extract_commit_data(repo_path: &str) -> Result<Vec<CommitData>, Error> {
    let repo = Repository::open(repo_path)?;
    let mut commit_data_list = Vec::new();

    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    for oid in revwalk {
        let commit = repo.find_commit(oid?)?;
        let sha = commit.id().to_string();
        let comment = commit.message()
            .unwrap_or("")
            .trim_end()
            .to_string();
        let date = commit.time().seconds().to_string();

        commit_data_list.push(CommitData { sha, comment, date });
    }

    Ok(commit_data_list)
}

// Save commit data to a JSON file
pub fn save_to_json(commit_data: &[CommitData], file_path: &str) -> Result<(), serde_json::Error> {
    let json = serde_json::to_string_pretty(commit_data)?;
    fs::write(file_path, json).expect("Unable to write to file");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Read;

    #[test]
    fn test_extract_commit_data() {
        // Setup: path to the test repository
        let test_repo_path = "../test_repo/"; // Update with actual path

        // Act: Call the function with the test repository
        let result = extract_commit_data(test_repo_path);

        // Assert: Check if the result is as expected
        assert!(result.is_ok());
        let commits = result.unwrap();

        assert_eq!(commits.len(), 4);
        assert_eq!(commits[0].sha, "0239187061c3cc536d51b9d34c95390c9ab1e8ef");
        // save_to_json(&commits, "data/commit_data.json").expect("Error saving JSON");
    }

    #[test]
    fn test_save_to_json() {
        // Arrange: Create mock data and a temporary file
        let mock_data = vec![
        CommitData { sha: "5f2d4b7468be1b13c9919e29c0ebe24aa6c88945".into(), comment: "Fourth commit".into(), date: "1702435688".into() },
        CommitData { sha: "38111bc1c0547a6debb0b836f4271f5987d35d4c".into(), comment: "Third commit".into(), date: "1702435549".into() },
        CommitData { sha: "a0e6bc34b3d8b7d6efdf0f359d6a5780af3bf082".into(), comment: "Second commit".into(), date: "1702435492".into() },
        CommitData { sha: "20aebad0585f3c7ddbf22e599fd16e9691d5a1b4".into(), comment: "Initial commit".into(), date: "1702435437".into() },
        ];

        let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");

        // Act: Call the function with the mock data and temporary file path
        save_to_json(&mock_data, temp_file.path().to_str().unwrap())
            .expect("Failed to save to JSON");

        // Assert: Read back the file and compare with expected JSON
        let mut contents = String::new();
        temp_file.as_file_mut().read_to_string(&mut contents)
            .expect("Failed to read temporary file");

        let expected_json = serde_json::to_string_pretty(&mock_data).expect("Failed to serialize data");
        assert_eq!(contents, expected_json);
    }
    
}