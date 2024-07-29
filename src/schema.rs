// @generated automatically by Diesel CLI.

diesel::table! {
    communities (id) {
        id -> Int4,
        slug -> Text,
        name -> Text,
        description -> Text,
        deleted -> Bool,
        suspended -> Bool,
        restricted -> Bool,
        created_on -> Timestamp,
        deleted_on -> Timestamp,
        suspended_on -> Timestamp,
        restricted_on -> Timestamp,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Text,
        body -> Text,
        post_type -> Int2,
        score -> Int4,
        deleted -> Bool,
        removed -> Bool,
        edited -> Bool,
        locked -> Bool,
        archived -> Bool,
        created_on -> Timestamp,
        edited_on -> Timestamp,
        deleted_on -> Timestamp,
        removed_on -> Timestamp,
        locked_on -> Timestamp,
        archived_on -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Text,
        username -> Text,
        display_name -> Text,
        bio -> Text,
        pw_hash -> Text,
        pronouns -> Text,
        deleted -> Bool,
        suspended -> Bool,
        locked -> Bool,
        created_on -> Timestamp,
        deleted_on -> Timestamp,
        suspended_on -> Timestamp,
        locked_on -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    communities,
    posts,
    users,
);
