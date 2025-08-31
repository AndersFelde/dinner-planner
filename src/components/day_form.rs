use crate::app::RouteUrl;
use crate::models::day::DayForm;
use crate::models::ingredient::IngredientForm;
use crate::models::meal::Meal;
use chrono::{Datelike, Local, NaiveDate};
use leptos::prelude::*;
use leptos::logging::log;
use leptos_router::hooks::use_navigate;

#[server]
pub async fn create_day_with_meal(day_form: DayForm) -> Result<usize, ServerFnError> {
    use crate::db::*;
    use crate::models::day::*;
    use crate::models::ingredient::*;
    use crate::models::meal::*;
    use crate::schema::days;
    use crate::schema::ingredients;
    use crate::schema::meals;
    use diesel::dsl::{insert_into, update};
    use diesel::prelude::*;

    let db = &mut use_context::<Db>()
        .ok_or(ServerFnError::new("Missing Db context"))?
        .get()
        .map_err(|_| ServerFnError::new("Failed to get Db connection"))?;
    insert_into(days::table)
        .values(&day_form)
        .on_conflict(days::date)
        .do_update()
        .set(&day_form)
        .execute(db)
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))
}

#[server]
pub async fn get_all_meals() -> Result<Vec<Meal>, ServerFnError> {
    use crate::db::*;
    use crate::models::day::*;
    use crate::models::ingredient::*;
    use crate::models::meal::*;
    use crate::schema::days;
    use crate::schema::ingredients;
    use crate::schema::meals;
    use diesel::dsl::insert_into;
    use diesel::prelude::*;

    let db = &mut use_context::<Db>()
        .ok_or(ServerFnError::new("Missing Db context"))?
        .get()
        .map_err(|_| ServerFnError::new("Failed to get Db connection"))?;
    meals::table
        .select(Meal::as_select())
        .load(db)
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))
}
// TODO: if date allready exists, edit
#[component]
pub fn DayForm() -> impl IntoView {
    // Signals for meal fields
    let (date, set_date) = signal(Local::now().format("%d-%m-%Y").to_string());
    let (meal_id, set_meal_id) = signal(0 as i32);

let navigate = use_navigate();

let add_day_action = Action::new(|day_form: &DayForm| {
    let day_form = day_form.clone();
    async move { create_day_with_meal(day_form).await }
});

// Effect to redirect after success
Effect::new(move || {
    if let Some(Ok(_)) = add_day_action.value().get() {
        navigate(&RouteUrl::Week.to_string(), Default::default());
    }
});

    // Handle form submission (pseudo-code, replace with your server call)
    let on_submit = move |_| {
        let date = NaiveDate::parse_from_str(&date.get(), "%d-%m-%Y").unwrap();
        let meal_id = meal_id.get();
        let day_form = DayForm {
            date,
            meal_id: Some(meal_id),
            week: date.iso_week().week() as i32,
            year: date.year(),
        };
        add_day_action.dispatch(day_form);
    };

    let meals_resource = Resource::new(move || {}, |_| get_all_meals());
    Effect::new({
        let set_meal_id = set_meal_id.clone();
        move || {
            if let Some(Ok(meals)) = meals_resource.get() {
                if let Some(first_meal) = meals.first() {
                    set_meal_id(first_meal.id);
                }
            }
        }
    });
    let meals_data = move || {
        meals_resource.get().map(|val| {
            val.unwrap()
                .iter()
                .map(|meal| {
                    view! { <option value=meal.id>{meal.name.clone()}</option> }
                })
                .collect::<Vec<_>>()
        })
    };

    view! {
        <div class="max-w-lg mx-auto mt-8 p-6 bg-white dark:bg-gray-900 rounded-xl shadow-lg">
            <a href=RouteUrl::Week class="text-blue-500 hover:underline mb-4 inline-block">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke-width="1.5"
                    stroke="currentColor"
                    class="size-6"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="M10.5 19.5 3 12m0 0 7.5-7.5M3 12h18"
                    />
                </svg>

            </a>
            <form on:submit=on_submit class="space-y-6">
                <h2 class="font-bold text-2xl mb-4 text-gray-900 dark:text-white text-center">
                    Create Day
                </h2>
                <div class="space-y-3">
                    <input
                        type="text"
                        placeholder="Date"
                        prop:value=date
                        on:input=move |ev| set_date(event_target_value(&ev))
                        class="w-full px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-400 dark:bg-gray-800 dark:text-white"
                        required
                    />
                    <Transition fallback=move || {
                        view! { <span>"Loading..."</span> }
                    }>
                        <select
                            prop:value=meal_id
                            on:change=move |ev| set_meal_id(
                                event_target_value(&ev).parse().unwrap(),
                            )
                            class="w-full px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-400 dark:bg-gray-800 dark:text-white"
                            required
                        >
                            {move || meals_data}
                        </select>
                    </Transition>
                </div>

                <a
                    href=RouteUrl::MealForm
                    class="w-full py-2 bg-blue-100 text-blue-700 rounded-lg hover:bg-blue-200 transition mb-2 flex items-center justify-center font-semibold"
                >
                    "+ Add Meal"
                </a>
                <button
                    type="submit"
                    class="w-full py-2 bg-blue-500 text-white font-semibold rounded-lg hover:bg-blue-600 transition"
                >
                    "Create Day"
                </button>
            </form>
        </div>
    }
}
