# RI_TODO_LIST

A command-line To-Do List application built with Rust. It supports user registration and login with password hashing, and lets you manage tasks with priorities, statuses, and date-based filtering — all stored locally as JSON files.

---

## Features

- **User Registration & Login** — Register once; credentials are saved locally. Passwords are hashed with SHA-256 before storage.
- **Task Management** — Create, update, and view tasks from the CLI.
- **Priority Levels** — Assign High, Medium, or Low priority to each task.
- **Task Status** — Track tasks as Pending, Completed, or Not Going To Do.
- **Task Views** — Filter tasks by today's date, completion status, priority, or any custom date.
- **Persistent Storage** — User data and tasks are saved in `registration.json` and `tasks.json`.

---

## Project Structure
├── src/
│   ├── main.rs          # Entry point; handles app flow and task operations
│   ├── login.rs         # Login logic and status handling
│   ├── registrtion.rs   # User registration struct and constructor
│   └── todo.rs          # Task struct, priority levels, and task status
├── registration.json    # Stores registered user data (auto-generated)
├── tasks.json           # Stores task list (auto-generated)
└── Cargo.toml

---

## Dependencies

```toml
[dependencies]
chrono = "0.4"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sha2 = "0.10"
hex = "0.4"
rpassword = "7"
uuid = { version = "1", features = ["v4"] }
```

---

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (stable)

### Run the app

```bash
git clone https://github.com/your-username/ri_todo_list.git
cd ri_todo_list
cargo run
```

Before running, make sure empty JSON files exist:

```bash
echo "" > registration.json
echo "" > tasks.json
```

---

## Usage

### First Run — Register
You'll be prompted to enter a username, email, and password. Your password is hashed with SHA-256 and saved securely.

### Subsequent Runs — Login
Enter your username or email along with your password to log in.

### Home Menu
1.Create Task
2.Update Task
3.Delete Task
4.View Tasks
5.Complete Task
6.Quit

You can type the number **or** the option name (e.g. `create task`). Type `back` at any point to return to the previous menu.

### Task Views
1.Today's All Tasks
2.Today's Pending Tasks
3.Today's Completed Tasks
4.Priority Wise Tasks
5.Date Wise Tasks
6.Back to Home Page

---

## Known Limitations / Work in Progress

- Delete Task and Complete Task options are not yet fully implemented.
- Task description and priority updates are stubbed out.
- No multi-user support — one registered user per installation.
- Data is stored in plain JSON files with no encryption (except the hashed password).

---

## License

MIT