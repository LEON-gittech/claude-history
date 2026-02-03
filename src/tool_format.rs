//! Tool-specific formatting for nicer display of tool calls.
//!
//! Instead of showing raw JSON, this module formats each tool's input
//! in a human-readable way that highlights the most relevant information.

use serde_json::Value;

/// Formatted tool call representation
pub struct FormattedToolCall {
    /// The header line (e.g., "Task (Explore): description" or "$ command")
    pub header: String,
    /// Optional continuation lines (e.g., prompt text, diff lines)
    pub body: Option<String>,
}

/// Format a tool call for display
pub fn format_tool_call(name: &str, input: &Value) -> FormattedToolCall {
    match name {
        "Task" => format_task(input),
        "Bash" => format_bash(input),
        "Read" => format_read(input),
        "Grep" => format_grep(input),
        "Glob" => format_glob(input),
        "Edit" => format_edit(input),
        "Write" => format_write(input),
        "WebFetch" => format_web_fetch(input),
        "WebSearch" => format_web_search(input),
        _ => format_fallback(name, input),
    }
}

fn format_task(input: &Value) -> FormattedToolCall {
    let subagent_type = input
        .get("subagent_type")
        .and_then(|v| v.as_str())
        .unwrap_or("agent");
    let description = input
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let prompt = input.get("prompt").and_then(|v| v.as_str());

    FormattedToolCall {
        header: format!("Task ({}): {}", subagent_type, description),
        body: prompt.map(|p| p.to_string()),
    }
}

fn format_bash(input: &Value) -> FormattedToolCall {
    let command = input.get("command").and_then(|v| v.as_str()).unwrap_or("");

    FormattedToolCall {
        header: format!("Bash: {}", command),
        body: None,
    }
}

fn format_read(input: &Value) -> FormattedToolCall {
    let file_path = input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let offset = input.get("offset").and_then(|v| v.as_u64());
    let limit = input.get("limit").and_then(|v| v.as_u64());

    let header = match (offset, limit) {
        (Some(o), Some(l)) => format!("Read: {}:{}-{}", file_path, o, o + l),
        (Some(o), None) => format!("Read: {}:{}", file_path, o),
        _ => format!("Read: {}", file_path),
    };

    FormattedToolCall { header, body: None }
}

fn format_grep(input: &Value) -> FormattedToolCall {
    let pattern = input.get("pattern").and_then(|v| v.as_str()).unwrap_or("");
    let path = input.get("path").and_then(|v| v.as_str());
    let glob = input.get("glob").and_then(|v| v.as_str());

    let location = match (path, glob) {
        (Some(p), Some(g)) => format!("{}/{}", p, g),
        (Some(p), None) => p.to_string(),
        (None, Some(g)) => g.to_string(),
        (None, None) => ".".to_string(),
    };

    FormattedToolCall {
        header: format!("Grep: \"{}\" in {}", pattern, location),
        body: None,
    }
}

fn format_glob(input: &Value) -> FormattedToolCall {
    let pattern = input.get("pattern").and_then(|v| v.as_str()).unwrap_or("");
    let path = input.get("path").and_then(|v| v.as_str());

    let header = match path {
        Some(p) => format!("Glob: {} in {}", pattern, p),
        None => format!("Glob: {}", pattern),
    };

    FormattedToolCall { header, body: None }
}

fn format_edit(input: &Value) -> FormattedToolCall {
    let file_path = input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let old_string = input.get("old_string").and_then(|v| v.as_str());
    let new_string = input.get("new_string").and_then(|v| v.as_str());

    let body = match (old_string, new_string) {
        (Some(old), Some(new)) => {
            let mut diff = String::new();
            for line in old.lines() {
                diff.push_str(&format!("- {}\n", line));
            }
            for line in new.lines() {
                diff.push_str(&format!("+ {}\n", line));
            }
            // Remove trailing newline
            if diff.ends_with('\n') {
                diff.pop();
            }
            Some(diff)
        }
        _ => None,
    };

    FormattedToolCall {
        header: format!("Edit: {}", file_path),
        body,
    }
}

fn format_write(input: &Value) -> FormattedToolCall {
    let file_path = input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    FormattedToolCall {
        header: format!("Write: {}", file_path),
        body: None,
    }
}

fn format_web_fetch(input: &Value) -> FormattedToolCall {
    let url = input.get("url").and_then(|v| v.as_str()).unwrap_or("");
    let prompt = input.get("prompt").and_then(|v| v.as_str());

    FormattedToolCall {
        header: format!("Fetch: {}", url),
        body: prompt.map(|p| p.to_string()),
    }
}

fn format_web_search(input: &Value) -> FormattedToolCall {
    let query = input.get("query").and_then(|v| v.as_str()).unwrap_or("");

    FormattedToolCall {
        header: format!("Search: \"{}\"", query),
        body: None,
    }
}

fn format_fallback(name: &str, input: &Value) -> FormattedToolCall {
    let body = serde_json::to_string_pretty(input).ok();

    FormattedToolCall {
        header: format!("{}:", name),
        body,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_format_task() {
        let input = json!({
            "subagent_type": "Explore",
            "description": "Find the bug",
            "prompt": "Look for issues in the code"
        });
        let result = format_tool_call("Task", &input);
        assert_eq!(result.header, "Task (Explore): Find the bug");
        assert_eq!(result.body, Some("Look for issues in the code".to_string()));
    }

    #[test]
    fn test_format_bash() {
        let input = json!({
            "command": "git status",
            "description": "Check repo status"
        });
        let result = format_tool_call("Bash", &input);
        assert_eq!(result.header, "Bash: git status");
        assert_eq!(result.body, None);
    }

    #[test]
    fn test_format_read_with_range() {
        let input = json!({
            "file_path": "/src/main.rs",
            "offset": 100,
            "limit": 50
        });
        let result = format_tool_call("Read", &input);
        assert_eq!(result.header, "Read: /src/main.rs:100-150");
    }

    #[test]
    fn test_format_grep() {
        let input = json!({
            "pattern": "fn main",
            "path": "src",
            "glob": "*.rs"
        });
        let result = format_tool_call("Grep", &input);
        assert_eq!(result.header, "Grep: \"fn main\" in src/*.rs");
    }

    #[test]
    fn test_format_edit() {
        let input = json!({
            "file_path": "/src/lib.rs",
            "old_string": "old code",
            "new_string": "new code"
        });
        let result = format_tool_call("Edit", &input);
        assert_eq!(result.header, "Edit: /src/lib.rs");
        assert_eq!(result.body, Some("- old code\n+ new code".to_string()));
    }
}
