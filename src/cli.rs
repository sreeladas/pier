use std::path::PathBuf;
use structopt::{clap::AppSettings, StructOpt};

#[derive(Debug, StructOpt)]
pub enum CliSubcommand {
    /// Add a new script to config.
    Add {
        /// The query/script content.
        /// If this argument is not found it will open your $EDITOR for you to enter the script into.
        query: Option<String>,

        /// The alias or name for the script.
        #[structopt(short = "a", long = "alias")]
        alias: String,

        /// Set which sources this query should be run on.
        #[structopt(short = "s", long = "sources")]
        sources: Option<Vec<String>>,

        /// The description for the script.
        #[structopt(short = "d", long = "--description")]
        description: Option<String>,

        /// Set references that explain this query (Notion, Jira, github, zendesk).
        #[structopt(short = "r", long = "refs")]
        references: Option<Vec<String>>,

        /// Set which tags the script belongs to.
        #[structopt(short = "t", long = "tags")]
        tags: Option<Vec<String>>,

        /// Allows to overwrite the existing script
        #[structopt(short = "f", long = "force")]
        force: bool,
    },
    /// alias: rm - Remove a script matching alias.
    #[structopt(alias = "rm")]
    Remove {
        /// The alias or name for the script.
        alias: String,
    },
    /// alias: init - Add a config file.
    #[structopt(alias = "init")]
    ConfigInit,
    /// Edit a script matching alias.
    Edit {
        /// The alias or name for the script.
        alias: String,
    },
    /// Show a script matching alias.
    Show {
        /// The alias or name for the script.
        alias: String,
    },
    /// alias: ls - List scripts
    ///
    /// Display options are determined by priority in this order:
    ///
    /// 1. List only aliases
    ///
    /// 2. Show full command
    ///
    /// 3. Command display with (cli option)
    ///
    /// 4. Command display with (config option)
    #[structopt(alias = "ls")]
    List {
        /// Only displays aliases of the scripts.
        #[structopt(short = "q", long = "list_aliases")]
        list_aliases: bool,

        /// Display the full command.
        #[structopt(short = "l", long = "query_full")]
        query_full: bool,

        /// The max number of characters to display from the command.
        #[structopt(short = "c", long = "query_width")]
        query_width: Option<usize>,

        /// Filter based on tags.
        #[structopt(short = "t", long = "tag")]
        tags: Option<Vec<String>>,
    },
    /// alias: cp - Copy existing alias to the new one
    #[structopt(alias = "cp")]
    Copy {
        /// The alias of the script that will be copied.
        from_alias: String,
        /// The new alias of the copy of the script.
        to_alias: String,
    },
    /// alias: mv, rename - Move/rename existing alias to the new one
    #[structopt(aliases = &["mv", "rename"])]
    Move {
        /// The alias of the script that will be moved.
        from_alias: String,
        /// The new alias of the script.
        to_alias: String,
        /// Allows to overwrite the existing script
        #[structopt(short = "f", long = "force")]
        force: bool,
    },
}

#[derive(StructOpt, Debug)]
pub struct CliOpts {
    /// The level of verbosity
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,

    /// Sets a custom config file.
    ///
    /// DEFAULT PATH is otherwise determined in this order:
    ///
    ///   - $PIER_CONFIG_PATH (environment variable if set)
    ///
    ///   - pier.toml (in the current directory)
    ///
    ///   - $XDG_CONFIG_HOME/pier/config.toml
    ///
    ///   - $XDG_CONFIG_HOME/pier/config
    ///
    ///   - $XDG_CONFIG_HOME/pier.toml
    ///
    ///   - $HOME/.pier.toml
    ///
    ///   - $HOME/.pier
    ///
    #[structopt(short = "c", long = "config-file", env = "PIER_CONFIG_PATH")]
    pub path: Option<PathBuf>,
}

#[derive(Debug, StructOpt)]
#[structopt(setting = AppSettings::SubcommandsNegateReqs, setting = AppSettings::TrailingVarArg, author)]
/// A simple script management CLI
pub struct Cli {
    #[structopt(flatten)]
    pub opts: CliOpts,

    /// The alias or name for the script.
    #[structopt(required_unless = "cmd")]
    pub alias: Option<String>,

    /// The positional arguments to send to script.
    pub args: Vec<String>,

    /// Pier subcommands
    #[structopt(subcommand)]
    pub cmd: Option<CliSubcommand>,
}
