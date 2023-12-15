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

fn update_commit_data(commit_data: &CommitData, new_author: &str, new_email: &str) -> CommitData {
    let author_string = format!("{} <{}>", new_author, new_email);
    
    let unix_time = commit_data.date.parse::<i64>().unwrap();
    let datetime = Utc.timestamp_opt(unix_time, 0);
    let commit_date = datetime.unwrap().format("%a %b %e %T %Y %z").to_string();

    CommitData {
        sha: commit_data.sha.clone(),
        comment: commit_data.comment.clone(),
        date: commit_date,
    }
}

// fn set_commit_data(json_file: &str) {
//     let data = fs::read_to_string(json_file).map_err(|e| e.to_string())?;
//     let commits: Vec<CommitData> = serde_json::from_str(&data).map_err(|e| e.to_string())?;
// }

// fn get_commit_data(json_file: &str) -> Vec<CommitData> {
//     let data = fs::read_to_string(json_file).map_err(|e| e.to_string())?;
//     let commits: Vec<CommitData> = serde_json::from_str(&data).map_err(|e| e.to_string())?;
// }

pub fn sequence_builder(extracted_data: &str, new_author: &str, new_email: &str) -> Result<(), String> {
    let data = fs::read_to_string(extracted_data).map_err(|e| e.to_string())?;
    let commits: Vec<CommitData> = serde_json::from_str(&data).map_err(|e| e.to_string())?;

    let mut script_lines = Vec::new();

    for commit in &commits {
        let commit_amend = amend_commit(commit, new_author, new_email);
        script_lines.push(commit_amend.rebase_commit);
        script_lines.push(commit_amend.amend_command);
    }

    let script_content = script_lines.join("\n");
    fs::write("./data/rebase_todo.txt", &script_content).map_err(|e| e.to_string())?;

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
    use crate::test_util::get_test_rebase_todo;

    #[test]
    fn test_sequence_builder() {
        // Arrange
        let new_author = "test_author";
        let new_email = "test_email@example.com";

        // Act
        let extracted_data = get_test_rebase_todo();
        let extracted_data = &*extracted_data;
        let result = sequence_builder(extracted_data, new_author, new_email);
        
        // Assert
        assert!(result.is_ok());

        // Additional assertion to check if the script file is created
        // let script_path = Path::new("./data/sequence_editor.sh");
        // assert!(script_path.exists(), "sequence_editor.sh script was not created");
    }
}
