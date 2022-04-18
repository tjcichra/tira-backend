table! {
    assignments (id) {
        id -> Int8,
        ticket_id -> Int8,
        assignee_id -> Int8,
        assigner_id -> Int8,
        assigned -> Timestamp,
    }
}

table! {
    categories (id) {
        id -> Int8,
        name -> Text,
        description -> Nullable<Text>,
        creator_id -> Int8,
        created -> Timestamp,
        archived -> Bool,
    }
}

table! {
    comments (id) {
        id -> Int8,
        ticket_id -> Int8,
        commenter_id -> Int8,
        content -> Text,
        commented -> Timestamp,
    }
}

table! {
    sessions (uuid) {
        uuid -> Text,
        user_id -> Int8,
        created -> Timestamp,
        expiration -> Timestamp,
    }
}

table! {
    tickets (id) {
        id -> Int8,
        subject -> Text,
        description -> Nullable<Text>,
        category_id -> Nullable<Int8>,
        priority -> Text,
        status -> Text,
        created -> Timestamp,
        reporter_id -> Int8,
    }
}

table! {
    users (id) {
        id -> Int8,
        username -> Text,
        password -> Text,
        email_address -> Nullable<Text>,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        profile_picture_url -> Nullable<Text>,
        created -> Timestamp,
        archived -> Bool,
    }
}

joinable!(assignments -> tickets (ticket_id));
joinable!(categories -> users (creator_id));
joinable!(comments -> tickets (ticket_id));
joinable!(comments -> users (commenter_id));
joinable!(sessions -> users (user_id));
joinable!(tickets -> categories (category_id));
joinable!(tickets -> users (reporter_id));

allow_tables_to_appear_in_same_query!(
    assignments,
    categories,
    comments,
    sessions,
    tickets,
    users,
);
