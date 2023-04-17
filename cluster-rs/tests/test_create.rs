#[cfg(test)]
mod tests {
    use assert_cmd::cargo::CommandCargoExt;
    use std::process::Command;
    use tempfile::TempDir;

    #[test]
    fn test_create_cluster_only_required_args() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let output = Command::cargo_bin("cluster")?
            .arg("create")
            .arg("--name")
            .arg("test-cluster")
            .current_dir(temp_dir.path())
            .output()?;
        assert!(output.status.success());
        // assert more stuff

        Ok(())
    }

    #[test]
    fn create_cluster_optimize_general() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let output = Command::cargo_bin("cluster")?
            .arg("create")
            .arg("--name")
            .arg("test-cluster")
            .arg("--optimize")
            .arg("general")
            .current_dir(temp_dir.path())
            .output()?;
        assert!(output.status.success());
        Ok(())
    }

    #[test]
    fn create_cluster_optimize_memory() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let output = Command::cargo_bin("cluster")?
            .arg("create")
            .arg("--name")
            .arg("test-cluster")
            .arg("--optimize")
            .arg("memory")
            .current_dir(temp_dir.path())
            .output()?;
        assert!(output.status.success());
        Ok(())
    }
    #[test]
    fn create_cluster_optimize_storage() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let output = Command::cargo_bin("cluster")?
            .arg("create")
            .arg("--name")
            .arg("test-cluster")
            .arg("--optimize")
            .arg("storage")
            .current_dir(temp_dir.path())
            .output()?;
        assert!(output.status.success());
        Ok(())
    }
    #[test]
    fn create_cluster_optimize_compute() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let output = Command::cargo_bin("cluster")?
            .arg("create")
            .arg("--name")
            .arg("test-cluster")
            .arg("--optimize")
            .arg("compute")
            .current_dir(temp_dir.path())
            .output()?;
        assert!(output.status.success());
        Ok(())
    }
    #[test]
    fn test_create_cluster_with_invalid_optimization() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let output = Command::cargo_bin("cluster")?
            .arg("create")
            .arg("--name")
            .arg("test-cluster")
            .arg("--optimize")
            .arg("not-a-real-optimization")
            .current_dir(temp_dir.path())
            .output()?;
        assert!(!output.status.success());
        Ok(())
    }

    #[test]
    fn test_create_cluster_with_no_name() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let output = Command::cargo_bin("cluster")?
            .arg("create")
            .current_dir(temp_dir.path())
            .output()?;
        assert!(!output.status.success());
        Ok(())
    }

    #[test]
    fn test_create_cluster_with_invalid_args() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let output = Command::cargo_bin("cluster")?
            .arg("create")
            .arg("--name")
            .arg("test-cluster")
            .arg("--invalid-arg")
            .arg("value")
            .current_dir(temp_dir.path())
            .output()?;
        assert!(!output.status.success());
        Ok(())
    }
}
