//! Shell command parsing and execution

use anyhow::Result;
use regex::Regex;
use std::process::Command;
use std::sync::LazyLock;

/// A command found in prompt content
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellCommand {
    /// The full match including braces: {{command}}
    pub full_match: String,
    /// The command to execute
    pub command: String,
    /// Start position in the content
    pub start: usize,
    /// End position in the content
    pub end: usize,
}

// Regex for matching {{command}} patterns
static COMMAND_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\{\{([^}]+)\}\}").expect("Invalid command regex")
});

/// Find all commands in content
pub fn find_commands(content: &str) -> Vec<ShellCommand> {
    COMMAND_REGEX
        .captures_iter(content)
        .map(|cap| {
            let full_match = cap.get(0).unwrap();
            let cmd_match = cap.get(1).unwrap();
            
            ShellCommand {
                full_match: full_match.as_str().to_string(),
                command: cmd_match.as_str().trim().to_string(),
                start: full_match.start(),
                end: full_match.end(),
            }
        })
        .collect()
}

/// Check if content contains any commands
pub fn has_commands(content: &str) -> bool {
    COMMAND_REGEX.is_match(content)
}

/// Execute a shell command and return its output
pub fn execute_command(command: &str) -> Result<String> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", command])
            .output()?
    } else {
        Command::new("sh")
            .args(["-c", command])
            .output()?
    };

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow::anyhow!("Command failed: {}", stderr))
    }
}

/// Execute a command and format the result (including error handling)
pub fn execute_command_safe(command: &str) -> String {
    match execute_command(command) {
        Ok(output) => output.trim_end().to_string(),
        Err(e) => format!("<!-- Command failed: {} -->", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_commands() {
        let content = "Files: {{ls -la}} and date: {{date}}";
        let cmds = find_commands(content);
        
        assert_eq!(cmds.len(), 2);
        assert_eq!(cmds[0].command, "ls -la");
        assert_eq!(cmds[0].full_match, "{{ls -la}}");
        assert_eq!(cmds[1].command, "date");
    }

    #[test]
    fn test_no_commands() {
        let content = "No commands here!";
        let cmds = find_commands(content);
        assert!(cmds.is_empty());
    }

    #[test]
    fn test_has_commands() {
        assert!(has_commands("Contains {{command}}"));
        assert!(!has_commands("No commands here"));
    }

    #[test]
    fn test_execute_simple_command() {
        let result = execute_command("echo hello");
        assert!(result.is_ok());
        assert!(result.unwrap().trim() == "hello");
    }

    #[test]
    fn test_execute_invalid_command() {
        let result = execute_command("nonexistent_command_12345");
        assert!(result.is_err());
    }
}
