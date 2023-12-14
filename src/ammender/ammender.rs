use serde::Deserialize;
use std::fs;
use chrono::{TimeZone, Utc};

#[derive(Deserialize, Debug, Clone)]
struct CommitData {
    sha: String,
    comment: String,
    date: String,
}

struct CommitAmend {
    commit_data: CommitData,
    rebase_commit: String,
    amend_command: String,
}

fn amend_commit(commit_data: &CommitData, new_author: &str, new_email: &str) -> CommitAmend {
    let author_string = format!("{} <{}>", new_author, new_email);
    
    let unix_time = commit_data.date.parse::<i64>().unwrap();
    let datetime = Utc.timestamp_opt(unix_time, 0);
    let commit_date = datetime.unwrap().format("%a %b %e %T %Y %z").to_string();

    let amend_command = format!(
        "exec GIT_COMMITTER_NAME='{}' GIT_COMMITTER_EMAIL='{}' GIT_COMMITTER_DATE='{}' git commit --amend --author '{}' --date '{}'",
        new_author, new_email, commit_date, author_string, commit_date
    );
    let rebase_commit = format!("pick {} {}", commit_data.sha, commit_data.comment);
    
    CommitAmend {
        commit_data: commit_data.clone(),
        rebase_commit,
        amend_command,
    }
}

pub fn sequence_builder(json_file: &str, new_author: &str, new_email: &str) -> Result<(), String> {
    let data = fs::read_to_string(json_file).map_err(|e| e.to_string())?;
    let commits: Vec<CommitData> = serde_json::from_str(&data).map_err(|e| e.to_string())?;

    let mut script_lines = Vec::new();

    for commit in &commits {
        let commit_amend = amend_commit(commit, new_author, new_email);
        script_lines.push(commit_amend.rebase_commit);
        script_lines.push(commit_amend.amend_command);
    }

    let script_content = script_lines.join("\n");
    fs::write("./data/sequence_editor.sh", &script_content).map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use serde_json::json;
    use tempfile::TempDir;
    use std::path::Path;

    #[test]
    fn test_sequence_builder() {
        // Arrange
        // let temp_dir = TempDir::new().expect("Failed to create temporary directory");
        // let json_file = temp_dir.path().join("commit_data.json");
        let new_author = "test_author";
        let new_email = "test_email@example.com";

        // let mock_data = b"[
        //     {
        //       \"sha\": \"0239187061c3cc536d51b9d34c95390c9ab1e8ef\",
        //       \"comment\": \"fourth commit\n\",
        //       \"date\": \"1702508887\"
        //     },
        //     {
        //       \"sha\": \"38111bc1c0547a6debb0b836f4271f5987d35d4c\",
        //       \"comment\": \"third commit\n\",
        //       \"date\": \"1702435549\"
        //     },
        //     {
        //       \"sha\": \"a0e6bc34b3d8b7d6efdf0f359d6a5780af3bf082\",
        //       \"comment\": \"second commit\n\",
        //       \"date\": \"1702435492\"
        //     },
        //     {
        //       \"sha\": \"20aebad0585f3c7ddbf22e599fd16e9691d5a1b4\",
        //       \"comment\": \"init commit\n\",
        //       \"date\": \"1702435437\"
        //     }
        //   ]";

        // let mut file = File::create(&json_file).expect("Failed to create test JSON file");
        // file.write_all(mock_data).expect("Failed to write test JSON file");

        // Act
//        let result = sequence_builder(json_file.to_str().expect("Failed to convert path to str"), new_author, new_email);
        let json_file = "test/data/test_repo_commit_data.json";
        let result = sequence_builder(json_file, new_author, new_email);
        // Assert
        assert!(result.is_ok());

        // Additional assertion to check if the script file is created
        let script_path = Path::new("./data/sequence_editor.sh");
        assert!(script_path.exists(), "sequence_editor.sh script was not created");
    }
}
