use std::fs;
use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_integration_audit_fails() {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();

    // Create suites directory
    let suites_dir = root.join("suites");
    fs::create_dir(&suites_dir).unwrap();
    fs::write(
        suites_dir.join("hope_primitives_basic.json"),
        r#"{
            "suite_id": "basic",
            "description": "Basic requirements",
            "policies": [
                {
                    "id": "must_be_implemented",
                    "path": "implementation_status",
                    "operator": "eq",
                    "expected": "implemented"
                }
            ]
        }"#,
    )
    .unwrap();

    // Create potentials directory with a bad primitive
    let potentials_dir = root.join("potentials");
    fs::create_dir(&potentials_dir).unwrap();
    fs::write(
        potentials_dir.join("primitive_bad.json"),
        r#"{
            "id": "primitive_bad",
            "implementation_status": "planned"
        }"#,
    )
    .unwrap();

    // Run bineval audit
    let output = Command::new(env!("CARGO_BIN_EXE_bineval"))
        .arg("audit")
        .arg("--path")
        .arg(root)
        .arg("--targets")
        .arg(&potentials_dir)
        .output()
        .unwrap();

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Policy 'must_be_implemented' failed for primitive 'primitive_bad'"));
}

#[test]
fn test_integration_audit_passes() {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();

    // Create suites directory
    let suites_dir = root.join("suites");
    fs::create_dir(&suites_dir).unwrap();
    fs::write(
        suites_dir.join("hope_primitives_basic.json"),
        r#"{
            "suite_id": "basic",
            "policies": [
                {
                    "id": "must_be_implemented",
                    "path": "implementation_status",
                    "operator": "eq",
                    "expected": "implemented"
                }
            ]
        }"#,
    )
    .unwrap();

    // Create potentials directory with a good primitive
    let potentials_dir = root.join("potentials");
    fs::create_dir(&potentials_dir).unwrap();
    fs::write(
        potentials_dir.join("primitive_good.json"),
        r#"{
            "id": "primitive_good",
            "implementation_status": "implemented"
        }"#,
    )
    .unwrap();

    // Run bineval audit
    let output = Command::new(env!("CARGO_BIN_EXE_bineval"))
        .arg("audit")
        .arg("--path")
        .arg(root)
        .arg("--targets")
        .arg(&potentials_dir)
        .output()
        .unwrap();

    assert!(output.status.success());
    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("AUDIT PASSED"));
}

#[test]
fn test_integration_numeric_subset_passes() {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();

    let suites_dir = root.join("suites");
    fs::create_dir(&suites_dir).unwrap();
    fs::write(
        suites_dir.join("num_ref.json"),
        r#"{
            "suite_id": "num_ref",
            "policies": [
                {
                    "id": "valid_subset_basic",
                    "path": "agent_output",
                    "operator": "numeric_subset",
                    "expected": [120, 45.5, 2026]
                }
            ]
        }"#,
    )
    .unwrap();

    let potentials_dir = root.join("potentials");
    fs::create_dir(&potentials_dir).unwrap();
    fs::write(
        potentials_dir.join("prim_valid.json"),
        r#"{
            "id": "prim_valid",
            "agent_output": "Revenue was 120 with margin 45.5 in 2026."
        }"#,
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_bineval"))
        .arg("run")
        .arg("--suite")
        .arg("num_ref")
        .arg("--path")
        .arg(root)
        .arg("--targets")
        .arg(&potentials_dir)
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "expected exit code 0 for valid subset case"
    );
}

#[test]
fn test_integration_numeric_subset_fails() {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();

    let suites_dir = root.join("suites");
    fs::create_dir(&suites_dir).unwrap();
    fs::write(
        suites_dir.join("num_ref.json"),
        r#"{
            "suite_id": "num_ref",
            "policies": [
                {
                    "id": "hallucinated_number_present",
                    "path": "agent_output",
                    "operator": "numeric_subset",
                    "expected": [120, 45.5]
                }
            ]
        }"#,
    )
    .unwrap();

    let potentials_dir = root.join("potentials");
    fs::create_dir(&potentials_dir).unwrap();
    fs::write(
        potentials_dir.join("prim_invalid.json"),
        r#"{
            "id": "prim_invalid",
            "agent_output": "Revenue was 999 last quarter."
        }"#,
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_bineval"))
        .arg("run")
        .arg("--suite")
        .arg("num_ref")
        .arg("--path")
        .arg(root)
        .arg("--targets")
        .arg(&potentials_dir)
        .output()
        .unwrap();

    assert!(
        !output.status.success(),
        "expected non-zero exit code for hallucinated number"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("999"),
        "error message should reference the hallucinated number: {}",
        stderr
    );
}
