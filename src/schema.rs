table! {
    assignments (ticket_id, user_id) {
        ticket_id -> Int4,
        user_id -> Int4,
        assigned -> Timestamp,
    }
}

table! {
    categories (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        created -> Timestamp,
    }
}

table! {
    comments (id) {
        id -> Int4,
        ticket_id -> Int4,
        commenter_id -> Int4,
        content -> Text,
        commented -> Timestamp,
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

joinable!(assignments -> tickets (ticket_id));
joinable!(assignments -> users (user_id));
joinable!(comments -> tickets (ticket_id));
joinable!(comments -> users (commenter_id));
joinable!(tickets -> categories (category_id));
joinable!(tickets -> users (reporter_id));

allow_tables_to_appear_in_same_query!(
    assignments,
    categories,
    comments,
    tickets,
    users,
);
