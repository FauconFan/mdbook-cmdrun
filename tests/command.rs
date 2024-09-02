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

cfg_if! {
    if #[cfg(target_family = "unix")] {
        add_test!(simple_inline1, "echo oui", "oui", true);
        add_test!(simple_inline2, "echo oui non", "oui non", true);
        add_test!(simple_inline3, "echo oui       non", "oui non", true);
        add_test!(simple_inline4, "echo oui; echo non", "oui\nnon", true);
        add_test!(simple_inline5, "echo \"hello world\"", "hello world", true);

        add_test!(simple1, "echo oui", "oui\n", false);
        add_test!(simple2, "echo oui non", "oui non\n", false);
        add_test!(simple3, "echo oui       non", "oui non\n", false);
        add_test!(simple4, "echo oui; echo non", "oui\nnon\n", false);
        add_test!(simple5, "echo \"hello world\"", "hello world\n", false);

        add_test!(pipe_inline1, "cat LICENSE | head -n 1", "MIT License", true);
        add_test!(pipe_inline2, "yes 42 | head -n 3", "42\n42\n42", true);
        add_test!(pipe_inline3, "echo \" coucou   \" | tr -d ' '", "coucou", true);

        add_test!(pipe1, "cat LICENSE | head -n 1", "MIT License\n", false);
        add_test!(pipe2, "yes 42 | head -n 3", "42\n42\n42\n", false);
        add_test!(pipe3, "echo \" coucou   \" | tr -d ' '", "coucou\n", false);

        add_test!(quote_inline1, "echo \"\"", "", true);
        add_test!(quote_inline2, "echo \"\\\"\"", "\"", true);
        add_test!(quote_inline3, "echo ''", "", true);
        add_test!(quote_inline4, "echo '\\'", "\\", true);

        add_test!(quote1, "echo \"\"", "\n", false);
        add_test!(quote2, "echo \"\\\"\"", "\"\n", false);
        add_test!(quote3, "echo ''", "\n", false);
        add_test!(quote4, "echo '\\'", "\\\n", false);

        #[test]
        fn fail_inline() {
            assert!(CmdRun::run_cmdrun("-1 exit 1".to_string(), ".", true).is_err())
        }

        #[test]
        fn fail() {
            assert!(CmdRun::run_cmdrun("-1 exit 1".to_string(), ".", false).is_err())
        }

        add_test!(
            mixed_inline1,
            "yes 42 | head -n 4 | sed -z 's/\\n/  \\n/g'",
            "42  \n42  \n42  \n42", true
            );

        add_test!(
            mixed1,
            "yes 42 | head -n 4 | sed -z 's/\\n/  \\n/g'",
            "42  \n42  \n42  \n42  \n", false
            );
    } else if #[cfg(target_family = "windows")] {
        add_test!(simple_inline1, "echo oui", "oui", true);
        add_test!(simple_inline2, "echo oui non", "oui non", true);
        add_test!(simple_inline3, "echo oui       non", "oui       non", true);
        add_test!(simple_inline4, "echo oui& echo non", "oui\r\nnon", true);
        add_test!(simple_inline5, "echo hello world", "hello world", true);

        add_test!(simple1, "echo oui", "oui\r\n", false);
        add_test!(simple2, "echo oui non", "oui non\r\n", false);
        add_test!(simple3, "echo oui       non", "oui       non\r\n", false);
        add_test!(simple4, "echo oui& echo non", "oui\r\nnon\r\n", false);
        add_test!(simple5, "echo hello world", "hello world\r\n", false);

        add_test!(pipe_inline1, "cat LICENSE | head -n 1", "MIT License", true);
        add_test!(pipe_inline2, "yes 42 | head -n 3", "42\r\n42\r\n42", true);
        add_test!(pipe_inline3, "echo  coucou    | tr -d ' '", "coucou", true);

        add_test!(pipe1, "cat LICENSE | head -n 1", "MIT License\r\n", false);
        add_test!(pipe2, "yes 42 | head -n 3", "42\r\n42\r\n42\r\n", false);
        add_test!(pipe3, "echo  coucou    | tr -d ' '", "coucou\r\n", false);

        add_test!(
            mixed_inline1,
            "yes 42 | head -n 4 | sed -z 's/\\n/  \\n/g'",
            "42  \r\n42  \r\n42  \r\n42", true
        );

        add_test!(
            mixed1,
            "yes 42 | head -n 4 | sed -z 's/\\n/  \\n/g'",
            "42  \r\n42  \r\n42  \r\n42  \r\n", false
            );

    }
}
