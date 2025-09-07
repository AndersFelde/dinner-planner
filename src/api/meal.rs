use crate::models::ingredient::IngredientForm;
use crate::models::meal::{Meal, MealForm, MealWithIngredients};
use leptos::logging::log;
use leptos::prelude::*;

#[server]
pub async fn get_meals() -> Result<Vec<Meal>, ServerFnError> {
    use crate::api::ssr::*;
    let db = &mut get_db()?;
    server_err!(
        meals::table.select(Meal::as_select()).load(db),
        "Could not get meals"
    )
}

#[server]
pub async fn get_meal(id: i32) -> Result<MealWithIngredients, ServerFnError> {
    use crate::api::ssr::*;
    let db = &mut get_db()?;

    let meal = server_err!(
        meals::table.filter(meals::id.eq(id)).first::<Meal>(db),
        "Could not get meal with id {id}"
    )?;
    let ingredients = server_err!(
        Ingredient::belonging_to(&meal)
            .select(Ingredient::as_select())
            .load(db),
        "Could not get ingredients for meal_id {id}"
    )?;
    Ok(MealWithIngredients { meal, ingredients })
}
#[server]
pub async fn update_meal_with_ingredients(
    meal: Meal,
    ingredient_forms: Vec<IngredientForm>,
) -> Result<(), ServerFnError> {
    use crate::api::ingredient::{delete_ingredients, insert_ingredient};
    use crate::api::ssr::*;
    let db = &mut get_db()?;
    server_err!(
        update(meals::table).set(meal.clone()).execute(db),
        "Could not update meal {meal:?}"
    )?;
    // This is kinda hacky, but easy
    delete_ingredients(db, meal.id)?;
    for mut ingredient_form in ingredient_forms {
        ingredient_form.meal_id = Some(meal.id);
        insert_ingredient(db, ingredient_form)?;
    }
    Ok(())
}
#[server]
pub async fn create_meal_with_ingredients(
    meal_form: MealForm,
    ingredient_forms: Vec<IngredientForm>,
) -> Result<(), ServerFnError> {
    use crate::api::ingredient::insert_ingredient;
    use crate::api::ssr::*;
    let db = &mut get_db()?;
    let mut meal_form = meal_form.clone();
    if !utils::is_valid_url(&meal_form.image) {
        meal_form.image = utils::get_image_url(meal_form.name.clone()).await?
    }
    let meal: Meal = server_err!(
        insert_into(meals::table).values(&meal_form).get_result(db),
        "Could not insert meal {meal_form:?}"
    )?;
    for mut ingredient_form in ingredient_forms {
        ingredient_form.meal_id = Some(meal.id);
        insert_ingredient(db, ingredient_form)?;
    }
    Ok(())
}

#[server]
pub async fn get_all_meals_with_ingredients() -> Result<Vec<MealWithIngredients>, ServerFnError> {
    use crate::api::ingredient::get_ingredients;
    use crate::api::ssr::*;
    let db = &mut get_db()?;
    let meals = get_meals().await?;
    let ingredients = get_ingredients(db)?;
    Ok(ingredients
        .grouped_by(&meals)
        .into_iter()
        .zip(meals)
        .map(|(ingredients, meal)| MealWithIngredients { meal, ingredients })
        .collect())
}

#[cfg(feature = "ssr")]
mod utils {
    use leptos::prelude::ServerFnError;

    #[derive(serde::Deserialize, Debug)]
    pub struct GoogleImageResult {
        pub items: [ImageItem; 1],
    }

    #[derive(serde::Deserialize, Debug)]
    pub struct ImageItem {
        pub link: String,
    }

    pub fn is_valid_url(url: &str) -> bool {
        use url::Url;
        Url::parse(url).is_ok()
    }

    pub async fn get_image_url(query: String) -> Result<String, ServerFnError> {
        let api_key = std::env::var("GOOGLE_KEY").map_err(|e| ServerFnError::new(e.to_string()))?;
        let search_engine_id = std::env::var("GOOGLE_SEARCH_ENGINE_ID")
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        let url = format!(
        "https://customsearch.googleapis.com/customsearch/v1?key={}&cx={}&q={}&searchType=image&num=1",
        api_key,
        search_engine_id,
        urlencoding::encode(&query)
    );

        let client = reqwest::Client::new();
        let resp = client
            .get(&url)
            .send()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        let data = resp
            .json::<GoogleImageResult>()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(data.items[0].link.clone())
    }
}
