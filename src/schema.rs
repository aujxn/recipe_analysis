table! {
    comments (id) {
        id -> Int4,
        recipes_id -> Int4,
        body -> Text,
        votes -> Int4,
    }
}

table! {
    ingredients (id) {
        id -> Int4,
        name -> Text,
    }
}

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
    recipe_ingredient (id) {
        id -> Int4,
        recipes_id -> Nullable<Int4>,
        ingredients_id -> Nullable<Int4>,
        sub_components_id -> Nullable<Int4>,
        quantity -> Float4,
        quantity_note -> Nullable<Text>,
    }
}

table! {
    recipe_tag (id) {
        id -> Int4,
        recipes_id -> Nullable<Int4>,
        tags_id -> Nullable<Int4>,
    }
}

table! {
    recipes (id) {
        id -> Int4,
        title -> Text,
        source -> Int4,
        url -> Text,
        yields -> Text,
        time -> Nullable<Int4>,
        description -> Nullable<Text>,
        steps -> Array<Text>,
        num_ratings -> Int4,
        avg_rating -> Float4,
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

table! {
    sub_components (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    tags (id) {
        id -> Int4,
        name -> Text,
    }
}

joinable!(comments -> recipes (recipes_id));
joinable!(recipe_ingredient -> ingredients (ingredients_id));
joinable!(recipe_ingredient -> recipes (recipes_id));
joinable!(recipe_ingredient -> sub_components (sub_components_id));
joinable!(recipe_tag -> recipes (recipes_id));
joinable!(recipe_tag -> tags (tags_id));

allow_tables_to_appear_in_same_query!(
    comments,
    ingredients,
    nyt,
    recipe_ingredient,
    recipe_tag,
    recipes,
    recipes_table,
    sub_components,
    tags,
);
