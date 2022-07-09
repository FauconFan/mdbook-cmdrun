use anyhow::Context;
use anyhow::Result;

use mdbook::book::Book;
use mdbook::book::Chapter;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

use regex::Captures;
use regex::Regex;

use std::path::Path;
use std::process::{Command, Stdio};

use crate::utils::map_chapter;

pub struct CmdRun;

impl Preprocessor for CmdRun {
    fn name(&self) -> &str {
        "cmdrun"
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        map_chapter(&mut book, &mut CmdRun::run_on_chapter)?;

        Ok(book)
    }
}

impl CmdRun {
    fn run_on_chapter(chapter: &mut Chapter) -> Result<()> {
        let working_dir = match &chapter.path {
            None => String::new(),
            Some(pathbuf) => {
                let pathbuf = Path::new("src").join(pathbuf);
                match pathbuf.parent() {
                    None => String::new(),
                    Some(parent) => match parent.to_str() {
                        None => String::new(),
                        Some(s) => String::from(s),
                    },
                }
            }
        };

        chapter.content = CmdRun::run_on_content(&chapter.content, &working_dir)?;

        Ok(())
    }

    // This method is public for regression tests
    pub fn run_on_content(content: &str, working_dir: &str) -> Result<String> {
        let re = Regex::new(r"<!--[ ]*cmdrun(.*)-->\n").unwrap();
        let mut err = None;

        let content = re
            .replace_all(content, |caps: &Captures| {
                match CmdRun::run_cmdrun(caps[1].to_string(), working_dir) {
                    Ok(s) => s,
                    Err(e) => {
                        err = Some(e);
                        String::new()
                    }
                }
            })
            .to_string();

        match err {
            None => Ok(content),
            Some(err) => Err(err),
        }
    }

    fn run_cmdrun(command: String, working_dir: &str) -> Result<String> {
        let mut res = String::new();

        for command in command.split(';') {
            let mut previous_stdout = None;
            let connected_commands: Vec<_> = command.split('|').collect();
            // let mut childs = vec![];

            for index in 0..(connected_commands.len() - 1) {
                let words: Vec<_> = connected_commands[index]
                    .split(' ')
                    .filter(|s| s.len() > 0)
                    .collect();

                let mut command = Command::new(words[0]);
                command.args(&words[1..]);
                command.current_dir(working_dir);

                if let Some(stdout) = previous_stdout {
                    command.stdin(stdout);
                }

                command.stdout(Stdio::piped());

                let mut child = command
                    .spawn()
                    .with_context(|| format!("Fail to spawn child with command {:?}", words))?;

                previous_stdout = child.stdout.take();
            }

            {
                let last_command = connected_commands[connected_commands.len() - 1];
                let words: Vec<_> = last_command.split(' ').filter(|s| s.len() > 0).collect();

                let mut command = Command::new(words[0]);
                command.args(&words[1..]);
                command.current_dir(working_dir);

                if let Some(stdout) = previous_stdout {
                    command.stdin(stdout);
                }

                let output = command
                    .output()
                    .with_context(|| format!("Fail to spawn child with command {:?}", words))?;

                res.push_str(&String::from_utf8_lossy(&output.stdout))
            }
        }

        Ok(res)
    }
}
