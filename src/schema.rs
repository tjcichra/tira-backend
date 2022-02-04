table! {
    categories (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        created -> Timestamp,
    }
}

table! {
    tickets (id) {
        id -> Int4,
        category_id -> Nullable<Int4>,
        subject -> Text,
        description -> Nullable<Text>,
        status -> Text,
        priority -> Text,
        created -> Timestamp,
        reporter_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
        email_address -> Nullable<Text>,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
    }
}

joinable!(tickets -> categories (category_id));
joinable!(tickets -> users (reporter_id));

allow_tables_to_appear_in_same_query!(
    categories,
    tickets,
    users,
);
