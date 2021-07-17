use {
    assert_cmd::Command,
    predicate::str,
    predicates::prelude::*,
    std::io::{Read, Seek, SeekFrom, Write},
    tempfile::NamedTempFile,
};

#[test]
fn unspecified_content_arg() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("angry")?;
    cmd.arg("a little bit OF TEXT")
        .assert()
        .failure()
        .code(1)
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
        .code(0)
        .stdout(str::contains("A LITTLE BIT OF TEXT"));

    Ok(())
}

#[test]
fn uppercase_content_arg_to_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;

    let mut cmd = Command::cargo_bin("angry")?;
    cmd.arg("-u")
        .arg("a little bit OF TEXT")
        .arg("-o")
        .arg(file.path())
        .assert()
        .success()
        .stdout("");

    // reset the file to the start
    file.seek(SeekFrom::Start(0))?;

    // read and asset on the file contents
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    assert_eq!("A LITTLE BIT OF TEXT", buf);

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
        .write_stdin("this is some text")
        .assert()
        .success()
        .stdout(str::contains("THIS IS SOME TEXT"));

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
        .stdout(predicate::eq("A LITTLE BIT of text\n"));

    Ok(())
}

#[test]
fn random_content_arg() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("angry")?;
    cmd.arg("-a")
        .arg("a little bit OF TEXT")
        .assert()
        .success()
        .stdout(predicate::function(|s: &str| s.len() == 21));

    Ok(())
}
