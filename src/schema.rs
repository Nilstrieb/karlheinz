table! {
    person (id) {
        id -> Bpchar,
        name -> Varchar,
        age -> Int4,
    }
}

table! {
    posts (id) {
        id -> Int4,
        author -> Varchar,
        title -> Varchar,
        body -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    person,
    posts,
);
