use serde::Deserialize;
use std::process::Command;
use std::fs;

#[derive(Deserialize)]
struct CommitData {
    sha: String,
    date: String
}

pub fn amend_commits(repo_path: &str, json_file: &str, new_author: &str, new_email: &str) -> Result<(), String> {
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


use std::fs;
use std::process::Command;

fn main() {
    // ... your existing code ...

    // Step 1: Parse the commit_data.json
    let commit_data = parse_commit_data("commit_data.json").expect("Error parsing commit data");

    // Step 2: Write the sequence.editor script
    let script_content = create_sequence_editor_script(&commit_data);
    fs::write("sequence_editor.sh", &script_content).expect("Error writing script");

    // Step 3: Configure Git to use the script
    Command::new("git")
        .args(["config", "sequence.editor", "sh sequence_editor.sh"])
        .status()
        .expect("Failed to set sequence.editor");

    // Step 4: Start the interactive rebase
    Command::new("git")
        .args(["rebase", "-i", "--root"]) // adjust as needed
        .status()
        .expect("Rebase failed");

    // Cleanup: Remove or reset the sequence.editor configuration if necessary
}
