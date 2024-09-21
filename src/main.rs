use clap::{Arg, ArgMatches, Command};
use mdbook::errors::Error;
use mdbook::preprocess::CmdPreprocessor;
use mdbook::preprocess::Preprocessor;

use std::io;
use std::process;

use mdbook_cmdrun::CmdRun;

fn main() {
    let matches = make_app().try_get_matches();

    match matches {
        Ok(matches) => {
            if let Some(sub_args) = matches.subcommand_matches("supports") {
                handle_supports(sub_args);
            } else if let Some(sub_args) = matches.subcommand_matches("cmdrun") {
                let text : String = sub_args
                    .try_get_many::<String>("text")
                    .expect("able to parse a command and not get Err")
                    .expect("able to parse a command and not get None")
                    .map(|s| s.as_str())
                    .collect();
                println!("{}", CmdRun::run_cmdrun(text, ".", false).unwrap());
            } else if let Err(e) = handle_preprocessing() {
                eprintln!("{e}");
                process::exit(1);
            }
        },
        Err(e) => e.exit()
    }
}

fn make_app() -> Command {
    Command::new("mdbook-cmdrun")
        .about("mdbook preprocessor to run arbitrary commands and replace the stdout of these commands inside the markdown file.")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        ).subcommand(
            Command::new("cmdrun")
                .arg(Arg::new("text").num_args(1..).trailing_var_arg(true))
        )
}

fn handle_preprocessing() -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
        eprintln!(
            "Warning: The mdbook-cmdrun preprocessor was built against version \
             {} of mdbook, but we're being called from version {}",
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = CmdRun.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(sub_args: &ArgMatches) -> ! {
    let renderer = sub_args
        .get_one::<String>("renderer")
        .expect("Required argument");
    let supported = CmdRun.supports_renderer(renderer);

    // Signal whether the renderer is supported by exiting with 1 or 0.
    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}
