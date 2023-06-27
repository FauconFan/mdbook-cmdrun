use cfg_if::cfg_if;
use mdbook_cmdrun::CmdRun;

macro_rules! add_test {
    ($name:ident, $cmd:literal, $output:literal, $val:expr $(,)?) => {
        #[test]
        fn $name() {
            let actual_output = CmdRun::run_cmdrun($cmd.to_string(), ".", $val).unwrap();

            assert_eq!(actual_output, $output);
        }
    };
}
add_test!(simple1, "echo oui", "oui", true);
add_test!(simple2, "echo oui non", "oui non", true);
add_test!(pipe1, "cat LICENSE | head -n 1", "MIT License", true);

cfg_if! {
    if #[cfg(any(target_family = "unix", target_family = "other"))] {
        add_test!(simple3, "echo oui       non", "oui non", true);
        add_test!(simple4, "echo oui; echo non", "oui\nnon", true);
        add_test!(simple5, "echo \"hello world\"", "hello world", true);
        add_test!(pipe2, "yes 42 | head -n 3", "42\n42\n42", true);
        add_test!(pipe3, "echo \" coucou   \" | tr -d ' '", "coucou", true);
        add_test!(quote1, "echo \"\"", "", true);
        add_test!(quote2, "echo \"\\\"\"", "\"", true);
        add_test!(quote3, "echo ''", "", true);
        add_test!(quote4, "echo '\\'", "\\", true);
        add_test!(
            mixed1,
            "yes 42 | head -n 4 | sed -z 's/\\n/  \\n/g'",
            "42  \n42  \n42  \n42", true
        );
    } else if #[cfg(target_family = "windows")] {
        add_test!(simple3, "echo oui       non", "oui       non", true);
        add_test!(simple4, "echo oui& echo non", "oui\r\nnon", true);
        add_test!(simple5, "echo hello world", "hello world", true);
        add_test!(pipe2, "yes 42 | head -n 3", "42\r\n42\r\n42", true);
        add_test!(pipe3, "echo  coucou    | tr -d ' '", "coucou", true);
        add_test!(
            mixed1,
            "yes 42 | head -n 4 | sed -z 's/\\n/  \\n/g'",
            "42  \r\n42  \r\n42  \r\n42", true
        );
    }
}
