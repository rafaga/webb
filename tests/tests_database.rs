
#[cfg(test)]
mod test_database{
    use webb::objects::{Character, Corporation, Alliance};
    use std::path::Path;
    use std::fs;

    #[test]
    fn test_database_creation() {
        let app_data = vec!["telescope/v0","a4b4a52e65fe4dce95eec1fab224407c","AFgvjrXi8rRpYbhsYe5hQFpPk266jyU40QlPYIam","http://localhost:4500/login"];
        let scope = vec!["publicData"];

        let path_str = "tests/test1.db";
        let path = Path::new(path_str);
        if path.exists() && path.is_file() {
            let _ = fs::remove_file(path_str);
        }
        let _ = webb::esi::EsiManager::new(app_data[0],app_data[1],app_data[2],app_data[3], scope, Some(path_str)); 
        assert!(path.exists());
    }


    #[test]
    fn test_database_character() {
        let app_data = vec!["telescope/v0","a4b4a52e65fe4dce95eec1fab224407c","AFgvjrXi8rRpYbhsYe5hQFpPk266jyU40QlPYIam","http://localhost:4500/login"];
        let scope = vec!["publicData"];

        let path_str = "tests/test2.db";
        let path = Path::new(path_str);
        if path.exists() && path.is_file() {
            let _ = fs::remove_file(path_str);
        }
        let mut mon = webb::esi::EsiManager::new(app_data[0],app_data[1],app_data[2],app_data[3], scope, Some(path_str)); 
        
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
            Err(_) => assert!(false),
        };

    }

    #[test]
    fn test_database_corporation() {
        let app_data = vec!["telescope/v0","a4b4a52e65fe4dce95eec1fab224407c","AFgvjrXi8rRpYbhsYe5hQFpPk266jyU40QlPYIam","http://localhost:4500/login"];
        let scope = vec!["publicData"];

        let path_str = "tests/test3.db";
        let path = Path::new(path_str);
        if path.exists() && path.is_file() {
            let _ = fs::remove_file(path_str);
        }
        let mut mon = webb::esi::EsiManager::new(app_data[0],app_data[1],app_data[2],app_data[3], scope, Some(path_str)); 
        
        let mut zcorp = Corporation::new();
        zcorp.id = 1;
        zcorp.name = "Test Corporation".to_string();

        match mon.write_corporation(&zcorp){
            Ok(rows_affected) => {
                if rows_affected > 0 {
                    if let Ok(corp) = mon.read_corporation(Some(vec![1])){
                        assert_eq!(Some(corp[0].id),Some(1))
                    }
                    else {
                        assert!(false)
                    }
                }
                else {
                    assert!(false)
                }
            },
            Err(_) => assert!(false),
        };
    }

    #[test]
    fn test_database_alliance() {
        let app_data = vec!["telescope/v0","a4b4a52e65fe4dce95eec1fab224407c","AFgvjrXi8rRpYbhsYe5hQFpPk266jyU40QlPYIam","http://localhost:4500/login"];
        let scope = vec!["publicData"];

        let path_str = "tests/test4.db";
        let path = Path::new(path_str);
        if path.exists() && path.is_file() {
            let _ = fs::remove_file(path_str);
        }
        let mut mon = webb::esi::EsiManager::new(app_data[0],app_data[1],app_data[2],app_data[3], scope, Some(path_str)); 
        
        let mut zally = Alliance::new();
        zally.id = 1;
        zally.name = "Test Alliance".to_string();

        match mon.write_alliance(&zally){
            Ok(rows_affected) => {
                if rows_affected > 0 {
                    if let Ok(ally) = mon.read_corporation(Some(vec![1])){
                        assert_eq!(Some(ally[0].id),Some(1))
                    }
                    else {
                        assert!(false)
                    }
                }
                else {
                    assert!(false)
                }
            },
            Err(_) => assert!(false),
        };
    }

}