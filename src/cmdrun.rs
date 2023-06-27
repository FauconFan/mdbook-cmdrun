use anyhow::Context;
use anyhow::Result;
use cfg_if::cfg_if;

use mdbook::book::Book;
use mdbook::book::Chapter;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

use regex::Captures;
use regex::Regex;

use std::path::Path;
use std::process::Command;

use crate::utils::map_chapter;

pub struct CmdRun;

cfg_if! {
    if #[cfg(target_family = "windows")] {
        const LAUNCH_SHELL_COMMAND: &str = "cmd";
        const LAUNCH_SHELL_FLAG: &str = "/C";
    } else if #[cfg(any(target_family = "unix", target_family = "other"))] {
        const LAUNCH_SHELL_COMMAND: &str = "sh";
        const LAUNCH_SHELL_FLAG: &str = "-c";
    }
}

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
        let re = Regex::new(r"<!--[ ]*cmdrun (.*)-->").unwrap();
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

    // This method is public for unit tests
    pub fn run_cmdrun(command: String, working_dir: &str) -> Result<String> {
        let output = Command::new(LAUNCH_SHELL_COMMAND)
            .args([LAUNCH_SHELL_FLAG, &command])
            .current_dir(working_dir)
            .output()
            .with_context(|| "Fail to run shell")?;

        let stdout = String::from_utf8_lossy(&output.stdout)
            .trim_end()
            .to_string();

        // let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        // eprintln!("command: {}", command);
        // eprintln!("stdout: {:?}", stdout);
        // eprintln!("stderr: {:?}", stderr);

        Ok(stdout)
    }
}
