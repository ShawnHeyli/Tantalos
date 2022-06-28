pub mod args {
    use clap::Parser;
    use std::path::PathBuf;

    #[derive(Parser, Debug)]
    #[clap(author, version, about, long_about = None)]
    pub struct Args {
        /// Path of the repository to analyze
        #[clap(short = 'p', long = "path", parse(from_os_str), default_value = "./")]
        pub repo_path: PathBuf,

        /// Path of the output file
        #[clap(short, long, parse(from_os_str), default_value = "./gitGraph.mmd")]
        pub output: PathBuf,

        /// Theme to use
        #[clap(short, long, value_parser, default_value = "base")]
        pub theme: String,

        /// Show branches names
        #[clap(long, value_parser, default_value = "true")]
        pub show_branches: bool,

        /// Rotate commit label
        #[clap(long, value_parser, default_value = "true")]
        pub rotate_commit_label: bool,

        /// Show commit label
        #[clap(long, value_parser, default_value = "true")]
        pub show_commit_label: bool,
    }
}
