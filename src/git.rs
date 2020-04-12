use std::process::{Command, Output};

pub fn new_git() -> Command {
    Command::new("git")
}

pub fn git_tag(version: &str) -> std::io::Result<Output> {
    let mut cmd = new_git();
    cmd.arg("tag").arg(version).output()
}

fn git_tag_delete(version: &str) -> std::io::Result<Output> {
    let mut cmd = new_git();
    cmd.arg("tag").arg("-d").arg(version).output()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_tag() {
        let version_str = "test-git-tag";
        git_tag(version_str).unwrap();
        let result = git_tag_delete(version_str).unwrap();
        let output = String::from_utf8_lossy(&result.stdout).to_string();
        assert!(output.contains("Deleted tag 'test-git-tag'"), true);
    }
}