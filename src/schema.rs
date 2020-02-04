table! {
    nyt (id) {
        id -> Int4,
        title -> Varchar,
        author -> Varchar,
        #[sql_name = "yield"]
        yield_ -> Varchar,
        time -> Float4,
        description -> Text,
        featured -> Text,
        tags -> Text,
        ratings -> Nullable<Int4>,
        rating -> Nullable<Float4>,
        ingredients -> Text,
        steps -> Text,
        comments -> Text,
    }
}

table! {
    recipes_table (id) {
        id -> Int4,
        title -> Varchar,
        time -> Float4,
        yields -> Int4,
        ingredients -> Varchar,
        instructions -> Varchar,
        rating -> Float4,
        reviews -> Int4,
        url_id -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    nyt,
    recipes_table,
);
