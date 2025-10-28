// TEMPORARY: Warmup message filtering
// This module contains logic to filter out "warmup" messages that are sometimes
// injected into Claude Code conversations. These messages consist of:
// 1. A user message containing only "Warmup"
// 2. An assistant response starting with "I'm Claude Code, Anthropic's CLI for Claude..."
//
// This filtering can be removed once warmup messages are no longer injected.

use crate::claude::{ContentBlock, LogEntry, UserContent};

/// Enable/disable warmup filtering. Set to false to disable all filtering.
pub const ENABLE_WARMUP_FILTERING: bool = true;

/// Check if a log entry is a "Warmup" user message
pub fn is_warmup_user(entry: &LogEntry) -> bool {
    if !ENABLE_WARMUP_FILTERING {
        return false;
    }

    if let LogEntry::User { message, .. } = entry {
        let text = match &message.content {
            UserContent::String(s) => s.trim(),
            UserContent::Blocks(blocks) => {
                return blocks.iter().any(|block| {
                    if let ContentBlock::Text { text } = block {
                        text.trim() == "Warmup"
                    } else {
                        false
                    }
                });
            }
        };
        text == "Warmup"
    } else {
        false
    }
}

/// Check if a log entry is a warmup assistant message
pub fn is_warmup_assistant(entry: &LogEntry) -> bool {
    if !ENABLE_WARMUP_FILTERING {
        return false;
    }

    if let LogEntry::Assistant { message, .. } = entry {
        for block in &message.content {
            if let ContentBlock::Text { text } = block {
                let trimmed = text.trim();
                if trimmed.starts_with("I'm Claude Code, Anthropic's CLI for Claude. I'm ready to help you navigate and explore this codebase.") {
                    return true;
                }
            }
        }
    }
    false
}
