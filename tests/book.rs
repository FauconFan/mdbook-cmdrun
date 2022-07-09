use std::process::Command;

#[test]
fn build_test_book() {
    let ls = Command::new("ls").arg("-lR").output().unwrap();
    println!("{:?}", String::from_utf8_lossy(&ls.stdout).to_string());

    let output = Command::new("mdbook")
        .arg("build")
        .current_dir("./tests/book/")
        .output()
        .unwrap();

    assert_eq!(output.status.code().unwrap(), 0);
}
