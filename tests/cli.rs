use std::process::Command;

fn mimic_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_mimic"))
}

#[test]
fn lint_exits_zero() {
    let output = mimic_bin().args(["lint"]).output().unwrap();
    assert!(
        output.status.success(),
        "lint should exit 0: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn lint_with_warnings_exits_zero() {
    let output = mimic_bin().args(["lint", "--warnings"]).output().unwrap();
    assert!(
        output.status.success(),
        "lint --warnings should exit 0: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn lint_bad_fragments_exits_nonzero() {
    let tmp = tempfile::tempdir().unwrap();
    let mimic_dir = tmp.path().join(".mimic");
    let personas_dir = mimic_dir.join("personas");
    std::fs::create_dir_all(&personas_dir).unwrap();
    // Fragment with empty body (error)
    std::fs::write(
        personas_dir.join("bad.md"),
        "---\ndescription: Bad persona\ntags: [test]\n---\n",
    )
    .unwrap();

    let output = mimic_bin()
        .args(["lint"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    assert!(
        !output.status.success(),
        "lint should exit nonzero for bad fragments: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn help_flag() {
    let output = mimic_bin().args(["--help"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Usage"),
        "help should contain Usage: {stdout}"
    );
}

#[test]
fn version_flag() {
    let output = mimic_bin().args(["--version"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("mimic"),
        "version should contain 'mimic': {stdout}"
    );
}
