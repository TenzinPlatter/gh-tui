# shortcut-notes

TUI for browsing current iteration stories from Shortcut and managing one-note-per-story markdown files. Notes have YAML frontmatter with story metadata and are Obsidian-compatible.

## Setup

1. `cargo install --path .`
2. Set `$EDITOR`
3. Get a [Shortcut API token](https://app.shortcut.com/settings/account/api-tokens)
4. Create config at `~/.config/shortcut-notes/config.toml`:

```toml
notes_dir = "~/notes/work"
api_token = "your-token-here"
# cache_dir = "~/.cache/shortcut-notes"  # optional
```

## Usage

```
note              # launch TUI
note open         # open note for active story in $EDITOR
note tmux         # open/attach tmux session for active story
```

### Keys

| Key | Action |
|-----|--------|
| `j/k` | Navigate |
| `Enter` | Open note |
| `Space` | Expand/collapse description |
| `a` | Set active story |
| `t` | Tmux session |
| `1-4` | Switch tabs |
| `q` | Quit |

## Dev

```
cargo run                    # run
DUMMY_DATA=1 cargo run       # run without API calls
cargo test                   # test
cargo clippy                 # lint
```
