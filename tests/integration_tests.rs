//! Integration tests for gith CLI

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Helper to create a test git repository
fn setup_test_repo() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Initialize git repository
    Command::new("git")
        .args(["init"])
        .current_dir(&temp_dir)
        .assert()
        .success();

    // Set git config
    Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(&temp_dir)
        .assert()
        .success();

    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&temp_dir)
        .assert()
        .success();

    temp_dir
}

#[test]
fn test_git_init_compatibility() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Test that gith init works like git init
    let mut cmd = Command::cargo_bin("gith").unwrap();
    cmd.arg("init").current_dir(&temp_dir).assert().success();

    // Check that .git directory was created (just like git init)
    assert!(temp_dir.path().join(".git").exists());
}

#[test]
fn test_init_with_gith_flag() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Test that gith init --gith works like git init but also initializes gith tracking
    let mut cmd = Command::cargo_bin("gith").unwrap();
    cmd.args(["init", "--gith"])
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Gith tracking initialized successfully!",
        ));

    // Check that both .git and .gith directories were created
    assert!(temp_dir.path().join(".git").exists());
    assert!(temp_dir.path().join(".gith").exists());
    assert!(temp_dir.path().join(".gith/human_manifest.json").exists());
}

#[test]
fn test_init_with_gith_flag_in_directory() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_dir = temp_dir.path().join("my-project");

    // Test that gith init --gith my-project works
    let mut cmd = Command::cargo_bin("gith").unwrap();
    cmd.args(["init", "--gith", "my-project"])
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Gith tracking initialized successfully!",
        ));

    // Check that both .git and .gith directories were created in the project directory
    assert!(project_dir.join(".git").exists());
    assert!(project_dir.join(".gith").exists());
    assert!(project_dir.join(".gith/human_manifest.json").exists());
}

#[test]
fn test_init_tracking() {
    let temp_dir = setup_test_repo();

    let mut cmd = Command::cargo_bin("gith").unwrap();
    cmd.arg("init-tracking")
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Gith tracking initialized successfully!",
        ));

    // Check that .gith directory was created
    assert!(temp_dir.path().join(".gith").exists());
    assert!(temp_dir.path().join(".gith/human_manifest.json").exists());
}

#[test]
fn test_gith_version() {
    let mut cmd = Command::cargo_bin("gith").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("gith"));
}

#[test]
fn test_gith_help() {
    let mut cmd = Command::cargo_bin("gith").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("A friendly Git wrapper"));
}

#[test]
fn test_human_commit() {
    let temp_dir = setup_test_repo();

    // Initialize gith tracking
    Command::cargo_bin("gith")
        .unwrap()
        .arg("init-tracking")
        .current_dir(&temp_dir)
        .assert()
        .success();

    // Create a test file
    fs::write(temp_dir.path().join("test.txt"), "Hello, human world!").unwrap();

    // Add the file
    Command::cargo_bin("gith")
        .unwrap()
        .args(["add", "test.txt"])
        .current_dir(&temp_dir)
        .assert()
        .success();

    // Create a human-flagged commit
    Command::cargo_bin("gith")
        .unwrap()
        .args(["commit", "--human", "-m", "Add test file"])
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Human-flagged commit created"));
}

#[test]
fn test_list_human_empty() {
    let temp_dir = setup_test_repo();

    // Initialize gith tracking
    Command::cargo_bin("gith")
        .unwrap()
        .arg("init-tracking")
        .current_dir(&temp_dir)
        .assert()
        .success();

    // List human content (should be empty)
    Command::cargo_bin("gith")
        .unwrap()
        .arg("list-human")
        .current_dir(&temp_dir)
        .assert()
        .success();
}

#[test]
fn test_git_forwarding() {
    let temp_dir = setup_test_repo();

    // Test that git status works through gith
    Command::cargo_bin("gith")
        .unwrap()
        .arg("status")
        .current_dir(&temp_dir)
        .assert()
        .success();
}
