use std::sync::Arc;

// Library Imports
use anyhow::Result;
use async_trait::async_trait;
use colored::Colorize;

// Crate Level Imports
use crate::utils::App;
use crate::VERSION;

// Super Imports
use super::Command;

/// Struct implementation for the `Help` command.
pub struct Help;

#[async_trait]
impl Command for Help {
    /// Display a help menu for the `volt help` command.
    fn help() -> String {
        format!(
            r#"volt {}
    
Displays help information.
Usage: {} {} {}
Commands:
  {} {} - Install all dependencies for a project.
  {} {} - Interactively create or update a package.json file for a project.
  {} {} - Add a dependency to a project.
  {} {} - Remove a dependency from the package.json file for a project.
  {} {} - Push changes to a github repository the easy way.
  {} {} - Clean the volt cache files and metadata.
  {} {} - Clone a github repository and get setup with all required dependencies.
  "#,
            VERSION.bright_green().bold(),
            "volt".bright_green().bold(),
            "[commands]".bright_purple(),
            "[flags]".bright_purple(),
            "*".bright_magenta().bold(),
            "install".bright_blue(),
            "*".bright_magenta().bold(),
            "init".bright_blue(),
            "*".bright_magenta().bold(),
            "add".bright_blue(),
            "*".bright_magenta().bold(),
            "remove".bright_blue(),
            "*".bright_magenta().bold(),
            "cache".bright_blue(),
            "*".bright_magenta().bold(),
            "deploy".bright_blue(),
            "*".bright_magenta().bold(),
            "clone".bright_blue(),
        )
    }

    /// Execute the `volt help` command
    ///
    /// Displays help information.
    /// ## Arguments
    /// * `app` - Instance of the command (`Arc<App>`)
    /// * `packages` - List of packages to add (`Vec<String>`)
    /// * `flags` - List of flags passed in through the CLI (`Vec<String>`)
    /// ## Examples
    /// ```
    /// // Display a help menu.
    /// // .exec() is an async call so you need to await it
    /// Help.exec(app, vec![], vec![]).await;
    /// ```
    /// ## Returns
    /// * `Result<()>`
    async fn exec(_app: Arc<App>) -> Result<()> {
        println!("{}", Self::help());
        Ok(())
    }
}
