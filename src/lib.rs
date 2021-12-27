use prettytable::{cell, row, Table};
use snafu::{ensure, OptionExt, ResultExt};
use std::fs;
use std::path::PathBuf;
pub mod cli;
mod config;
pub mod error;
use config::Config;
mod defaults;
mod macros;
use defaults::*;
pub mod script;
use error::*;
use scrawl;
use script::Script;

// Creates a Result type that return PierError by default
pub type PierResult<T, E = PierError> = ::std::result::Result<T, E>;

/// Main library interface
#[derive(Debug, Default)]
pub struct Pier {
    config: Config,
    path: PathBuf,
    verbose: bool,
}

#[macro_use]
extern crate lazy_static;

use prettytable::format::FormatBuilder;
use prettytable::format::LinePosition;
use prettytable::format::LineSeparator;
use prettytable::format::TableFormat;

lazy_static! {
    static ref COOL_SEP: LineSeparator =
        LineSeparator::new('\u{2256}', '\u{2256}', '\u{2256}', '\u{2256}');
    pub static ref COOL_FORMAT: TableFormat = FormatBuilder::new()
        .column_separator('\u{22EE}')
        .borders('\u{22EE}')
        .separator(LinePosition::Title, *COOL_SEP)
        .separator(LinePosition::Bottom, *COOL_SEP)
        .separator(LinePosition::Top, *COOL_SEP)
        .padding(1, 1)
        .build();
}

impl Pier {
    /// Wrapper to write the configuration to path.
    pub fn write(&self) -> PierResult<()> {
        self.config.write(&self.path)?;

        Ok(())
    }

    pub fn config_init(&mut self, new_path: Option<PathBuf>) -> PierResult<()> {
        self.path = new_path
            .unwrap_or(fallback_path().unwrap_or(xdg_config_home!("pier/config.toml").unwrap()));

        ensure!(!self.path.exists(), ConfigInitFileAlreadyExists {
            path: &self.path.as_path()
        });

        if let Some(parent_dir) = &self.path.parent() {
            if !parent_dir.exists() {
                fs::create_dir(parent_dir).context(CreateDirectory)?;
            }
        };

        &self.add_script(
            Script {
                alias: String::from("hello-pier"),
                query: String::from("select * from querylake"),
                sources: None,
                description: Some(String::from("This is an example query.")),
                references: None,
                tags: None,
            },
            false,
        );

        self.write()?;

        Ok(())
    }

    pub fn new() -> Self {
        Pier::default()
    }

    /// Create new pier directly from path.
    pub fn from_file(path: PathBuf, verbose: bool) -> PierResult<Self> {
        let pier = Self {
            config: Config::from(&path)?,
            verbose,
            path,
        };
        Ok(pier)
    }
    /// Create new pier from what might be a path, otherwise use the first existing default path.
    pub fn from(input_path: Option<PathBuf>, verbose: bool) -> PierResult<Self> {
        let path = match input_path {
            Some(path) => path,
            None => fallback_path()?,
        };

        let pier = Pier::from_file(path, verbose)?;

        Ok(pier)
    }

    /// Fetches a query that matches the alias
    pub fn fetch_script(&self, alias: &str) -> PierResult<&Script> {
        ensure!(!self.config.scripts.is_empty(), NoScriptsExists);

        let script = self.config.scripts.get(alias).context(AliasNotFound {
            alias: &alias.to_string(),
        })?;

        Ok(script)
    }

    /// Edits a query that matches the alias
    pub fn edit_script(&mut self, alias: &str) -> PierResult<&Script> {
        ensure!(!self.config.scripts.is_empty(), NoScriptsExists);

        let mut script = self.config.scripts.get_mut(alias).context(AliasNotFound {
            alias: &alias.to_string(),
        })?;

        script.query = open_editor(Some(&script.query))?;

        println!("Edited {}", &alias);

        Ok(script)
    }

    /// Removes a query that matches the alias
    pub fn remove_script(&mut self, alias: &str) -> PierResult<()> {
        ensure!(!self.config.scripts.is_empty(), NoScriptsExists);

        self.config.scripts.remove(alias).context(AliasNotFound {
            alias: &alias.to_string(),
        })?;

        println!("Removed {}", &alias);

        Ok(())
    }

