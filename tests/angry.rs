use {
    assert_cmd::Command, predicate::str::contains, predicates::prelude::*, std::io::Write,
    tempfile::NamedTempFile,
};

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("angry")?;
    cmd.arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(contains("No such file or directory"));

    Ok(())
}

#[test]
fn convert_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "this is some text")?;

    let mut cmd = Command::cargo_bin("angry")?;
    cmd.arg(file.path());
    cmd.assert().success().stdout(contains("THIS IS SOME TEXT"));

    Ok(())
}
