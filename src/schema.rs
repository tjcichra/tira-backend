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
