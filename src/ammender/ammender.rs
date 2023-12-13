use serde::Deserialize;
use std::process::Command;
use std::fs;

#[derive(Deserialize)]
struct CommitData {
    sha: String,
    date: String
}

pub fn amend_commits(json_file: &str, new_author: &str, new_email: &str) -> Result<(), String> {
    let data = fs::read_to_string(json_file).map_err(|e| e.to_string())?;
    let commits: Vec<CommitData> = serde_json::from_str(&data).map_err(|e| e.to_string())?;

    for commit in commits {
        let author_string = format!("{} <{}>", new_author, new_email);
        let commit_date = &commit.date;

        // Execute git command to amend the commit
        Command::new("git")
            .args(["checkout", &commit.sha])
            .status()
            .map_err(|e| e.to_string())?;

        let command_str = format!(
            "GIT_COMMITTER_NAME='{}' GIT_COMMITTER_EMAIL='{}' GIT_COMMITTER_DATE='{}' git commit --amend --author='{}' --date='{}'",
            new_author, new_email, commit_date, author_string, commit_date
        );

        Command::new("sh")
            .arg("-c")
            .arg(&command_str)
            .status()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}
