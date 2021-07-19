use {
    assert_cmd::Command,
    predicate::str,
    predicates::prelude::*,
    std::io::{Read, Seek, SeekFrom, Write},
    tempfile::NamedTempFile,
};

type CmdResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn unspecified_content_arg() -> CmdResult {
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
fn uppercase_content_arg() -> CmdResult {
    let mut cmd = Command::cargo_bin("angry")?;
    cmd.arg("--uppercase")
        .arg("a little bit OF TEXT")
        .assert()
        .success()
        .code(0)
        .stdout(str::contains("A LITTLE BIT OF TEXT"));

    Ok(())
}

#[test]
fn uppercase_content_arg_to_file() -> CmdResult {
    let mut file = NamedTempFile::new()?;

    let mut cmd = Command::cargo_bin("angry")?;
    cmd.arg("--uppercase")
        .arg("a little bit OF TEXT")
        .arg("--output")
        .arg(file.path())
        .assert()
        .success()
        .stdout("");

    // reset the file to the start
    file.seek(SeekFrom::Start(0))?;

    // read and asset on the file contents
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    assert_eq!("A LITTLE BIT OF TEXT\n", buf);

    Ok(())
}

#[test]
fn uppercase_content_in_file() -> CmdResult {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "this is some TEXT")?;

    let mut cmd = Command::cargo_bin("angry")?;
    cmd.arg("--uppercase")
        .arg("--input")
        .arg(file.path())
        .assert()
        .success()
        .stdout(str::contains("THIS IS SOME TEXT"));

    Ok(())
}

#[test]
fn uppercase_content_in_file_multiline() -> CmdResult {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "this is some text\nit covers at least\nthree lines!")?;

    let mut cmd = Command::cargo_bin("angry")?;
    cmd.arg("--uppercase")
        .arg("--input")
        .arg(file.path())
        .assert()
        .success()
        .stdout(str::contains(
            "THIS IS SOME TEXT\nIT COVERS AT LEAST\nTHREE LINES!",
        ));

    Ok(())
}

#[test]
fn uppercase_content_stdin() -> CmdResult {
    let mut cmd = Command::cargo_bin("angry")?;
    cmd.arg("--uppercase")
        .write_stdin("this is some text")
        .assert()
        .success()
        .stdout(str::contains("THIS IS SOME TEXT"));

    Ok(())
}

#[test]
fn file_doesnt_exist() -> CmdResult {
    let mut cmd = Command::cargo_bin("angry")?;
    cmd.arg("--uppercase")
        .args(["--input", "test/file/doesnt/exist"])
        .assert()
        .failure()
        .stderr(str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn lowercase_content_arg() -> CmdResult {
    let mut cmd = Command::cargo_bin("angry")?;
    cmd.arg("--lowercase")
        .arg("a little bit OF TEXT")
        .assert()
        .success()
        .stdout(str::contains("a little bit of text"));

    Ok(())
}

#[test]
fn reverse_content_arg() -> CmdResult {
    let mut cmd = Command::cargo_bin("angry")?;
    cmd.arg("--reverse")
        .arg("a little bit OF TEXT")
        .assert()
        .success()
        .stdout(predicate::eq("A LITTLE BIT of text\n"));

    Ok(())
}

#[test]
fn random_content_arg() -> CmdResult {
    let mut cmd = Command::cargo_bin("angry")?;
    cmd.arg("--random")
        .arg("a little bit OF TEXT")
        .assert()
        .success()
        .stdout(predicate::function(|s: &str| s.len() == 21));

    Ok(())
}
