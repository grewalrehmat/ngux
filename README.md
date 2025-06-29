# 🌳 ngux

**ngux** is a modern, extensible, and enhanced alternative to the classic `tree` command—written in Rust.  
It supports recursive file listing, filtering, JSON output, editor shortcuts, and even file content dumping 📁📄.

## ✨ Features

- 📂 Pretty tree view of file structure
- ⚙️ CLI flags for customization (`--all`, `--json`, `--limit`, etc.)
- 🎨 Smart color output
- 📑 Dump contents of project files into one output file (`--dump-tree-files-to`)
- ✍️ Read from/write to files directly via CLI
- ⚡ Fast and efficient (Rust, baby!)

---

## 🚀 Installation

### 📦 Install Globally from Source

1. Make sure you have [Rust & Cargo](https://rustup.rs/) installed.
2. Clone this repo:
   ```bash
   git clone https://github.com/yourusername/ngux.git
   cd ngux
   ```
3. Install it globally:

  ```bash
    cargo install --path .
  ```
4. Make sure $HOME/.cargo/bin is in your PATH:

  ```bash
    export PATH="$HOME/.cargo/bin:$PATH"
  ```
