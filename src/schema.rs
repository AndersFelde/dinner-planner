// @generated automatically by Diesel CLI.

diesel::table! {
    days (id) {
        id -> Integer,
        date -> Date,
        meal_id -> Nullable<Integer>,
        week -> Integer,
        year -> Integer,
        anders_attend -> Bool,
        ac_attend -> Bool,
        andreas_attend -> Bool,
    }
}

diesel::table! {
    days_ingredients (day_id, ingredient_id) {
        day_id -> Integer,
        ingredient_id -> Integer,
        bought -> Bool,
    }
}

diesel::table! {
    extra_items (id) {
        id -> Integer,
        name -> Text,
        amount -> Integer,
        bought -> Bool,
    }
}

diesel::table! {
    ingredients (id) {
        id -> Integer,
        name -> Text,
        amount -> Integer,
        meal_id -> Integer,
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

diesel::table! {
    receipt_days (receipt_id, day_id) {
        receipt_id -> Integer,
        day_id -> Integer,
    }
}

diesel::table! {
    receipt_items (id) {
        id -> Integer,
        receipt_id -> Integer,
        name -> Text,
        price -> Float,
        anders_pay -> Bool,
        andreas_pay -> Bool,
        ac_pay -> Bool,
    }
}

diesel::table! {
    receipts (id) {
        id -> Integer,
        store -> Text,
        datetime -> Timestamp,
    }
}

diesel::joinable!(days -> meals (meal_id));
diesel::joinable!(days_ingredients -> days (day_id));
diesel::joinable!(days_ingredients -> ingredients (ingredient_id));
diesel::joinable!(ingredients -> meals (meal_id));
diesel::joinable!(receipt_days -> days (day_id));
diesel::joinable!(receipt_days -> receipts (receipt_id));
diesel::joinable!(receipt_items -> receipts (receipt_id));

diesel::allow_tables_to_appear_in_same_query!(
    days,
    days_ingredients,
    extra_items,
    ingredients,
    meals,
    receipt_days,
    receipt_items,
    receipts,
);
