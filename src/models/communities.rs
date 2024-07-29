use diesel::*;

pub struct Community {
    id: i32,
    slug: String,
    name: String,
    description: String,

    deleted: bool,
    suspended: bool,
    restricted: bool,

    created_on: chrono::NaiveDateTime,
    deleted_on: chrono::NaiveDateTime,
    suspended_on: chrono::NaiveDateTime,
    restricted_on: chrono::NaiveDateTime,
}
