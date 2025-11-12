use std::process::Command;

fn run_cli(args: &[&str]) -> std::process::Output {
    let exe = env!("CARGO_BIN_EXE_bmi-calc");
    Command::new(exe)
        .args(args)
        .output()
        .expect("bmi-calc binary should run successfully")
}

#[test]
fn metric_arguments_print_expected_bmi() {
    let output = run_cli(&["1.80", "80"]);

    assert!(
        output.status.success(),
        "binary exited with failure: {:?}",
        output
    );
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert_eq!(stdout.trim(), "BMI: 24.69");
}

#[test]
fn imperial_arguments_print_expected_bmi() {
    let output = run_cli(&["--imperial", "70", "160"]);

    assert!(
        output.status.success(),
        "binary exited with failure: {:?}",
        output
    );
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert_eq!(stdout.trim(), "BMI: 22.96");
}

#[test]
fn zero_height_exits_with_failure_and_message() {
    let output = run_cli(&["0", "70"]);

    assert!(
        !output.status.success(),
        "expected failure but exited successfully"
    );
    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf8");
    assert!(
        stderr.contains("Height must be a positive number."),
        "stderr was: {stderr}"
    );
}
