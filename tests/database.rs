#[cfg(test)]
mod tests_database {
    use webb::database::{Database,Character};
    use std::path::Path;
    use std::fs;

    #[test]
    fn test_database_creation() {
        let path = Path::new("tests/tests.db");
        if path.exists() && path.is_file() {
            fs::remove_file("tests/tests-create.db");
        }
        let mut  db = Database::new("tests/tests.db".to_string());
        if let Ok(true) = db.open() {
            assert!(path.exists());
            fs::remove_file("tests/tests-create.db");
        }
    }

    /* 
    #[test]
    fn test_uuidv5_url() {
        let uuid = generate_uuid5(NamespaceType::URL, "woah!".to_string());
        //4e8693aa-3cfa-53cd-acc5-6f94fca1c983
        assert_eq!(uuid,"4e8693aa-3cfa-53cd-acc5-6f94fca1c983");
    }

    #[test]
    fn test_uuidv5_oid() {
        let uuid = generate_uuid5(NamespaceType::OID, "woah!".to_string());
        //62e9a41d-4bab-5cb0-949f-70c2602a9402
        assert_eq!(uuid,"62e9a41d-4bab-5cb0-949f-70c2602a9402");
    }

    #[test]
    fn test_uuidv5_x500() {
        let uuid = generate_uuid5(NamespaceType::X500, "woah!".to_string());
        //c62447c2-a78c-521e-9f4e-709bd995acb2
        assert_eq!(uuid,"c62447c2-a78c-521e-9f4e-709bd995acb2");
    }*/
}