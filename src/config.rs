#[allow(non_snake_case)]
pub mod config {
    use std::fmt::Display;

    use crate::args::args::Args;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Config {
        init: Init,
    }

    #[derive(Serialize, Deserialize)]
    struct Init {
        theme: String,
        gitGraph: GitGraph,
    }

    #[derive(Serialize, Deserialize)]

    struct GitGraph {
        showBranches: bool,
        rotateCommitLabel: bool,
        showCommitLabel: bool,
    }

    impl Display for Config {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.to_string())
        }
    }

    impl Config {
        pub fn new(args: &Args) -> Config {
            let init = Init {
                theme: args.theme.to_string(),
                gitGraph: GitGraph {
                    showBranches: args.show_branches,
                    rotateCommitLabel: args.rotate_commit_label,
                    showCommitLabel: args.show_commit_label,
                },
            };
            Config { init }
        }

        pub fn to_string(&self) -> String {
            let mut string = String::new();
            let config_json = serde_json::to_string(&self).unwrap();
            string.push_str(&format!("%%{}%%", config_json));
            string
        }
    }
}
