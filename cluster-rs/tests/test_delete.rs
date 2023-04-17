// load in first element of cluster_ids.txt so we can delete it

#[cfg(test)]
mod tests {
    use assert_cmd::cargo::CommandCargoExt;
    use std::process::Command;
    use tempfile::TempDir;

    #[test]
    fn test_delete_cluster() -> Result<(), Box<dyn std::error::Error>> {
        // get a true cluster id by calling databricks cluster list and saving one value as a variable
        let output = std::process::Command::new("databricks")
            .args(&["clusters", "list"])
            .output()
            .expect("Failed to execute command");
        let output_str = std::str::from_utf8(&output.stdout).unwrap();
        let re = regex::Regex::new(r"^([\d\w\-]+)").unwrap();
        let cluster_id = re
            .captures(output_str)
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .to_owned();
        println!("cluster_id: {}", cluster_id);
        let temp_dir = TempDir::new()?;
        let output = Command::cargo_bin("cluster")?
            .arg("delete")
            .arg("--cluster")
            .arg(cluster_id)
            .current_dir(temp_dir.path())
            .output()?;
        assert!(output.status.success());
        Ok(())
    }

    #[test]
    fn test_delete_invalid_cluster() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let output = Command::cargo_bin("cluster")?
            .arg("delete")
            .arg("--cluster")
            .arg("invalid_cluster_id")
            .current_dir(temp_dir.path())
            .output()?;
        assert!(output.status.success());
        Ok(())
    }
}
