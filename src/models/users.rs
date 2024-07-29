use diesel::*;

pub struct User {
    id: i32,
    email: String,
    username: String,
    display_name: String,
    bio: String,
    pw_hash: String,
    pronouns: String,

    deleted: bool,
    suspended: bool,
    locked: bool,

    created_on: chrono::NaiveDateTime,
    deleted_on: chrono::NaiveDateTime,
    suspended_on: chrono::NaiveDateTime,
    locked_on: chrono::NaiveDateTime,
}
