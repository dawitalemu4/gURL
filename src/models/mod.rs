pub mod request;
pub mod user;

pub fn serialize_favorites_for_db(favorites: &Option<Vec<i32>>) -> String {
    match favorites {
        Some(favorites) => {
            let csv = favorites
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(",");
            csv
        }
        None => String::new(),
    }
}

pub fn deserialize_favorites_for_db(favorites_str: Option<String>) -> Option<Vec<i32>> {
    favorites_str.filter(|s| !s.is_empty()).map(|s| {
        s.split(',')
            .filter_map(|n| n.trim().parse::<i32>().ok())
            .collect::<Vec<i32>>()
    })
}

pub fn serialize_bool_for_db(value: bool) -> u8 {
    match value {
        true => 1,
        false => 0,
    }
}

pub fn deserialize_bool_for_db(value: u8) -> bool {
    match value {
        1 => true,
        0 => false,
        _ => false,
    }
}