    /// Adds a query that matches the alias
    pub fn add_script(&mut self, script: Script, force: bool) -> PierResult<()> {
        if !force {
            ensure!(
                !&self.config.scripts.contains_key(&script.alias),
                AliasAlreadyExists {
                    alias: script.alias
                }
            );
        }

        println!("Added {}", &script.alias);

        self.config.scripts.insert(script.alias.to_string(), script);

        Ok(())
    }

    /// Prints only the aliases in current config file that matches tags.
    pub fn list_aliases(&self, tags: Option<Vec<String>>) -> PierResult<()> {
        ensure!(!self.config.scripts.is_empty(), NoScriptsExists);

        for (alias, script) in self.config.scripts.iter() {
            match (&tags, &script.tags) {
                (Some(list_tags), Some(script_tags)) => {
                    for tag in list_tags {
                        if script_tags.contains(tag) {
                            println!("{}", alias);

                            continue;
                        }
                    }
                }
                (None, _) => {
                    println!("{}", alias);

                    continue;
                }
                _ => (),
            };
        }

        Ok(())
    }

    /// Copy an alias a script that matches the alias
    pub fn copy_script(&mut self, from_alias: &str, new_alias: &str) -> PierResult<()> {
        ensure!(
            !&self.config.scripts.contains_key(new_alias),
            AliasAlreadyExists { alias: new_alias }
        );

        // TODO: refactor the line below.
        let script = self
            .config
            .scripts
            .get(from_alias)
            .context(AliasNotFound {
                alias: &from_alias.to_string(),
            })?
            .clone();

        println!(
            "Copy from alias {} to new alias {}",
            &from_alias.to_string(),
            &new_alias.to_string()
        );

        self.config.scripts.insert(new_alias.to_string(), script);

        Ok(())
    }

    /// Move a script that matches the alias to another alias
    pub fn move_script(
        &mut self,
        from_alias: &str,
        new_alias: &str,
        force: bool,
    ) -> PierResult<()> {
        if !force {
            ensure!(
                !&self.config.scripts.contains_key(new_alias),
                AliasAlreadyExists { alias: new_alias }
            );
        }

        let script = self
            .config
            .scripts
            .remove(from_alias)
            .context(AliasNotFound {
                alias: &from_alias.to_string(),
            })?
            .clone();

        println!(
            "Move from alias {} to new alias {}",
            &from_alias.to_string(),
            &new_alias.to_string()
        );

        self.config.scripts.insert(new_alias.to_string(), script);

        Ok(())
    }

    /// Prints a terminal table of the scripts in current config file that matches tags.
    pub fn list_scripts(
        &self,
        tags: Option<Vec<String>>,
        query_full: bool,
        query_width: Option<usize>,
    ) -> PierResult<()> {
        let width = match (query_width, self.config.default.query_width) {
            (Some(width), _) => width,
            (None, Some(width)) => width,
            (None, None) => FALLBACK_QUERY_DISPLAY_WIDTH,
        };
        ensure!(!self.config.scripts.is_empty(), NoScriptsExists);

        let mut table = Table::new();

        table.set_format(*COOL_FORMAT);
        // cyan titles
        table.set_titles(row![
            Fc -> "Alias",
            Fc -> "Tags",
            Fc -> "Query",
            Fc -> "Data Sources",
            Fc -> "Description",
            Fc -> "References",
            ]);

        for (alias, script) in self.config.scripts.iter() {
            let descp = match &script.description.as_ref() {
                Some(d) => d,
                None => "",
            };

            let script_tags = match (&tags, &script.tags) {
                (Some(list_tags), Some(script_tags)) => {
                    for tag in list_tags {
                        if script_tags.contains(tag) {
                            table.add_row(row![
                                FY -> &alias,
                                Fg -> script_tags.join(","),
                                Fb -> script.display_query(query_full, width),
                                Fw -> script.sources.join(","),
                                Fw -> descp,
                                Fw -> script.references.join(","),
                            ]);
                            continue;
                        }
                    }
                }
                (None, Some(script_tags)) => {
                       script_tags.join(",");
                       continue;
                }
                (None, None) => {
                       "";
                       continue;
                }
                _ => (),
            };
        }

        // forced color explicitly. works in pipes
        table.print_tty(true);

        Ok(())
    }
}

pub fn open_editor(content: Option<&str>) -> PierResult<String> {
    let edited_text = scrawl::editor::new()
        .contents(match content {
            Some(txt) => txt,
            None => "",
        })
        .open()
        .context(EditorError)?;

    Ok(edited_text)
}
