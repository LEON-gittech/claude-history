# claude-history

`claude-history` is a companion CLI for Claude Code. It lets you search recent
conversations recorded in Claude's local project history with an
`fzf`-powered fuzzy finder, then prints the selected transcript in a tidy,
readable format.

Run it from the project directory you work on with Claude Code and it will
discover the matching transcript folder automatically.

## requirements

- Rust toolchain from [rustup.rs](https://rustup.rs/) for building
- [`fzf`](https://github.com/junegunn/fzf) available on your `PATH`
- Claude Code conversation logs under `~/.claude/projects`

## install

Clone the repository and install the binary locally:

```sh
$ cargo install --path .
```

## usage

Run the tool from inside the project directory you're interested in:

```sh
$ claude-history
```

This opens an `fzf` session listing all conversations, newest first. The search
matches against the entire transcript; the preview shows the first three
messages by default.

```
View Claude conversation history with fuzzy search

Usage: claude-history [OPTIONS]

Options:
  -t, --no-tools       Hide tool calls from the conversation output
  -d, --show-dir       Print the conversation directory path and exit
  -l, --last           Show the last messages in the fuzzy finder preview
  -r, --relative-time  Display relative time instead of absolute timestamp
  -c, --resume         Resume the selected conversation in Claude Code
  -h, --help           Print help
```

### preview modes

- `claude-history` shows the first three messages in the preview
- `claude-history --last` flips the preview to the last three messages.

### hiding tool calls

Tool invocations (`<Calling Tool: …>`) can clutter the output when you're only
interested in the human conversation. Use `--no-tools` to suppress those lines.

### resuming conversations

If you want to continue a conversation, launch `claude-history` with `--resume`
and it will hand off to `claude --resume <conversation-id>`.

### integration with other scripts

You can integrate `claude-history` into other tools to pass conversation context
to new Claude Code sessions. This is useful when you want Claude to understand
what you were working on previously.

For example, a commit message generator script could use the conversation history
to write more contextual commit messages:

```bash
# Get conversation history if --context flag is set
conversation_context=""
if [ "$include_history" = true ]; then
    echo "Loading conversation history..."
    conversation_history=$(claude-history --no-tools 2>/dev/null)
    if [ -n "$conversation_history" ]; then
        conversation_context="

=== START CONVERSATION CONTEXT ===
$conversation_history
=== END CONVERSATION CONTEXT ===

"
    fi
fi

# Pass to Claude CLI with the conversation context
prompt="Write a commit message for these changes.
${conversation_context}
Staged changes:
$staged_diff"

claude -p "$prompt"
```

The `--no-tools` flag is particularly useful here since it filters out tool
invocations, giving you clean conversation text that's easier for Claude to
process as context.

## filtering details

The tool filters out some noisy artifacts before showing conversations, so you
only see transcripts that are likely to matter for your recent work.

- Skips the "Warmup / I'm Claude Code…" exchanges that are sometimes injected
  without user interaction
- Skips conversations that only contain the `/clear` terminal command

## development

The repository includes `just` recipes:

```sh
$ just check
```

This runs `cargo fmt`, `cargo clippy --fix`, and `cargo build` in parallel.
