# 🌳 ngux

**ngux** is a modern, extensible, and enhanced alternative to the classic `tree` command—written in Rust. 


The tree command that I built on was build by dduan here is the repo link -> https://github.com/dduan/tre 

It supports recursive file listing, filtering, editor shortcuts, and even file content dumping.

##  Features

-  Pretty tree view of file structure
-  CLI flags for customization (`--all`, `--json`, `--limit`, etc.)
-  Smart color output
-  Dump contents of project files into one output file (`--dump-tree-files-to`)
-  Read from/write to files directly via CLI
-  Fast and efficient 

---

##  Installation

###  Install Globally from Source

1. Make sure you have [Rust & Cargo](https://rustup.rs/) installed.

2. Clone this repo:
   ```bash
      git clone https://github.com/yourusername/ngux.git
   ```
3. Install it globally:

  ```bash
     cd ngux
     cargo install --path .
  ```
4. Make sure $HOME/.cargo/bin is in your PATH:

  ```bash
     export PATH="$HOME/.cargo/bin:$PATH"
  ```
5. Now `ngux` command should be globally available ,type ngux for the cli to run.
6. It will create a `out.txt` file in the directory that has all the file dump.
