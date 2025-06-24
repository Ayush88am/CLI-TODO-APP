# 📝 CLI Todo App in Rust

A simple command-line Todo app built in Rust, as part of my learning journey through _The Rust Programming Language_ (The Rust Book).

This project is built and tested on [Replit](https://replit.com/) using:

- 📦 `clap` for CLI argument parsing
- 💾 `serde` and `serde_json` for saving tasks in `tasks.json`

---

## 📚 Learning Goals

- ✅ Read Chapters 1–8 of _The Rust Book_
- ✅ Build a fully working CLI app
- ✅ Practice key Rust concepts with real code

---

## 🚀 Features

| Command       | Description                     |
|---------------|---------------------------------|
| `--add`       | Add a new task                  |
| `--remove`    | Remove a task by number or name |
| `--list`      | Show all current tasks          |

Example usage:

```bash
cargo run -- --add "Learn Rust"
cargo run -- --remove 1
cargo run -- --list
