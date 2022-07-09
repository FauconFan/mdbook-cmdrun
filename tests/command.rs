use mdbook_cmdrun::CmdRun;

macro_rules! add_test {
    ($name:ident, $cmd:literal, $output:literal) => {
        #[test]
        fn $name() {
            let actual_output = CmdRun::run_cmdrun($cmd.to_string(), ".").unwrap();

            assert_eq!(actual_output, $output);
        }
    };
}

add_test!(simple1, "echo oui", "oui\n");
add_test!(simple2, "echo oui non", "oui non\n");
add_test!(simple3, "echo oui       non", "oui non\n");
add_test!(simple4, "echo oui; echo non", "oui\nnon\n");
add_test!(simple5, "echo \"hello world\"", "hello world\n");
add_test!(pipe1, "cat LICENSE | head -n 1", "MIT License\n");
add_test!(pipe2, "yes 42 | head -n 3", "42\n42\n42\n");
add_test!(pipe3, "echo \" coucou   \" | tr -d ' '", "coucou\n");
add_test!(quote1, "echo \"\"", "\n");
add_test!(quote2, "echo \"\\\"\"", "\"\n");
add_test!(quote3, "echo ''", "\n");
add_test!(quote4, "echo '\\'", "\\\n");
add_test!(
    mixed1,
    "yes 42 | head -n 4 | sed -z 's/\\n/  \\n/g'",
    "42  \n42  \n42  \n42  \n"
);
