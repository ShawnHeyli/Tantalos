pub mod args {
    use clap::Parser;
    use std::path::PathBuf;

    #[derive(Parser, Debug)]
    #[clap(author, version, about, long_about = None)]
    pub struct Args {
        /// Path of the repository to analyze
        #[clap(short = 'p', long = "path", parse(from_os_str), default_value = "./")]
        repo_path: PathBuf,

        /// Path of the output file
        #[clap(short, long, parse(from_os_str), default_value = "./gitGraph.mmd")]
        output: PathBuf,
    }
}
