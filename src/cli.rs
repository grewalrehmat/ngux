use clap::{ArgEnum, Parser};

#[derive(ArgEnum, Clone, Debug, PartialEq)]
pub enum Coloring {
    Automatic,
    Always,
    Never,
}

impl core::str::FromStr for Coloring {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "never" => Coloring::Never,
            "always" => Coloring::Always,
            _ => Coloring::Automatic,
        })
    }
}


#[derive(Debug, Clone, Parser)]
#[clap(author, version, about)]
pub struct Interface {
    /// Print all files and directories, including hidden ones.
    #[clap(long, short, parse(from_flag))]
    pub all: bool,

    /// Use normal print despite gitignore settings. '-a' has higher priority.
    #[cfg(not(target_os = "windows"))]
    #[clap(long, short, parse(from_flag))]
    pub simple: bool,

    /// Only list directories in output.
    #[clap(long, short, parse(from_flag))]
    pub directories: bool,

    /// Create aliases for displayed results (like e42 to open with $EDITOR).
    #[clap(long, short, value_name = "COMMAND")]
    pub editor: Option<Option<String>>,

    /// Output JSON instead of tree diagram.
    #[clap(long, short, parse(from_flag))]
    pub json: bool,

    /// Limit depth of the tree in output.
    #[clap(long, short)]
    pub limit: Option<usize>,

    /// Exclude paths matching a regex pattern. Repeatable.
    #[clap(long, short = 'E', value_name = "PATTERN")]
    pub exclude: Vec<String>,

    /// When to color the output.
    #[clap(
        arg_enum,
        long,
        short,
        value_name = "WHEN",
        default_value = "automatic"
    )]
    pub color: Coloring,

    /// Use absolute paths for editor aliases.
    #[clap(short, long)]
    pub portable: bool,

    /// Path to list (default: current directory).
    #[clap(default_value = ".")]
    pub path: String,

    /// 🆕 Read contents from a file
    #[clap(long, value_name = "FILE")]
    pub read_file: Option<String>,

    /// 🆕 Write a simple message to a file
    #[clap(long, value_name = "FILE")]
    pub write_file: Option<String>,

    /// 🆕 Recursively write all source files to a single file
    #[clap(long, value_name = "FILE")]
    pub dump_tree_files_to: Option<String>,
}


#[cfg(test)]
mod test {
    use super::Coloring;
    use core::str::FromStr;
    #[test]
    fn coloring_parsing() {
        assert_eq!(Coloring::Always, Coloring::from_str("always").unwrap());
        assert_eq!(Coloring::Never, Coloring::from_str("never").unwrap());
        assert_eq!(
            Coloring::Automatic,
            Coloring::from_str("automatic").unwrap()
        );
        assert_eq!(Coloring::Always, Coloring::from_str("AlwAys").unwrap());
        assert_eq!(Coloring::Never, Coloring::from_str("NeveR").unwrap());
        assert_eq!(
            Coloring::Automatic,
            Coloring::from_str("AutoMATic").unwrap()
        );
        assert_eq!(Coloring::Automatic, Coloring::from_str("xxx").unwrap());
    }
}
