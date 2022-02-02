table! {
    categories (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        created -> Timestamp,
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

allow_tables_to_appear_in_same_query!(categories, users,);
