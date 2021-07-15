use {
    assert_cmd::Command, predicate::str, predicates::prelude::*, std::io::Write,
    tempfile::NamedTempFile,
};

#[test]
fn unspecified_content_arg() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("angry")?;
    cmd.arg("a little bit OF TEXT")
        .assert()
        .failure()
        .stderr(str::contains(
            "The following required arguments were not provided:",
        ));

    Ok(())
}

#[test]
fn uppercase_content_arg() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("angry")?;
    cmd.arg("-u")
        .arg("a little bit OF TEXT")
        .assert()
        .success()
        .stdout(str::contains("A LITTLE BIT OF TEXT"));

    Ok(())
}

#[test]
fn uppercase_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "this is some TEXT")?;

    let mut cmd = Command::cargo_bin("angry")?;
    cmd.arg("-u")
        .arg("-i")
        .arg(file.path())
        .assert()
        .success()
        .stdout(str::contains("THIS IS SOME TEXT"));

    Ok(())
}

#[test]
fn uppercase_content_in_file_long() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "this is some text")?;

    let mut cmd = Command::cargo_bin("angry")?;
    cmd.arg("-u")
        .arg("--input")
        .arg(file.path())
        .assert()
        .success()
        .stdout(str::contains("THIS IS SOME TEXT"));

    Ok(())
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("angry")?;
    cmd.arg("-u")
        .args(["-i", "test/file/doesnt/exist"])
        .assert()
        .failure()
        .stderr(str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn no_content_supplied() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("angry")?;
    cmd.arg("-u")
        .assert()
        .failure()
        .stderr(str::contains("Missing content"));

    Ok(())
}

#[test]
fn lowercase_content_arg() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("angry")?;
    cmd.arg("-l")
        .arg("a little bit OF TEXT")
        .assert()
        .success()
        .stdout(str::contains("a little bit of text"));

    Ok(())
}

#[test]
fn reverse_content_arg() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("angry")?;
    cmd.arg("-r")
        .arg("a little bit OF TEXT")
        .assert()
        .success()
        .stdout(str::contains("A LITTLE BIT of text"));

    Ok(())
}

#[test]
fn random_content_arg() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("angry")?;
    cmd.arg("-a")
        .arg("a little bit OF TEXT")
        .assert()
        .success()
        .stdout(predicate::eq("a little bit of text").not());

    Ok(())
}
