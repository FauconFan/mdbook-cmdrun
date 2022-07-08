use std::fs;

use mdbook_cmdrun::CmdRun;

#[test]
fn regression() {
    let paths = fs::read_dir("./tests/regression").expect("expect to read the current directory");

    for path in paths {
        let direntry = path.unwrap();
        let mut input = None;
        let mut output = None;

        for file in fs::read_dir(direntry.path()).expect(&format!(
            "expect to read the current direcory {:?}",
            direntry
        )) {
            let file = file.unwrap();

            if file.file_name().as_os_str() == "input.md" {
                input = Some(file);
            } else if file.file_name().as_os_str() == "output.md" {
                output = Some(file);
            }
        }

        let input = input.expect(&format!("input.md not present in {:?}", direntry));
        let output = output.expect(&format!("output.md not present in {:?}", direntry));

        let working_dir = String::from(direntry.path().as_path().to_string_lossy());

        let input_content = fs::read_to_string(input.path())
            .expect(&format!("unable to read input.md in {:?}", direntry));
        let output_content = fs::read_to_string(output.path())
            .expect(&format!("unable to read output.md in {:?}", direntry));

        let actual_output_content =
            CmdRun::run_on_content(&input_content, &working_dir).expect("unable to execute cmdrun");

        assert_eq!(output_content, actual_output_content);
    }
}
