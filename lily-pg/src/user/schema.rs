table! {
    tabledetails (id) {
        id -> Integer,
        table_name -> Varchar,
        counts -> BigInt,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Varchar,
        email -> Varchar,
        phone -> Nullable<Varchar>,
        password -> Text,
        uname -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    tabledetails,
    users,
);
