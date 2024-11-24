use std::borrow::Cow;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Context;
use anyhow::Result;
use cfg_if::cfg_if;
use lazy_static::lazy_static;
use regex::Captures;
use regex::Regex;
use serde::Deserialize;

use mdbook::book::Book;
use mdbook::book::Chapter;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

use crate::utils::map_chapter;

pub struct CmdRun;

lazy_static! {
    static ref CMDRUN_REG_NEWLINE: Regex = Regex::new(r"<!--[ ]*cmdrun (.*?)-->\r?\n")
        .expect("Failed to init regex for finding newline pattern");
    static ref CMDRUN_REG_INLINE: Regex = Regex::new(r"<!--[ ]*cmdrun (.*?)-->")
        .expect("Failed to init regex for finding inline pattern");
}

cfg_if! {
    if #[cfg(target_family = "unix")] {
        const LAUNCH_SHELL_COMMAND: &str = "sh";
        const LAUNCH_SHELL_FLAG: &str = "-c";
    } else if #[cfg(target_family = "windows")] {
        const LAUNCH_SHELL_COMMAND: &str = "cmd";
        const LAUNCH_SHELL_FLAG: &str = "/C";
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
        let mut err = None;

        let mut result = CMDRUN_REG_NEWLINE
            .replace_all(content, |caps: &Captures| {
                Self::run_cmdrun(caps[1].to_string(), working_dir, false).unwrap_or_else(|e| {
                    err = Some(e);
                    String::new()
                })
            })
            .to_string();

        if let Some(e) = err {
            return Err(e);
        }

        result = CMDRUN_REG_INLINE
            .replace_all(result.as_str(), |caps: &Captures| {
                Self::run_cmdrun(caps[1].to_string(), working_dir, true).unwrap_or_else(|e| {
                    err = Some(e);
                    String::new()
                })
            })
            .to_string();

        match err {
            None => Ok(result),
            Some(err) => Err(err),
        }
    }

    // Some progams output linebreaks in UNIX format,
    // this can cause problems on Windows if for any reason
    // the user is expecting consistent linebreaks,
    // e.g. they run the resulting markdown through a validation tool.
    //
    // So this function simply replaces all linebreaks with Windows linebreaks.
    #[cfg(target_family = "windows")]
    fn format_whitespace(str: Cow<'_, str>, inline: bool) -> String {
        let str = match inline {
            // When running inline it is undeseriable to have trailing whitespace
            true => str.trim_end(),
            false => str.as_ref(),
        };

        let mut res = str.lines().collect::<Vec<_>>().join("\r\n");
        if !inline && !res.is_empty() {
            res.push_str("\r\n");
        }

        return res;
    }

    #[cfg(target_family = "unix")]
    fn format_whitespace(str: Cow<'_, str>, inline: bool) -> String {
        match inline {
            // Wh;n running inline it is undeseriable to have trailing whitespace
            true => str.trim_end().to_string(),
            false => str.to_string(),
        }
    }

    fn cmdrun_error_message(message: &str, command: &str) -> String {
        format!("**cmdrun error**: {} in 'cmdrun {}'", message, command)
    }

    // This method is public for unit tests
    pub fn run_cmdrun(command: String, working_dir: &str, inline: bool) -> Result<String> {
        // unfortunately, we need to manually parse the command string for cmdrun's
        // exit status checking flags.
        // Some experimentation using clap was done; however, splitting and then re-escaping
        // the shellwords was found to be a large barrier to using this other tool.
        let (command, correct_exit_code): (String, Option<i32>) =
            if let Some(first_word) = command.split_whitespace().next() {
                if first_word.starts_with('-') {
                    if first_word.starts_with("--") {
                        // double-tick long form
                        match first_word {
                            "--strict" => (
                                command
                                    .split_whitespace()
                                    .skip(1)
                                    .collect::<Vec<&str>>()
                                    .join(" "),
                                Some(0),
                            ),
                            "--expect-return-code" => {
                                if let Some(second_word) = command.split_whitespace().nth(1) {
                                    match second_word.parse::<i32>() {
                                        Ok(return_code) => (
                                            command
                                                .split_whitespace()
                                                .skip(2)
                                                .collect::<Vec<&str>>()
                                                .join(" "),
                                            Some(return_code),
                                        ),
                                        Err(_) => {
                                            return Ok(Self::cmdrun_error_message(
                                                "No return code after '--expect-return-code'",
                                                &command,
                                            ));
                                        }
                                    }
                                } else {
                                    // no second word after return code, print error
                                    return Ok(Self::cmdrun_error_message(
                                        "No return code after '--expect-return-code'",
                                        &command,
                                    ));
                                }
                            }
                            some_other_word => {
                                // unrecognized flag, print error
                                return Ok(Self::cmdrun_error_message(
                                    &format!("Unrecognized cmdrun flag {}", some_other_word),
                                    &command,
                                ));
                            }
                        }
                    } else {
                        // single-tick short form
                        let (_, exit_code) = first_word.rsplit_once('-').unwrap_or(("", "0"));
                        match exit_code.parse::<i32>() {
                            Ok(return_code) => (
                                command
                                    .split_whitespace()
                                    .skip(1)
                                    .collect::<Vec<&str>>()
                                    .join(" "),
                                Some(return_code),
                            ),
                            Err(_) => {
                                return Ok(Self::cmdrun_error_message(
                                    &format!(
                                        "Unable to interpret short-form exit code {} as a number",
                                        first_word
                                    ),
                                    &command,
                                ));
                            }
                        }
                    }
                } else {
                    (command, None)
                }
            } else {
                (command, None)
            };

        let output = Command::new(LAUNCH_SHELL_COMMAND)
            .args([LAUNCH_SHELL_FLAG, &command])
            .current_dir(working_dir)
            .output()
            .with_context(|| "Fail to run shell")?;

        let stdout = Self::format_whitespace(String::from_utf8_lossy(&output.stdout), inline);
        match (output.status.code(), correct_exit_code) {
            (None, _) => Ok(Self::cmdrun_error_message(
                "Command was ended before completing",
                &command,
            )),
            (Some(code), Some(correct_code)) => {
                if code != correct_code {
                    Ok(format!(
                        "**cmdrun error**: '{command}' returned exit code {code} instead of {correct_code}.\n{0}\n{1}",
                        String::from_utf8_lossy(&output.stdout),
                        String::from_utf8_lossy(&output.stderr)))
                } else {
                    Ok(stdout)
                }
            }
            (Some(_code), None) => {
                // no correct code specified, program exited with some code _code
                // could put default check requiring code to be zero here but
                // that would break current behavior
                Ok(stdout)
            }
        }
    }
}
