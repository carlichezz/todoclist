

▄▄▄█████▓ ▒█████  ▓█████▄  ▒█████   ▄████▄   ██▓     ██▓  ██████ ▄▄▄█████▓
▓  ██▒ ▓▒▒██▒  ██▒▒██▀ ██▌▒██▒  ██▒▒██▀ ▀█  ▓██▒    ▓██▒▒██    ▒ ▓  ██▒ ▓▒
▒ ▓██░ ▒░▒██░  ██▒░██   █▌▒██░  ██▒▒▓█    ▄ ▒██░    ▒██▒░ ▓██▄   ▒ ▓██░ ▒░
░ ▓██▓ ░ ▒██   ██░░▓█▄   ▌▒██   ██░▒▓▓▄ ▄██▒▒██░    ░██░  ▒   ██▒░ ▓██▓ ░ 
  ▒██▒ ░ ░ ████▓▒░░▒████▓ ░ ████▓▒░▒ ▓███▀ ░░██████▒░██░▒██████▒▒  ▒██▒ ░ 
  ▒ ░░   ░ ▒░▒░▒░  ▒▒▓  ▒ ░ ▒░▒░▒░ ░ ░▒ ▒  ░░ ▒░▓  ░░▓  ▒ ▒▓▒ ▒ ░  ▒ ░░   
    ░      ░ ▒ ▒░  ░ ▒  ▒   ░ ▒ ▒░   ░  ▒   ░ ░ ▒  ░ ▒ ░░ ░▒  ░ ░    ░    
  ░      ░ ░ ░ ▒   ░ ░  ░ ░ ░ ░ ▒  ░          ░ ░    ▒ ░░  ░  ░    ░      
             ░ ░     ░        ░ ░  ░ ░          ░  ░ ░        ░           
                   ░               ░                       

# todoclist

CLI app to optimize your daily tasks.

## Overview

A simple command-line to-do list manager written in Rust. Tasks are stored in a local `tasks.json` file.

## Build & Run

```bash
cargo build --release
cargo run -- <command>
```

## Usage

### Add a task

```bash
cargo run -- add "Task name"
cargo run -- add "Task name" -d "Optional description"
```

### List all tasks

```bash
cargo run -- list
```

## Task Structure

| Field         | Type     | Description          |
|---------------|----------|----------------------|
| `id`          | `u32`    | Auto-incremented ID  |
| `name`        | `String` | Task name            |
| `description` | `String` | Task description     |

Tasks are persisted in `tasks.json` in the current working directory.

## Dependencies

- [clap](https://crates.io/crates/clap) 4.6.1 — CLI argument parsing
- [serde](https://crates.io/crates/serde) 1 — Serialization/deserialization
- [serde_json](https://crates.io/crates/serde_json) 1 — JSON support
