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

    red_analyses (id) {
        id -> Int4,
        wine_id -> Int4,
        red_fruit -> Int4,
        black_fruit -> Int4,
        blue_fruit -> Int4,
        floral -> Int4,
        vegetal -> Int4,
        dried_herbs -> Int4,
        mint -> Int4,
        peppercorn -> Int4,
        mocha -> Int4,
        animalic -> Int4,
        balsamic -> Int4,
        organic -> Int4,
        inorganic -> Int4,
        oak -> Int4,
        tannin -> Int4,
        acid -> Int4,
        alcohol -> Int4,
        fruit_condition -> Fruit_condition,
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

    reviews (user_id, wine_id) {
        user_id -> Int4,
        wine_id -> Int4,
        liked -> Bool,
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

    white_analyses (id) {
        id -> Int4,
        wine_id -> Int4,
        apple -> Int4,
        citrus -> Int4,
        stone_fruit -> Int4,
        tropical -> Int4,
        floral -> Int4,
        herbal -> Int4,
        vegetal -> Int4,
        botrytis -> Int4,
        nutty -> Int4,
        lees -> Int4,
        buttery -> Int4,
        organic -> Int4,
        inorganic -> Int4,
        wood -> Int4,
        phenolic -> Int4,
        sweetness -> Int4,
        acid -> Int4,
        alcohol -> Int4,
        fruit_condition -> Fruit_condition,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    wines (id) {
        id -> Int4,
        name -> Varchar,
        color -> Wine_color,
        producer -> Varchar,
        vintage -> Int4,
        region -> Varchar,
        country -> Varchar,
        sparkling -> Bool,
        alcohol -> Float8,
    }
}

joinable!(compositions -> grapes (grape));
joinable!(compositions -> wines (wine_id));
joinable!(red_analyses -> wines (wine_id));
joinable!(reviews -> users (user_id));
joinable!(reviews -> wines (wine_id));
joinable!(white_analyses -> wines (wine_id));
joinable!(wines -> countries (country));
joinable!(wines -> producers (producer));
joinable!(wines -> regions (region));

allow_tables_to_appear_in_same_query!(
    compositions,
    countries,
    grapes,
    producers,
    red_analyses,
    regions,
    reviews,
    users,
    white_analyses,
    wines,
);
