# cairndb

A SQL database written from scratch in Rust, built as a learning project.

## Status

**Phase 1 is complete:** a persistent key-value store with an interactive shell. Every change is appended to a log file (`store.db`), and the log is replayed on startup, so data survives restarts.

## Getting started

You need a recent stable Rust toolchain (edition 2024). Install it with [rustup](https://rustup.rs).

```bash
git clone git@github.com:niksan004/cairndb.git
cd cairndb
cargo run
```

This starts the shell. The data is stored in `store.db` in the current directory, which is created on first run.

### Commands

| Command | Description |
|---|---|
| `set <key> <value>` | Store a value under a key. Values may contain spaces. |
| `get <key>` | Print the value for a key, or `null` if it doesn't exist. |
| `del <key>` | Delete a key. |
| `list` | List all key-value pairs. |
| `.help` | Show the available commands. |
| `.exit` | Quit the shell (Ctrl-D also works). |

### Example

```
cairndb> set name niksan
Ok
cairndb> set greeting hello world
Ok
cairndb> get greeting
hello world
cairndb> del name
deleted name
cairndb> get name
null
cairndb> .exit
```

### Running the tests

```bash
cargo test
```
