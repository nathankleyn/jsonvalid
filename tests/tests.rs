extern crate assert_cli;
extern crate tempfile;

use tempfile::NamedTempFile;

use std::io::Write;

#[test]
fn test_stdin_invalid_json() {
    assert_cli::Assert::main_binary()
        .stdin("{ \"foo\": \"bar\"")
        .fails()
        .and()
        .stderr().contains("EOF while parsing an object at line 1 column 14")
        .unwrap();
}

#[test]
fn test_stdin_valid_json() {
    assert_cli::Assert::main_binary()
        .stdin("{ \"foo\": \"bar\" }")
        .succeeds()
        .unwrap();
}

#[test]
fn test_stdin_valid_with_unicode_bom() {
    assert_cli::Assert::main_binary()
        .stdin("\u{FEFF}{ \"foo\": \"bar\" }")
        .succeeds()
        .unwrap();
}

#[test]
fn test_files_single_invalid() {
    let mut file: NamedTempFile = NamedTempFile::new().unwrap();
    write!(file, "{{ \"foo\": \"bar\"").unwrap();

    assert_cli::Assert::main_binary()
        .with_args(&[file.path().to_str().unwrap()])
        .fails()
        .and()
        .stderr().contains("EOF while parsing an object at line 1 column 14")
        .unwrap();
}

#[test]
fn test_files_single_valid() {
    let mut file: NamedTempFile = NamedTempFile::new().unwrap();
    write!(file, "{{ \"foo\": \"bar\" }}").unwrap();

    assert_cli::Assert::main_binary()
        .with_args(&[file.path().to_str().unwrap()])
        .succeeds()
        .unwrap();
}

#[test]
fn test_files_single_valid_with_unicode_bom() {
    let mut file: NamedTempFile = NamedTempFile::new().unwrap();
    write!(file, "\u{FEFF}{{ \"foo\": \"bar\" }}").unwrap();

    assert_cli::Assert::main_binary()
        .with_args(&[file.path().to_str().unwrap()])
        .succeeds()
        .unwrap();
}

#[test]
fn test_files_multiple_invalid() {
    let mut file1: NamedTempFile = NamedTempFile::new().unwrap();
    write!(file1, "{{ \"foo\": \"bar\"").unwrap();
    let file1_path = file1.path().to_str().unwrap();

    let mut file2: NamedTempFile = NamedTempFile::new().unwrap();
    write!(file2, "{{ \"lux\": BLAH }}").unwrap();
    let file2_path = file2.path().to_str().unwrap();

    assert_cli::Assert::main_binary()
        .with_args(&[file1_path, file2_path])
        .fails()
        .and()
        .stderr().contains(format!("Error in file {}: EOF while parsing an object at line 1 column 14", file1_path))
        .and()
        .stderr().contains(format!("Error in file {}: expected value at line 1 column 10", file2_path))
        .unwrap();
}

#[test]
fn test_files_multiple_mixed() {
    let mut file1: NamedTempFile = NamedTempFile::new().unwrap();
    write!(file1, "{{ \"foo\": \"bar\" }}").unwrap();
    let file1_path = file1.path().to_str().unwrap();

    let mut file2: NamedTempFile = NamedTempFile::new().unwrap();
    write!(file2, "{{ \"lux\": BLAH }}").unwrap();
    let file2_path = file2.path().to_str().unwrap();

    assert_cli::Assert::main_binary()
        .with_args(&[file1_path, file2_path])
        .fails()
        .and()
        .stderr().contains(format!("Error in file {}: expected value at line 1 column 10", file2_path))
        .unwrap();
}

#[test]
fn test_files_multiple_valid() {
    let mut file1: NamedTempFile = NamedTempFile::new().unwrap();
    write!(file1, "{{ \"foo\": \"bar\" }}").unwrap();
    let file1_path = file1.path().to_str().unwrap();

    let mut file2: NamedTempFile = NamedTempFile::new().unwrap();
    write!(file2, "{{ \"lux\": true }}").unwrap();
    let file2_path = file2.path().to_str().unwrap();

    assert_cli::Assert::main_binary()
        .with_args(&[file1_path, file2_path])
        .succeeds()
        .unwrap();
}
