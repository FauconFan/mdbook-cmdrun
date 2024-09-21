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

        // the inline flag only affects the output and here I'm just checking exit codes
        // so I only test without the inline flag
        add_test!(pass_without_exit_code_spec, "exit 1", "", false);
        add_test!(short_match_fail_exit_code, "-1 exit 1", "", false);
        add_test!(short_match_pass_exit_code, "-0 exit 0", "", false);
        add_test!(short_exit_code_mismatch, "-0 exit 1",
                  "**cmdrun error**: 'exit 1' returned exit code 1 instead of 0.\n\n", false);
        add_test!(long_match_fail_exit_code, "--expect-return-code 1 exit 1", "", false);
        add_test!(long_match_pass_exit_code1, "--expect-return-code 0 exit 0", "", false);
        add_test!(long_match_pass_exit_code2, "--strict exit 0", "", false);
        add_test!(long_exit_code_mismatch1, "--expect-return-code 0 exit 1",
                  "**cmdrun error**: 'exit 1' returned exit code 1 instead of 0.\n\n", false);
        add_test!(long_exit_code_mismatch2, "--strict exit 1",
                  "**cmdrun error**: 'exit 1' returned exit code 1 instead of 0.\n\n", false);

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

        // the inline flag only affects the output and here I'm just checking exit codes
        // so I only test without the inline flag
        add_test!(pass_without_exit_code_spec, "exit 1", "", false);
        add_test!(short_match_fail_exit_code, "-1 exit 1", "", false);
        add_test!(short_match_pass_exit_code, "-0 exit 0", "", false);
        add_test!(short_exit_code_mismatch, "-0 exit 1",
                  "**cmdrun error**: 'exit 1' returned exit code 1 instead of 0.\n\n", false);
        add_test!(long_match_fail_exit_code, "--expect-return-code 1 exit 1", "", false);
        add_test!(long_match_pass_exit_code1, "--expect-return-code 0 exit 0", "", false);
        add_test!(long_match_pass_exit_code2, "--strict exit 0", "", false);
        add_test!(long_exit_code_mismatch1, "--expect-return-code 0 exit 1",
                  "**cmdrun error**: 'exit 1' returned exit code 1 instead of 0.\n\n", false);
        add_test!(long_exit_code_mismatch2, "--strict exit 1",
                  "**cmdrun error**: 'exit 1' returned exit code 1 instead of 0.\n\n", false);

    }
}
