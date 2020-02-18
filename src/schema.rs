table! {
    nyt (id) {
        id -> Int4,
        title -> Text,
        author -> Nullable<Text>,
        #[sql_name = "yield"]
        yield_ -> Text,
        time -> Nullable<Text>,
        description -> Nullable<Text>,
        featured -> Nullable<Text>,
        tags -> Array<Text>,
        ratings -> Int4,
        rating -> Int4,
        sub_components -> Nullable<Array<Text>>,
        indices -> Nullable<Array<Int4>>,
        ingredients -> Array<Text>,
        quantities -> Array<Text>,
        steps -> Array<Text>,
        comments -> Nullable<Array<Text>>,
        comment_votes -> Nullable<Array<Int4>>,
        url_id -> Text,
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
