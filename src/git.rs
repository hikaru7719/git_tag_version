use super::config::Config;
use std::process::{Command, Output};
pub fn new_git() -> Command {
    Command::new("git")
}

#[derive(Debug)]
pub enum CommandError {
    Io(std::io::Error),
    Read(std::string::FromUtf8Error),
}

pub fn git_tag() -> std::io::Result<Output> {
    let mut cmd = new_git();
    cmd.arg("tag").output()
}

pub fn git_tag_version(version: &String) -> std::io::Result<Output> {
    let mut cmd = new_git();
    cmd.arg("tag").arg(version).output()
}

pub fn git_tag_delete(version: &String) -> std::io::Result<Output> {
    let mut cmd = new_git();
    cmd.arg("tag").arg("-d").arg(version).output()
}

pub fn git_fetch() -> std::io::Result<Output> {
    let mut cmd = new_git();
    cmd.arg("fetch").output()
}

pub fn git_push(version: &String) -> std::io::Result<Output> {
    let mut cmd = new_git();
    cmd.arg("push").arg("origin").arg(version).output()
}

pub fn run_command(f: fn() -> std::io::Result<Output>) -> Result<String, CommandError> {
    f().map_err(CommandError::Io)
        .and_then(|output| String::from_utf8(output.stdout).map_err(CommandError::Read))
}

pub fn run_command_with_arg(
    arg: &String,
    f: fn(&String) -> std::io::Result<Output>,
) -> Result<String, CommandError> {
    f(arg)
        .map_err(CommandError::Io)
        .and_then(|output| String::from_utf8(output.stdout).map_err(CommandError::Read))
}

pub fn version_list(config: &Config) -> Result<Vec<String>, CommandError> {
    if config.fetch == true {
        run_command(git_fetch)?;
    }
    let stdout_string = run_command(git_tag)?;
    let mut iter = stdout_string.trim_end().split_ascii_whitespace();
    let mut vec = Vec::<String>::new();
    loop {
        match iter.next() {
            Some(x) => {
                vec.push(x.to_string());
            }
            None => break,
        }
    }
    Ok(vec)
}

pub fn tag_and_push(version: String, config: &Config) -> Result<i32, CommandError> {
    run_command_with_arg(&version, git_tag_version)?;
    if config.auto_push == true {
        run_command_with_arg(&version, git_push)?;
    }
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_tag() {
        let result = git_tag().unwrap();
        let string = String::from_utf8(result.stdout).unwrap();
        println!("{}", string);
        assert_eq!(result.status.code().unwrap(), 0);
    }

    #[test]
    fn test_git_tag_version() {
        let version = "test-git-tag";
        git_tag_version(&version.to_string()).unwrap();
        let result = git_tag_delete(&version.to_string()).unwrap();
        let output = String::from_utf8_lossy(&result.stdout).to_string();
        assert!(output.contains("Deleted tag 'test-git-tag'"), true);
    }

    #[test]
    fn test_git_fetch() {
        let result = git_fetch().unwrap();
        assert_eq!(result.status.code().unwrap(), 0);
    }
}
