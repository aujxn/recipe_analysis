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
