use mysql;

#[derive(Clone)]
struct User {
    id: i32,
    name: String,
}

pub fn find_by(id: i32) -> String {
    let pool = mysql::Pool::new("mysql://root:@localhost:3306").unwrap();

    let users: Vec<User> = pool.prep_exec(
        "SELECT id, name from test.user where id = ?",
        (id.to_string(),),
    ).map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (id, name) = mysql::from_row(row);
                    User { id: id, name: name }
                })
                .collect()
        })
        .unwrap();
    let user_name: String = users.first().unwrap().clone().name;
    user_name
}
