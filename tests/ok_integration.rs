use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_ok_default_output() {
    let mut cmd = Command::cargo_bin("snphost").unwrap();
    cmd.arg("ok");

    // The command should run successfully (exit code 0 or specific error codes are acceptable)
    // We're primarily testing that it doesn't crash and produces output
    let output = cmd.output().unwrap();

    // Convert output to string for inspection
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check that we get some output (not empty)
    assert!(
        !stdout.is_empty(),
        "Expected non-empty output from 'snphost ok'"
    );

    // Check for expected test categories in the output
    // These are the main test groups that should appear
    assert!(
        stdout.contains("AMD CPU")
            || stdout.contains("PASS")
            || stdout.contains("FAIL")
            || stdout.contains("SKIP"),
        "Expected to see test status indicators (PASS/FAIL/SKIP) or test names in output"
    );
}

#[test]
fn test_ok_short_output() {
    let mut cmd = Command::cargo_bin("snphost").unwrap();
    cmd.arg("ok").arg("--short");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check that we get some output
    assert!(
        !stdout.is_empty(),
        "Expected non-empty output from 'snphost ok --short'"
    );

    // Short format should include summary information
    // Look for patterns like "Passed:", "Failed:", "Skipped:" or numeric summaries
    assert!(
        stdout.contains("Passed") || stdout.contains("Failed") || stdout.contains("Skipped"),
        "Expected summary output with Passed/Failed/Skipped counts in --short mode"
    );
}

#[test]
fn test_ok_help() {
    let mut cmd = Command::cargo_bin("snphost").unwrap();
    cmd.arg("ok").arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Show only failures"));
}
