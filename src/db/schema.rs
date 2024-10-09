// @generated automatically by Diesel CLI.

diesel::table! {
    comments (comment_id) {
        comment_id -> Integer,
        post_id -> Integer,
        user_id -> Integer,
        comment_body -> Text,
    }
}

diesel::table! {
    posts (post_id) {
        post_id -> Integer,
        #[max_length = 255]
        title -> Varchar,
        post_body -> Text,
        published -> Bool,
        user_id -> Integer,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
    }
}

diesel::joinable!(comments -> posts (post_id));
diesel::joinable!(comments -> users (user_id));
diesel::joinable!(posts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    posts,
    users,
);
