use std::process::Command;

#[test]
fn agd_version_flag_prints_package_version() {
    let out = Command::new(env!("CARGO_BIN_EXE_agd"))
        .arg("--version")
        .output()
        .expect("spawn agd --version");

    assert!(
        out.status.success(),
        "agd --version exited non-zero: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    let expected = env!("CARGO_PKG_VERSION");
    assert!(
        stdout.contains(expected),
        "expected stdout to contain version '{expected}', got: {stdout}"
    );
    assert!(
        stdout.contains("agd"),
        "expected stdout to contain 'agd', got: {stdout}"
    );
}
