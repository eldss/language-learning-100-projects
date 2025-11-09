use std::process::{Command, ExitStatus};

fn run_command(args: &[&str]) -> (ExitStatus, String, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_rust-convert"))
        .args(args)
        .output()
        .expect("command should execute successfully");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    (output.status, stdout, stderr)
}

#[test]
fn converts_from_celsius_with_short_flag() {
    let (status, stdout, _stderr) = run_command(&["-c", "0"]);

    assert!(status.success());
    assert!(stdout.contains("32.00"));
}

#[test]
fn converts_from_celsius_with_long_flag() {
    let (status, stdout, _stderr) = run_command(&["--celsius", "0"]);

    assert!(status.success());
    assert!(stdout.contains("32.00"));
}

#[test]
fn converts_from_fahrenheit_with_short_flag() {
    let (status, stdout, _stderr) = run_command(&["-f", "32"]);

    assert!(status.success());
    assert!(stdout.contains("0.00"));
}

#[test]
fn converts_from_fahrenheit_with_long_flag() {
    let (status, stdout, _stderr) = run_command(&["--fahrenheit", "32"]);

    assert!(status.success());
    assert!(stdout.contains("0.00"));
}

#[test]
fn fails_when_no_args_are_given() {
    let (status, _stdout, stderr) = run_command(&[]);

    assert!(!status.success());
    assert!(stderr.contains("Incorrect usage"));
}

#[test]
fn fails_when_too_many_arguments_are_passed() {
    let (status, _stdout, stderr) = run_command(&["-c", "0", "another"]);

    assert!(!status.success());
    assert!(stderr.contains("Incorrect usage"));
}

#[test]
fn fails_when_given_unknown_argument() {
    let (status, _stdout, stderr) = run_command(&["-huhh??", "42"]);

    assert!(!status.success());
    assert!(stderr.contains("Unexpected argument"));
}

#[test]
fn fails_when_given_a_non_f64_value() {
    let (status, _stdout, stderr) = run_command(&["-c", "zero"]);

    assert!(!status.success());
    assert!(stderr.contains("Problem parsing"));
}
