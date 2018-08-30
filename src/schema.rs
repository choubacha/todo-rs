table! {
    lists (id) {
        id -> Int4,
        title -> Text,
    }
}

table! {
    todos (id) {
        id -> Int4,
        title -> Text,
        is_completed -> Bool,
        list_id -> Int4,
    }
}

joinable!(todos -> lists (list_id));

allow_tables_to_appear_in_same_query!(
    lists,
    todos,
);
