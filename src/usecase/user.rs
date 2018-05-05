use external;

pub fn find_by(user_id: i32) -> String {
    let name = external::mysql::user::find_by(user_id);
    name
}
