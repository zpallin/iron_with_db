
#[cfg(test)]
#[test]
fn test_DatabaseConfig() {
    use db::DatabaseConfig;
    
    let dbConf = DatabaseConfig::new("localhost:27017/test");

    assert_eq!("localhost", dbConf.hostname);
    assert_eq!("test", dbConf.dbname);
    assert_eq!(27017 as u16, dbConf.port);
}

