table! {
    use diesel::sql_types::*;
    use crate::models::*;

    compositions (wine_id, grape) {
        wine_id -> Int4,
        grape -> Varchar,
        percent -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    countries (name) {
        name -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    grapes (name) {
        name -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    producers (name) {
        name -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    regions (name) {
        name -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    tasting_notes (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    users (id) {
        id -> Int4,
        facebook_user_id -> Nullable<Varchar>,
        google_username -> Nullable<Varchar>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    wines (id) {
        id -> Int4,
        name -> Varchar,
        producer -> Varchar,
        vintage -> Int4,
        region -> Varchar,
        country -> Varchar,
        sparkling -> Bool,
        sweetness -> Int4,
        tannin -> Int4,
        acid -> Int4,
        alcohol -> Int4,
        body -> Int4,
        intensity -> Wine_intensity,
    }
}

joinable!(compositions -> grapes (grape));
joinable!(compositions -> wines (wine_id));
joinable!(wines -> countries (country));
joinable!(wines -> producers (producer));
joinable!(wines -> regions (region));

allow_tables_to_appear_in_same_query!(
    compositions,
    countries,
    grapes,
    producers,
    regions,
    tasting_notes,
    users,
    wines,
);
