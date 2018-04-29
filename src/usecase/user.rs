use ::domain;

pub fn find_by(user_id: i32) -> String {
    let name = match user_id {
        1 => "Taro".to_string(),
        2 => "Jiro".to_string(),
        _ => "Unknown".to_string(),
    };
    name
}
