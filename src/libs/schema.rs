table! {
    messages (id) {
        id -> Int4,
        website -> Varchar,
        user_name -> Varchar,
        message -> Text,
        published -> Bool,
    }
}
