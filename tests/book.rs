use std::process::Command;

#[test]
fn build_test_book() {
    let output = Command::new("mdbook")
        .arg("build")
        .current_dir("./tests/book")
        .output()
        .unwrap();

    assert_eq!(output.status.code().unwrap(), 0);
}
