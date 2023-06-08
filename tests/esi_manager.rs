#[cfg(test)]
mod esi_manager{
    use webb::objects::{Character, Corporation, Alliance};
    use std::path::Path;
    use std::fs;
    use lazy_static::lazy_static;


    lazy_static! {
        static ref USER_AGENT: &'static str = "telescope/v0";
        static ref CLIENT_ID: &'static str = "a4b4a52e65fe4dce95eec1fab224407c";
        static ref SECRET_KEY: &'static str = "AFgvjrXi8rRpYbhsYe5hQFpPk266jyU40QlPYIam";
        static ref CALLBACK: &'static str = "http://localhost:4500/login";
    }

    #[test]
    fn db_creation() {
        let scope = vec![""];

        let path_str = "tests/databases/test1.db";
        let path = Path::new(path_str);
        if path.exists() && path.is_file() {
            let _ = fs::remove_file(path_str);
        }
        let _ = webb::esi::EsiManager::new(*USER_AGENT, &CLIENT_ID,*SECRET_KEY,*CALLBACK, scope, Some(path_str)); 
        assert!(path.exists());
    }


    #[test]
    fn db_character() {
        let scope = vec![""];

        let path_str = "tests/databases/test2.db";
        let path = Path::new(path_str);
        if path.exists() && path.is_file() {
            let _ = fs::remove_file(path_str);
        }
        let mut mon = webb::esi::EsiManager::new(*USER_AGENT, &CLIENT_ID,*SECRET_KEY,*CALLBACK, scope, Some(path_str)); 
        
        let mut zchar = Character::new();
        zchar.id = 2132411;
        zchar.name = "Rain Agnon".to_string();

        match mon.write_character(&zchar){
            Ok(rows_affected) => {
                if rows_affected > 0 {
                    if let Ok(chars) = mon.read_characters(Some(vec![2132411])){
                        assert_eq!(Some(chars[0].id),Some(2132411))
                    }
                    else {
                        assert!(false)
                    }
                }
                else {
                    assert!(false)
                }
            },
            Err(t_error) => panic!("Error: {}", t_error),
        };

    }

    #[test]
    fn db_corporation() {
        let scope = vec![""];

        let path_str = "tests/databases/test3.db";
        let path = Path::new(path_str);
        if path.exists() && path.is_file() {
            let _ = fs::remove_file(path_str);
        }
        let mut mon = webb::esi::EsiManager::new(*USER_AGENT, &CLIENT_ID,*SECRET_KEY,*CALLBACK, scope, Some(path_str)); 
        
        let mut zcorp = Corporation::new();
        zcorp.id = 1;
        zcorp.name = "Test Corporation".to_string();

        match mon.write_corporation(&zcorp){
            Ok(rows_affected) => {
                if rows_affected > 0 {
                    match mon.read_corporation(Some(vec![1])) {
                        Ok(corp) => assert_eq!(Some(corp[0].id),Some(1)),
                        Err(t_error) => panic!("Error: {}", t_error),
                    }
                }
                else {
                    assert!(false)
                }
            },
            Err(t_error) => panic!("Error: {}", t_error),
        };
    }

    #[test]
    fn db_alliance() {
        let scope = vec![""];

        let path_str = "tests/databases/test4.db";
        let path = Path::new(path_str);
        if path.exists() && path.is_file() {
            let _ = fs::remove_file(path_str);
        }
        let mut mon = webb::esi::EsiManager::new(*USER_AGENT, &CLIENT_ID,*SECRET_KEY,*CALLBACK, scope, Some(path_str)); 
        
        let mut zally = Alliance::new();
        zally.id = 1;
        zally.name = "Test Alliance".to_string();

        match mon.write_alliance(&zally){
            Ok(rows_affected) => {
                if rows_affected > 0 {
                    match mon.read_alliance(Some(vec![1])){
                        Ok(ally) => assert_eq!(Some(ally[0].id),Some(1)),
                        Err(t_error) => panic!("Error: {}", t_error),
                    }
                }
                else {
                    assert!(false)
                }
            },
            Err(t_error) => panic!("Error: {}", t_error),
        };
    }


