use std::process;
use structopt::StructOpt;

use pier::{
    cli::{Cli, CliSubcommand},
    open_editor,
    script::Script,
    Pier, PierResult,
};

fn main() {
    let opt = Cli::from_args();

    match handle_subcommands(opt) {
        Ok(status) => {
            if let Some(status) = status {
                let code = status.code().unwrap_or(0);
                process::exit(code)
            } else {
                process::exit(0)
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };
}

/// Handles the commandline subcommands
fn handle_subcommands(cli: Cli) -> PierResult<Option<process::ExitStatus>> {
    if let Some(subcmd) = cli.cmd {
        match subcmd {
            CliSubcommand::Add {
                query,
                alias,
                sources,
                description,
                references,
                tags,
                force,
            } => {
                let mut pier = Pier::from(cli.opts.path, cli.opts.verbose)?;
                pier.add_script(
                    Script {
                        alias,
                        query: match query {
                            Some(query) => query,
                            None => open_editor(None)?,
                        },
                        sources,
                        description,
                        references,
                        tags,
                    },
                    force,
                )?;
                pier.write()?;
            }

            CliSubcommand::Edit { alias } => {
                let mut pier = Pier::from(cli.opts.path, cli.opts.verbose)?;
                pier.edit_script(&alias)?;
                pier.write()?;
            }
            CliSubcommand::Remove { alias } => {
                let mut pier = Pier::from(cli.opts.path, cli.opts.verbose)?;
                pier.remove_script(&alias)?;
                pier.write()?;
            }
            CliSubcommand::ConfigInit => {
                let mut pier = Pier::new();
                pier.config_init(cli.opts.path)?;
            }
            CliSubcommand::Show { alias } => {
                let pier = Pier::from(cli.opts.path, cli.opts.verbose)?;
                let script = pier.fetch_script(&alias)?;
                println!("{}", script.query);
            }
            CliSubcommand::List {
                list_aliases,
                tags,
                query_full,
                query_width,
            } => {
                let pier = Pier::from(cli.opts.path, cli.opts.verbose)?;
                if list_aliases {
                    pier.list_aliases(tags)?
                } else {
                    pier.list_scripts(tags, query_full, query_width)?
                }
            }
            CliSubcommand::Copy {
                from_alias,
                to_alias,
            } => {
                let mut pier = Pier::from(cli.opts.path, cli.opts.verbose)?;
                pier.copy_script(&from_alias, &to_alias)?;
                pier.write()?;
            }
            CliSubcommand::Move {
                from_alias,
                to_alias,
                force,
            } => {
                let mut pier = Pier::from(cli.opts.path, cli.opts.verbose)?;
                pier.move_script(&from_alias, &to_alias, force)?;
                pier.write()?;
            }
        };
    }
    Ok(None)
}
