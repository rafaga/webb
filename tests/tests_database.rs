
#[cfg(test)]
mod test_database{
    use webb::objects::Character;
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

        match mon.write_character(zchar){
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

}