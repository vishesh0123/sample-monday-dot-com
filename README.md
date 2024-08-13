
# Yew Task Management App

A simple task management application built with the Yew framework in Rust. This app mimics functionality similar to Monday.com, allowing users to create groups, tasks, and subitems with dynamic interactions.

## Features

- **Group Management**: Create and manage groups with custom names and colors.
- **Task Management**: Add tasks to groups with details such as name, date, area, project owner, notes, files, and budget.
- **Subitem Management**: Add subitems to tasks with similar details, all while allowing inline editing.
- **Dynamic Rendering**: Real-time updates of task and subitem details, leveraging the reactive nature of the Yew framework.

## Prerequisites

- **Rust Version**: 1.80.0 or later.
- **Trunk**: A WASM web application bundler for Rust.

## Installation

### Install Rust

Ensure you have Rust installed on your system. If not, you can install it via [rustup](https://rustup.rs/).

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update stable
```

Make sure your Rust version is up to date:

```bash
rustc --version
```

The output should be:

```
rustc 1.80.0 (your specific build date)
```

### Install Trunk

You can install Trunk via Cargo, the Rust package manager:

```bash
cargo install trunk
```

### Clone the Repository

Clone this repository to your local machine:

```bash
git clone https://github.com/vishesh0123/sample-monday-dot-com
cd sample-monday-dot-com
```

### Build and Serve the Project

Use Trunk to build and serve the project:

```bash
trunk serve
```

This command will compile the project and serve it locally. You can then view it in your web browser at `http://localhost:8080`.

## Usage

- **Add Groups**: Click the "Add Group" button to create a new group. Each group will be displayed with its assigned color and name.
- **Add Tasks**: Within a group, you can add tasks by typing in the input field and pressing Enter.
- **Add Subitems**: Expand a task and add subitems with detailed information.
- **Edit Inline**: Click on task or subitem fields to edit them directly.

