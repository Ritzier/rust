use super::*;

// `build()` error `Error::ConfigurationNotExists` when file not exist
#[test]
fn build_fails_when_config_does_not_exist() {
    let path = PathBuf::from("definetly_does_not_exist.toml");
    let result = Watcher::build(path.clone());

    let abs = absolute(path).unwrap();
    assert!(matches!(result, Err(Error::ConfigurationNotExists { ref path }) if path == &abs));
}

// `build()` success when the `configuration` file actually exists
#[tokio::test]
async fn build_success_with_existing_config() {
    let dir = TempDir::new().unwrap();
    let config = dir.path().join("config.toml");
    fs::write(&config, "[settings]").await.unwrap();

    let watcher = Watcher::build(config);
    assert!(watcher.is_ok())
}
