#[cfg(test)]
mod tests_database {
    use webb::auth_service;
    use webb::database::Database;
    use std::path::Path;
    use std::fs;
    use webb::esi::EsiManager;
    use webb::esi::data::Data;

    #[test]
    fn test_database_creation() {
        let path = Path::new("tests/tests.db");
        if path.exists() && path.is_file() {
            let _ = fs::remove_file("tests/tests-create.db");
        }
        let mut  db = Database::new("tests/tests.db".to_string());
        if let Ok(true) = db.open() {
            assert!(path.exists());
            let _ = fs::remove_file("tests/tests-create.db");
        }
    }

    #[test]
    fn test_web_auth() {
        let esi_data = Data::new();
        let esimon = EsiManager::new(esi_data);
        let (url,_rand) = esimon.esi.get_authorize_url().unwrap();
        match open::that(&url){
            Ok(()) => {
                let result = auth_service::open_auth_service().unwrap();
                assert_ne!(result.0.as_str(),"");
            },
            Err(err) => panic!("An error occurred when opening '{}': {}", url, err),
        }
    }


    /* 
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