    #[test]
    fn db_delete_character() {
        let scope = vec![""];

        let path_str = "tests/databases/test5.db";
        let path = Path::new(path_str);
        if path.exists() && path.is_file() {
            let _ = fs::remove_file(path_str);
        }
        let mut mon = webb::esi::EsiManager::new(*USER_AGENT, &CLIENT_ID,*SECRET_KEY,*CALLBACK, scope, Some(path_str)); 
        let mut chars = Vec::new();
        let mut zchar = Character::new();
        zchar.id = 23101429;
        zchar.name = "test1".to_string();
        chars.push(zchar);
        let mut zchar = Character::new();
        zchar.id = 12341245;
        zchar.name = "test2".to_string();
        chars.push(zchar);

        while let Some(char_x) = chars.pop(){
            let _ = mon.write_character(&char_x);
        }
        match mon.remove_characters(Some(vec![23101429])) {
            Ok(rows) => {
                assert_eq!(1,rows);
            },
            Err(t_error) => panic!("{}", t_error),
        }

    }

    
    #[test]
    fn db_delete_corporation() {
        let scope = vec![""];

        let path_str = "tests/databases/test6.db";
        let path = Path::new(path_str);
        if path.exists() && path.is_file() {
            let _ = fs::remove_file(path_str);
        }
        let mut mon = webb::esi::EsiManager::new(*USER_AGENT, &CLIENT_ID,*SECRET_KEY,*CALLBACK, scope, Some(path_str)); 
        let mut corps = Vec::new();
        let mut zcorp = Corporation::new();
        zcorp.id = 456;
        zcorp.name = "corporation 1".to_string();
        corps.push(zcorp);
        let mut zcorp = Corporation::new();
        zcorp.id = 907123;
        zcorp.name = "corporation 2".to_string();
        corps.push(zcorp);

        while let Some(corp_x) = corps.pop(){
            let _ = mon.write_corporation(&corp_x);
        }
        match mon.remove_corporation(Some(vec![907123])) {
            Ok(rows) => {
                assert_eq!(1,rows);
            },
            Err(t_error) => panic!("{}", t_error),
        }
    }
    
    #[test]
    fn db_delete_alliance() {
        let scope = vec![""];

        let path_str = "tests/databases/test7.db";
        let path = Path::new(path_str);
        if path.exists() && path.is_file() {
            let _ = fs::remove_file(path_str);
        }
        let mut mon = webb::esi::EsiManager::new(*USER_AGENT, &CLIENT_ID,*SECRET_KEY,*CALLBACK, scope, Some(path_str)); 
        let mut alliances = Vec::new();
        let mut zally = Alliance::new();
        zally.id = 21347;
        zally.name = "alliance 1".to_string();
        alliances.push(zally);
        let mut zally = Alliance::new();
        zally.id = 213948;
        zally.name = "alliance 2".to_string();
        alliances.push(zally);

        while let Some(ally_x) = alliances.pop(){
            let _ = mon.write_alliance(&ally_x);
        }
        match mon.remove_alliance(Some(vec![21347])) {
            Ok(rows) => {
                assert_eq!(1,rows);
            },
            Err(t_error) => panic!("{}", t_error),
        }
    }

    // required scope: publicData, esi-location.read_location.v1
    #[test]
    #[cfg_attr(not(feature = "esi-api-test"), ignore)]
    fn esi_get_public_data() {
        let scope = vec!["publicData","esi-location.read_location.v1"];
        let path_str = Some("tests/databases/esi0.db");
        let path = Path::new(path_str.unwrap());
        let mut vec = Vec::new();
        if path.exists() && path.is_file() {
            let _ = fs::remove_file(path);
        }

        let mut esimon = webb::esi::EsiManager::new(*USER_AGENT, &CLIENT_ID,*SECRET_KEY,*CALLBACK, scope, path_str); 
        let (url,_rand) = esimon.esi.get_authorize_url().unwrap();
        
        match open::that(&url){
            Ok(()) => {
                match esimon.auth_user(4500){
                    Ok(Some(player)) => {
                        vec.push(player);
                        assert_ne!(vec[0].name,"");
                    },
                    Ok(None) => {
                        panic!("No user has been authenticated");
                    },
                    Err(esi_error) => {
                        panic!("Error: {}", esi_error);
                    }
                }
            },
            Err(err) => panic!("An error occurred when opening '{}': {}", url, err),
        }
    }

    #[test]
    fn db_get_characters() {
        let scope = vec![""];

        let path_str = "tests/databases/char0.db";
        let path = Path::new(path_str);
        if !path.exists() && !path.is_file() {
            panic!("test database file not exists.")
        }

        let mut esimon = webb::esi::EsiManager::new(*USER_AGENT, &CLIENT_ID,*SECRET_KEY,*CALLBACK, scope, Some(path_str)); 
        let res_chars = esimon.read_characters(None);
        if let Ok(chars) = res_chars {
            assert_eq!(chars.len(),1);
        }
    }
}