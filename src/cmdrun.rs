use anyhow::Context;
use anyhow::Result;

use mdbook::book::Book;
use mdbook::book::Chapter;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

use regex::Captures;
use regex::Regex;

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use lazy_static::lazy_static;
use serde::Deserialize;

use crate::utils::map_chapter;

pub struct CmdRun;

#[cfg(any(target_family = "unix", target_family = "other"))]
const LAUNCH_SHELL_COMMAND: &str = "sh";
#[cfg(any(target_family = "unix", target_family = "other"))]
const LAUNCH_SHELL_FLAG: &str = "-c";

#[cfg(target_family = "windows")]
const LAUNCH_SHELL_COMMAND: &str = "cmd";
#[cfg(target_family = "windows")]
const LAUNCH_SHELL_FLAG: &str = "/c";

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

lazy_static! {
    static ref SRC_DIR: String = get_src_dir();
}

#[derive(Deserialize)]
struct BookConfig {
    book: BookField,
}

#[derive(Deserialize)]
struct BookField {
    src: Option<String>,
}

fn get_src_dir() -> String {
    fs::read_to_string(Path::new("book.toml"))
        .map_err(|_| None::<String>)
        .and_then(|fc| toml::from_str::<BookConfig>(fc.as_str()).map_err(|_| None))
        .and_then(|bc| bc.book.src.ok_or(None))
        .unwrap_or_else(|_| String::from("src"))
}

impl CmdRun {
    fn run_on_chapter(chapter: &mut Chapter) -> Result<()> {
        let working_dir = &chapter
            .path
            .to_owned()
            .and_then(|p| {
                Path::new(SRC_DIR.as_str())
                    .join(p)
                    .parent()
                    .map(PathBuf::from)
            })
            .and_then(|p| p.to_str().map(String::from))
            .unwrap_or_default();

        chapter.content = CmdRun::run_on_content(&chapter.content, working_dir)?;

        Ok(())
    }

    // This method is public for regression tests
    pub fn run_on_content(content: &str, working_dir: &str) -> Result<String> {
        let re = Regex::new(r"<!--[ ]*cmdrun (.*)-->\n").unwrap();
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

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        // let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        // eprintln!("command: {}", command);
        // eprintln!("stdout: {:?}", stdout);
        // eprintln!("stderr: {:?}", stderr);

        Ok(stdout)
    }
}
