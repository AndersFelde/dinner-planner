use crate::models::ingredient::IngredientForm;
use crate::models::meal::{Meal, MealForm, MealWithIngredients};
use leptos::prelude::*;

#[server]
pub async fn get_meals() -> Result<Vec<Meal>, ServerFnError> {
    use crate::api::ssr::*;
    let db = &mut get_db()?;
    server_err!(
       Meal::get_all(db) ,
        "Could not get meals"
    )
}

#[server]
pub async fn get_meal(id: i32) -> Result<MealWithIngredients, ServerFnError> {
    use crate::api::ssr::*;
    let db = &mut get_db()?;

    let meal = server_err!(
        Meal::get(db, id),
        "Could not get meal with id {id}"
    )?;
    let ingredients = server_err!(
        Ingredient::get_for_meal(db, meal.id),
        "Could not get ingredients for meal_id {id}"
    )?;
    Ok(MealWithIngredients { meal, ingredients })
}

#[server]
pub async fn delete_meal(id: i32) -> Result<usize, ServerFnError> {
    use crate::api::ssr::*;
    let db = &mut get_db()?;
    server_err!(
        Meal::delete(db, id),
        "Could not get meal with id {id}"
    )
}

#[server]
pub async fn update_meal_with_ingredients(
    meal: Meal,
    ingredient_forms: Vec<IngredientForm>,
) -> Result<(), ServerFnError> {
    use crate::api::ingredient::{delete_ingredients, insert_ingredient};
    use crate::api::days_ingredients::insert_day_ingredient;
    use crate::api::day::get_days_for_meal;
    use crate::api::ssr::*;
    let db = &mut get_db()?;
    server_err!(
        meal.update(db),
        "Could not update meal {meal:?}"
    )?;
    // This is kinda hacky, but easy
    delete_ingredients(db, meal.id)?;
    let meal_days = get_days_for_meal(meal.id).await?;
    for mut ingredient_form in ingredient_forms {
        ingredient_form.meal_id = meal.id;
        let ingredient = insert_ingredient(db, ingredient_form)?;
        for day in meal_days.iter() {
            insert_day_ingredient(DayIngredient {
                day_id: day.id,
                ingredient_id: ingredient.id,
                bought: false,
            }).await?;
        }
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
        meal_form.insert(db),
        "Could not insert meal {meal_form:?}"
    )?;
    for mut ingredient_form in ingredient_forms {
        ingredient_form.meal_id = meal.id;
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

#[cfg(feature = "ssr")]
#[cfg(test)]
mod test {
    use super::*;
    use crate::db::tests::TEST_POOL;
    use crate::models::meal::MealForm;
    use diesel::Connection;

    #[test]
    pub fn test_meals_all() {
        let db = &mut TEST_POOL.clone().get().unwrap();
        db.test_transaction(|db| -> Result<(), ()> {
            let mut meal = MealForm {
                name: String::new(),
                image: String::new(),
                recipie_url: None,
            }
            .insert(db)
            .unwrap();
            let meal_other = MealForm {
                name: String::new(),
                image: String::new(),
                recipie_url: None,
            }
            .insert(db)
            .unwrap();

            assert_eq!(Meal::get_all(db).unwrap().len(), 2);
            assert_eq!(meal, Meal::get(db, meal.id).unwrap());
            meal.recipie_url = Some(String::new());
            let meal = meal.update(db).unwrap();
            assert_ne!(Meal::get(db, meal.id).unwrap().recipie_url, Meal::get(db, meal_other.id).unwrap().recipie_url);
            Meal::delete(db, meal_other.id).unwrap();
            assert_eq!(Meal::get_all(db).unwrap().len(), 1);
            assert_eq!(Meal::get_all(db).unwrap(), vec![meal]);

            Ok(())
        });
    }
}