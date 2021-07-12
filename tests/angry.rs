use {
    assert_cmd::Command, predicate::str::contains, predicates::prelude::*, std::io::Write,
    tempfile::NamedTempFile,
};

#[test]
fn convert_content_arg() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("angry")?;
    cmd.arg("a little bit of text");
    cmd.assert()
        .success()
        .stdout(contains("A LITTLE BIT OF TEXT"));

    Ok(())
}

#[test]
fn convert_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "this is some text")?;

    let mut cmd = Command::cargo_bin("angry")?;
    cmd.arg("-i").arg(file.path());
    cmd.assert().success().stdout(contains("THIS IS SOME TEXT"));

    Ok(())
}

#[test]
fn convert_content_in_file_long() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "this is some text")?;

    let mut cmd = Command::cargo_bin("angry")?;
    cmd.arg("--input").arg(file.path());
    cmd.assert().success().stdout(contains("THIS IS SOME TEXT"));

    Ok(())
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("angry")?;
    cmd.args(["-i", "test/file/doesnt/exist"]);
    cmd.assert()
        .failure()
        .stderr(contains("No such file or directory"));

    Ok(())
}

#[test]
fn no_content_supplied() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("angry")?;
    cmd.assert().failure().stderr(contains("Missing content"));

    Ok(())
}
