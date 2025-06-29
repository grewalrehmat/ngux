use clap::Parser;

mod cli;
mod diagram_formatting;
mod file_tree;
mod json_formatting;
mod output;
mod path_finders;
mod tre;
mod file_ops;
use file_ops::*;

fn main() {
    let inputs = cli::Interface::parse();

    // 👇 Read a single file
    if let Some(path) = &inputs.read_file {
        match read_file(path) {
            Ok(content) => println!("📄 File content:\n{content}"),
            Err(e) => eprintln!("❌ Failed to read file: {e}"),
        }
        return;
    }

    // 👇 Write "Hello from tre!" to a file (basic write)
    if let Some(path) = &inputs.write_file {
        if let Err(e) = write_file(path, "Hello from tre!") {
            eprintln!("❌ Failed to write to file: {e}");
        } else {
            println!("✅ Successfully wrote to {path}");
        }
        return;
    }

    // 👇 Write ALL source files into an output file (recursive content dump)
    if let Some(path) = &inputs.dump_tree_files_to {
        if let Err(e) = write_all_files_to(".", path) {
            eprintln!("❌ Failed to dump files recursively: {e}");
        } else {
            println!("✅ Project contents dumped into {path}");
        }
        return;
    }

    // 👇 Default: run tre
    let options: tre::RunOptions = inputs.into();
    tre::run(options);
}
