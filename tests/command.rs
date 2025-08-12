use cfg_if::cfg_if;
use mdbook_cmdrun::CmdRun;

// Tests might differ on windows and unix because newlines, spaces and escaping work differently
cfg_if! {
    if #[cfg(target_family = "unix")] {
        const NL: &str = "\n";
    } else if #[cfg(target_family = "windows")] {
        const NL: &str = "\r\n";
    }
}

macro_rules! add_test {
    ($name:ident, $cmd:literal, $output:expr, $inline:expr $(,)?) => {
        #[test]
        fn $name() {
            let actual_output = CmdRun::run_cmdrun($cmd.to_string(), ".", $inline).unwrap();

            assert_eq!(actual_output, $output.to_string());
        }
    };
}

add_test!(simple1, "echo oui", &format!("oui{NL}"), false);
add_test!(simple_inline1, "echo oui", "oui", true);

add_test!(simple2, "echo oui non", &format!("oui non{NL}"), false);
add_test!(simple_inline2, "echo oui non", "oui non", true);

cfg_if! {
    if #[cfg(target_family = "unix")] {
        add_test!(simple3, "echo oui       non", &format!("oui non{NL}"), false);
        add_test!(simple_inline3, "echo oui       non", "oui non", true);
    } else if #[cfg(target_family = "windows")] {
        add_test!(simple3, "echo oui       non", &format!("oui       non{NL}"), false);
        add_test!(simple_inline3, "echo oui       non", "oui       non", true);
    }
}

cfg_if! {
    if #[cfg(target_family = "unix")] {
        add_test!(simple4, "echo oui; echo non", &format!("oui{NL}non{NL}"), false);
        add_test!(simple_inline4, "echo oui; echo non", "oui\nnon", true);
    } else if #[cfg(target_family = "windows")] {
        // on windows, ; is not a command separator, so it will be passed to the command
        add_test!(simple4, "echo oui; echo non", &format!("oui; echo non{NL}"), false);
        add_test!(simple_inline4, "echo oui; echo non", "oui; echo non", true);
    }
}

cfg_if! {
    if #[cfg(target_family = "unix")] {
        add_test!(simple5, "echo \"hello world\"", &format!("hello world{NL}"), false);
        add_test!(simple_inline5, "echo \"hello world\"", "hello world", true);
    } else if #[cfg(target_family = "windows")] {
        add_test!(simple5, "echo hello world", &format!("hello world{NL}"), false);
        add_test!(simple_inline5, "echo hello world", "hello world", true);
    }
}

add_test!(
    pipe1,
    "cat LICENSE | head -n 1",
    &format!("MIT License{NL}"),
    false
);
add_test!(pipe_inline1, "cat LICENSE | head -n 1", "MIT License", true);
add_test!(
    pipe2,
    "yes 42 | head -n 3",
    &format!("42{NL}42{NL}42{NL}"),
    false
);
add_test!(
    pipe_inline2,
    "yes 42 | head -n 3",
    &format!("42{NL}42{NL}42"),
    true
);

cfg_if! {
    if #[cfg(target_family = "unix")] {
        add_test!(
            pipe3,
            "echo \" coucou   \" | tr -d ' '",
            &format!("coucou{NL}"),
            false
        );
        add_test!(
            pipe_inline3,
            "echo \" coucou   \" | tr -d ' '",
            "coucou",
            true
        );
    } else if #[cfg(target_family = "windows")] {
        add_test!(
            pipe3,
            "echo coucou   | tr -d ' '",
            &format!("coucou{NL}"),
            false
        );
        add_test!(
            pipe_inline3,
            "echo coucou   | tr -d ' '",
            "coucou",
            true
        );
    }
}

cfg_if!(
    if #[cfg(target_family = "unix")] {
        add_test!(quote_inline1, "echo \"\"", "", true);
        add_test!(quote_inline2, "echo \"\\\"\"", "\"", true);
        add_test!(quote_inline3, "echo ''", "", true);
        add_test!(quote_inline4, "echo '\\'", "\\", true);

        add_test!(quote1, "echo \"\"", &format!("{NL}"), false);
        add_test!(quote2, "echo \"\\\"\"", &format!("\"{NL}"), false);
        add_test!(quote3, "echo ''", &format!("{NL}"), false);
        add_test!(quote4, "echo '\\'", &format!("\\{NL}"), false);
    }
);

add_test!(
    mixed_inline1,
    "yes 42 | head -n 4 | sed -z 's/\\n/  \\n/g'",
    &format!("42  {NL}42  {NL}42  {NL}42"),
    true
);

add_test!(
    mixed1,
    "yes 42 | head -n 4 | sed -z 's/\\n/  \\n/g'",
    &format!("42  {NL}42  {NL}42  {NL}42  {NL}"),
    false
);

add_test!(pass_without_exit_code_spec, "exit 1", "", false);
add_test!(short_match_fail_exit_code, "-1 exit 1", "", false);
add_test!(short_match_pass_exit_code, "-0 exit 0", "", false);
add_test!(
    short_exit_code_mismatch,
    "-0 exit 1",
    &format!("**cmdrun error**: 'exit 1' returned exit code 1 instead of 0.{NL}{NL}"),
    false
);
add_test!(
    long_match_fail_exit_code,
    "--expect-return-code 1 exit 1",
    "",
    false
);
add_test!(
    long_match_pass_exit_code1,
    "--expect-return-code 0 exit 0",
    "",
    false
);
add_test!(long_match_pass_exit_code2, "--strict exit 0", "", false);
add_test!(
    long_exit_code_mismatch1,
    "--expect-return-code 0 exit 1",
    &format!("**cmdrun error**: 'exit 1' returned exit code 1 instead of 0.{NL}{NL}"),
    false
);
add_test!(
    long_exit_code_mismatch2,
    "--strict exit 1",
    &format!("**cmdrun error**: 'exit 1' returned exit code 1 instead of 0.{NL}{NL}"),
    false
);
add_test!(
    not_a_cmdrun_flag,
    "--flag-dne echo hello world",
    &format!("**cmdrun error**: Unrecognized cmdrun flag --flag-dne in 'cmdrun --flag-dne echo hello world'"),
    false
);
add_test!(
    shortform_typo,
    "--0 echo hello world",
    "**cmdrun error**: Unrecognized cmdrun flag --0 in 'cmdrun --0 echo hello world'",
    false
);
add_test!(missing_arg_no_cmd, "--expect-return-code",
          "**cmdrun error**: No return code after '--expect-return-code' in 'cmdrun --expect-return-code'",
          false,);
add_test!(missing_arg_no_code, "--expect-return-code echo hello world",
          "**cmdrun error**: No return code after '--expect-return-code' in 'cmdrun --expect-return-code echo hello world'",
          false);
add_test!(bad_short_form_exit_code, "-NaN echo hello world",
          "**cmdrun error**: Unable to interpret short-form exit code -NaN as a number in 'cmdrun -NaN echo hello world'",
          false);
