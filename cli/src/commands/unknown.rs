use super::Command;
use crate::{utils::App, utils::VERSION};
use anyhow::{Context, Result};
use async_trait::async_trait;
use colored::Colorize;
use compiler::RustCompiler;
use lexer::{tokenize, Parser};
use no_comment::{languages, IntoWithoutComments as _};
use std::{io::Read, sync::Arc, vec};
pub struct Compile;

#[async_trait]
impl Command for Compile {
    fn help() -> String {
        format!(
            r#"torqc {}
    
Usage: {} {} {}
Commands:
  {asterisk} {} - Compiles the given file
Flags: 
  {asterisk} {} - uses clang (default)
  {asterisk} {} - uses gcc
  {asterisk} {} - verbose output
  {asterisk} {} - Builds for deployement
  {asterisk} {} - Compiles to rust code
  "#,
            VERSION.bright_green().bold(),
            "torqc".bright_green().bold(),
            "[commands]".bright_purple(),
            "[flags]".bright_purple(),
            "<filename>".bright_blue(),
            "--useclang, -ucg".bright_blue(),
            "--usegcc, -ugcc ".bright_blue(),
            "--verbose, -vb  ".bright_blue(),
            "--release, -r   ".bright_blue(),
            "--userust       ".bright_blue(),
            asterisk = "*".bright_magenta().bold(),
        )
    }

    async fn exec(app: Arc<App>) -> Result<()> {
        let acceptedflags: Vec<&str> = vec![
            "--useclang",
            "-ucg",
            "--usegcc",
            "-ugcc",
            "--verbose",
            "-vb",
            "--release",
            "-r",
            "--userust",
        ];
        let _flags = app.filter_flag(&acceptedflags);
        let args = app.args.clone();
        let filename: &str = args[0].as_str();
        let mut file =
            std::fs::File::open(filename).unwrap_or_else(|e| app.error(e.to_string().as_str()));
        let mut f_contents = String::new();

        file.read_to_string(&mut f_contents)
            .unwrap_or_else(|e| app.error(e.to_string().as_str()));

        let preprocessed = f_contents
            .chars()
            .without_comments(languages::rust())
            .collect::<String>();
        let tokenize = tokenize(&preprocessed).context("Failed to tokenize the contents.")?;
        let mut parse = Parser::new(tokenize);
        let mut parsedval = parse.parse().unwrap_or_else(|e| {
            println!("{:#?}", e);
            std::process::exit(1)
        });
        println!("{:#?}", parsedval);
        if app.has_flag(&["--userust"]) {
            let mut rustcompiler = RustCompiler::new(parsedval);
            rustcompiler.compile();
        }
        // compile(parsedval);
        // println!("{:#?}", parse.parse());
        Ok(())
    }
}
