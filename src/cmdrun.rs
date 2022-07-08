use anyhow::Context;
use anyhow::Result;

use mdbook::book::Book;
use mdbook::book::BookItem;
use mdbook::book::Chapter;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

use regex::Captures;
use regex::Regex;

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub struct CmdRun;

impl Preprocessor for CmdRun {
    fn name(&self) -> &str {
        "cmdrun"
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        map_chapter(&mut book, run_on_chapter)?;

        Ok(book)
    }
}

fn map_chapter<F>(book: &mut Book, mut func: F) -> Result<()>
where
    F: FnMut(&mut Chapter) -> Result<()>,
{
    fn _map_chapter_on<F>(item: &mut BookItem, mut func: F) -> Result<()>
    where
        F: FnMut(&mut Chapter) -> Result<()>,
    {
        match item {
            BookItem::Chapter(ref mut chapter) => {
                func(chapter)?;

                for sub_item in &mut chapter.sub_items {
                    _map_chapter_on(sub_item, &mut func)?;
                }
            }
            BookItem::PartTitle(_) | BookItem::Separator => {}
        }
        Ok(())
    }

    for item in &mut book.sections {
        _map_chapter_on(item, &mut func)?;
    }

    Ok(())
}

fn run_on_chapter(chapter: &mut Chapter) -> Result<()> {
    let re = Regex::new(r"<!--[ ]*cmdrun(.*)-->").unwrap();
    let mut err = None;

    chapter.content = re
        .replace_all(&chapter.content, |caps: &Captures| {
            let argv: Vec<&str> = caps[1].trim().split(" ").collect();

            match run_cmdrun(&argv, &chapter.path) {
                Ok(s) => s,
                Err(e) => {
                    err = Some(e);
                    String::new()
                }
            }
        })
        .to_string();

    match err {
        None => Ok(()),
        Some(err) => Err(err),
    }
}

fn run_cmdrun(argv: &Vec<&str>, path: &Option<PathBuf>) -> Result<String> {
    let working_dir = match path {
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

    let output = Command::new(argv[0])
        .args(&argv[1..])
        .stdin(Stdio::null())
        .current_dir(&working_dir)
        .output()
        .with_context(|| &"Fail to run command")?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
