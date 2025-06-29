use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

/// Reads the entire contents of a file into a string
pub fn read_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

/// Overwrites or creates a file with given data
pub fn write_file(path: &str, data: &str) -> io::Result<()> {
    fs::write(path, data)
}

/// Appends data to a file, creating it if it doesn't exist
pub fn append_file(path: &str, data: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).create(true).open(path)?;
    writeln!(file, "{data}")?;
    Ok(())
}

/// Recursively dumps all readable source files into a single output file
pub fn write_all_files_to(path: &str, out_file: &str) -> io::Result<()> {
    let mut output = File::create(out_file)?;
    recurse_write(Path::new(path), &mut output)?;
    Ok(())
}

/// Recursively visits directories and writes contents of each file into `output`
fn recurse_write(dir: &Path, output: &mut File) -> io::Result<()> {
    // Ignore common noisy or build-related directories
    let ignored_dirs = ["venv", ".venv", "__pycache__", ".git", "target", "node_modules", ".idea", ".DS_Store"];

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if ignored_dirs.contains(&name) {
                        continue;
                    }
                }
                recurse_write(&path, output)?;
            } else if path.is_file() {
                let display_path = path.display();
                writeln!(output, "==> {}\n", display_path)?;
                let content = fs::read_to_string(&path)
                    .unwrap_or_else(|_| "[⚠️ Could not read file]\n".to_string());
                writeln!(output, "{}\n", content)?;
            }
        }
    }
    Ok(())
}
