
#[cfg(test)]
#[test]
fn test_database_config() {
    use db::DatabaseConfig;
    
    let db_conf = DatabaseConfig::new("localhost:27017/test");

    assert_eq!("localhost", db_conf.hostname);
    assert_eq!("test", db_conf.dbname);
    assert_eq!(27017 as u16, db_conf.port);
}

