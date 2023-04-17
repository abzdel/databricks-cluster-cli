#[cfg(test)]
mod tests {
    use std::process::Command;
    use tempfile::TempDir;
    use assert_cmd::cargo::CommandCargoExt;

    #[test]
    fn test_create_cluster() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let output = Command::cargo_bin("cluster")?
            .arg("create")
            .arg("test-cluster")
            .arg("--optimize")
            .arg("general")
            .current_dir(temp_dir.path())
            .output()?;
        assert!(output.status.success());
        // TODO: Add more assertions to validate the output
        Ok(())
    }

    #[test]
    fn test_create_cluster_with_invalid_size() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let output = Command::cargo_bin("cluster")?
            .arg("create")
            .arg("--name")
            .arg("test-cluster")
            .arg("--size")
            .arg("invalid")
            .current_dir(temp_dir.path())
            .output()?;
        assert!(!output.status.success());
        // TODO: Add more assertions to validate the error message
        Ok(())
    }

    #[test]
    fn test_delete_cluster() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let output = Command::cargo_bin("cluster")?
            .arg("delete")
            .arg("--name")
            .arg("test-cluster")
            .current_dir(temp_dir.path())
            .output()?;
        assert!(output.status.success());
        // TODO: Add more assertions to validate the output
        Ok(())
    }
}
