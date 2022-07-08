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
    pub fn run_on_content(content: &String, working_dir: &String) -> Result<String> {
        let re = Regex::new(r"<!--[ ]*cmdrun(.*)-->\n").unwrap();
        let mut err = None;

        let content = re
            .replace_all(content, |caps: &Captures| {
                let argv: Vec<&str> = caps[1].trim().split(" ").collect();

                match CmdRun::run_cmdrun(&argv, &working_dir) {
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

    fn run_cmdrun(argv: &Vec<&str>, working_dir: &String) -> Result<String> {
        let output = Command::new(argv[0])
            .args(&argv[1..])
            .stdin(Stdio::null())
            .current_dir(working_dir)
            .output()
            .with_context(|| &"Fail to run command")?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
