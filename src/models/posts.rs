use diesel::*;

pub struct Post {
    id: i32,
    title: String,
    body: String,
    post_type: i32, // 0 = text, 1 = image, 2 = video, 3 = crosspost, 4 = embed
    score: i32,

    deleted: bool,
    removed: bool,
    edited: bool,
    locked: bool,
    archived: bool,

    created_on: chrono::NaiveDateTime,
    edited_on: chrono::NaiveDateTime,
    deleted_on: chrono::NaiveDateTime,
    removed_on: chrono::NaiveDateTime,
    locked_on: chrono::NaiveDateTime,
    archived_on: chrono::NaiveDateTime,
}
