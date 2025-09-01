// @generated automatically by Diesel CLI.

diesel::table! {
    days (id) {
        id -> Integer,
        date -> Date,
        meal_id -> Nullable<Integer>,
        week -> Integer,
        year -> Integer,
    }
}

diesel::table! {
    ingredients (id) {
        id -> Integer,
        name -> Text,
        amount -> Integer,
        bought -> Bool,
        meal_id -> Nullable<Integer>,
    }
}

diesel::table! {
    meals (id) {
        id -> Integer,
        name -> Text,
        image -> Text,
        recipie_url -> Nullable<Text>,
    }
}

diesel::joinable!(ingredients -> meals (meal_id));

diesel::allow_tables_to_appear_in_same_query!(
    days,
    ingredients,
    meals,
);
