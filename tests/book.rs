use std::fs;
use std::process::Command;

#[test]
fn build_test_book() {
    let output = Command::new("mdbook")
        .arg("build")
        .current_dir(fs::canonicalize("./tests/book/").unwrap())
        .output()
        .unwrap();

    let status = output.status.code().unwrap();
    if status != 0 {
        panic!(
            "mdbook build failed: {}",
            String::from_utf8(output.stderr).unwrap()
        );
    }
}
