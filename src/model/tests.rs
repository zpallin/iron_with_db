
#[cfg(test)]

#[test]
fn test_model() {
    use rustc_serialize::json;
    use model::Model;

    #[derive(RustcEncodable,RustcDecodable)]
    struct Person {
        name: String,
    }

    impl Model for Person {
        fn insert(&self) -> Result<String, String> {
            println!("model insert test...");
            Ok(format!("Insert worked for {}!", &self.name))
        }
        fn update(&self) -> Result<String, String> {
            println!("model update test...");
            Ok(format!("Update worked for {}!", &self.name))
        }
        fn delete(&self) -> Result<String, String> {
            println!("model delete test...");
            Ok(format!("Delete worked for {}!", &self.name))
        }
    }

    let mut p = Person{ name: "Test".to_string() };
    assert_eq!("Insert worked for Test!", p.insert().unwrap());
    assert_eq!("Update worked for Test!", p.update().unwrap());
    assert_eq!("Delete worked for Test!", p.delete().unwrap());

    p.name = "NewTest".to_string();
    assert_eq!("Insert worked for NewTest!", p.insert().unwrap());
    assert_eq!("Update worked for NewTest!", p.update().unwrap());
    assert_eq!("Delete worked for NewTest!", p.delete().unwrap());

}
