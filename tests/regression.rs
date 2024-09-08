use cfg_if::cfg_if;
use std::fs;

cfg_if! {
    if #[cfg(target_family = "unix")] {
        const INPUT_FILE: &str = "input.md";
        const OUTPUT_FILE: &str = "output.md";
    } else if  #[cfg(target_family = "windows")]{
        const INPUT_FILE: &str = "input_win.md";
        const OUTPUT_FILE: &str = "output_win.md";
    }
}

macro_rules! add_dir {
    ($working_dir:ident) => {
        #[cfg(test)]
        mod $working_dir {
            use super::*;

            use mdbook_cmdrun::CmdRun;

            #[test]
            fn regression() {
                let working_dir = format!("./tests/regression/{}", stringify!($working_dir));
                let mut input = None;
                let mut output = None;

                for file in fs::read_dir(&working_dir).expect(&format!(
                    "expect to read the current direcory {}",
                    &working_dir
                )) {
                    let file = file.unwrap();

                    if file.file_name().as_os_str() == INPUT_FILE {
                        input = Some(file);
                    } else if file.file_name().as_os_str() == OUTPUT_FILE {
                        output = Some(file);
                    }
                }

                let input = input.expect(&format!("{} not present in {}", INPUT_FILE, working_dir));
                let output =
                    output.expect(&format!("{} not present in {}", OUTPUT_FILE, working_dir));

                let input_content = fs::read_to_string(input.path())
                    .expect(&format!("unable to read {} in {}", INPUT_FILE, working_dir));
                let output_content = fs::read_to_string(output.path()).expect(&format!(
                    "unable to read {} in {}",
                    OUTPUT_FILE, working_dir
                ));

                let actual_output_content = CmdRun::run_on_content(&input_content, &working_dir)
                    .expect("unable to execute cmdrun");

                assert_eq!(output_content, actual_output_content);
            }
        }
    };
}

#[test]
fn check_all_regressions_dirs() {
    let mut entries: Vec<String> = fs::read_dir("./tests/regression")
        .unwrap()
        .map(|r| r.unwrap())
        .map(|de| String::from(de.file_name().to_string_lossy()))
        .collect();

    entries.sort();

    // If you update this, we have to update the list add_dir! below
    assert_eq!(
        entries,
        vec![
            "bash_call",
            "inline_call",
            "py_factorial",
            "py_fibonacci",
            "py_readme",
            "rust_call",
            "shell",
        ]
    );
}

add_dir!(bash_call);
add_dir!(inline_call);
add_dir!(py_readme);
add_dir!(py_factorial);
add_dir!(py_fibonacci);
add_dir!(rust_call);
add_dir!(shell);
