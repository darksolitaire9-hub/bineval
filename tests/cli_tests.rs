use assert_cmd::Command;

#[test]
fn test_validate_suites_success() {
    let mut cmd = Command::cargo_bin("bineval").expect("Failed to get binary");
    // Right now, the json_config adapter loads nothing so it will simulate 0 templates/suites successfully
    cmd.arg("validate").arg("suites");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Validation successful"));
}

#[test]
fn test_validate_unknown_component() {
    let mut cmd = Command::cargo_bin("bineval").expect("Failed to get binary");
    cmd.arg("validate").arg("unknown_thing");
    cmd.assert().failure().stderr(predicates::str::contains(
        "Unknown validation component: unknown_thing",
    ));
}

#[test]
fn test_audit_stub() {
    let mut cmd = Command::cargo_bin("bineval").expect("Failed to get binary");
    cmd.arg("audit");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("AUDIT: OK"));
}

#[test]
fn test_run_suite_stub() {
    let mut cmd = Command::cargo_bin("bineval").expect("Failed to get binary");
    cmd.arg("run-suite").arg("safety_core");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Running suite: safety_core"));
}
