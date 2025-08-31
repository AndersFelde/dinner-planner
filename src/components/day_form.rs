use crate::app::RouteUrl;
use crate::models::day::{Day, DayForm};
use crate::models::meal::Meal;
use chrono::{Datelike, Local, NaiveDate};
use leptos::logging::log;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_navigate;
use leptos_router::hooks::use_params_map;

#[server]
pub async fn create_day_with_meal(day_form: DayForm) -> Result<usize, ServerFnError> {
    use crate::db::*;
    use crate::schema::days;
    use diesel::dsl::insert_into;
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
    use crate::models::meal::*;
    use crate::schema::meals;
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
#[server]
pub async fn get_day(id: i32) -> Result<Day, ServerFnError> {
    use crate::db::*;
    use crate::models::day::*;
    use crate::schema::days;
    use diesel::prelude::*;

    let db = &mut use_context::<Db>()
        .ok_or(ServerFnError::new("Missing Db context"))?
        .get()
        .map_err(|_| ServerFnError::new("Failed to get Db connection"))?;
    days::table
        .filter(days::id.eq(id))
        .first::<Day>(db)
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))
}
// TODO: if date allready exists, edit
#[component]
pub fn DayForm() -> impl IntoView {
    log!("Loading day form");
    let params = use_params_map();
    let (date, set_date) = signal(String::new());
    let (meal_id, set_meal_id) = signal(-1 as i32);
    let navigate = use_navigate();

    let day_resource = Resource::new(
        move || {
            params
                .read()
                .get("id")
                .and_then(|id| id.parse::<i32>().ok())
        },
        move |id| async move {
            match id {
                Some(id) => get_day(id).await.map(Some),
                None => Ok(None),
            }
        },
    );

    let meals_resource = OnceResource::new(get_all_meals());

    // Effect to redirect after success

    // let day = Resource::new(move || day.get(), move |day| async move { get_day(id).await });
    Effect::new({
        let set_meal_id = set_meal_id.clone();
        let set_date = set_date.clone();
        move || {
            let day_resource = day_resource.get();
            if let Some(Ok(Some(day))) = day_resource.clone() {
                if let Some(id) = day.meal_id {
                    set_meal_id(id);
                } else if let Some(Ok(meals)) = meals_resource.get() {
                    if let Some(first_meal) = meals.first() {
                        set_meal_id(first_meal.id);
                    }
                }
            }
            if let Some(Ok(Some(day))) = day_resource {
                set_date(day.date.format("%d-%m-%Y").to_string());
            }
        }
    });
    let meals_data = move || {
        if let Some(Ok(Some(day))) = day_resource.get() {
            meals_resource.get().map(|val| {
                val.unwrap()
                .iter()
                .map(|meal| {
                    view! {
                        <option selected=day.meal_id.unwrap_or_else(|| -1) == meal.id value=meal.id>
                            {meal.name.clone()}
                        </option>
                    }
                })
                .collect::<Vec<_>>()
            })
        } else {
            None
        }
    };
    let add_day_action = Action::new(|day_form: &DayForm| {
        let day_form = day_form.clone();
        async move { create_day_with_meal(day_form).await }
    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let date = NaiveDate::parse_from_str(&date.get(), "%d-%m-%Y").unwrap();
        let meal_id = meal_id.get();
        let day_form = DayForm {
            date,
            meal_id: Some(meal_id),
            week: date.iso_week().week() as i32,
            year: date.year(),
        };
        add_day_action.dispatch(day_form);
        log!("add day action {:?}", add_day_action.value().get());
    };

    Effect::new(move || {
        log!("Check redirect");
        if let Some(Ok(_)) = add_day_action.value().get() {
            navigate(&RouteUrl::Home.to_string(), Default::default());
        }
    });

    view! {
        <div class="max-w-lg mx-auto mt-8 p-6 bg-white dark:bg-gray-900 rounded-xl shadow-lg">
            <A href=RouteUrl::Home attr:class="text-blue-500 hover:underline mb-4 inline-block">
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

            </A>
            <form on:submit=on_submit class="space-y-6">
                <h2 class="font-bold text-2xl mb-4 text-gray-900 dark:text-white text-center">
                    Create Day
                </h2>
                <div class="space-y-3">
                    <label
                        for="date-input"
                        class="block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1 text-left"
                    >
                        Date
                    </label>
                    <input
                        id="date-input"
                        type="text"
                        placeholder="Date"
                        prop:value=date
                        on:input=move |ev| set_date(event_target_value(&ev))
                        class="w-full px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-400 dark:bg-gray-800 dark:text-white"
                        required
                    />
                    <label
                        for="meal-select"
                        class="block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1 text-left"
                    >
                        Meal
                    </label>
                    <Transition fallback=move || {
                        view! { <span>"Loading..."</span> }
                    }>
                        <select
                            id="meal-select"
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

                <A
                    href=RouteUrl::NewMeal
                    attr:class="w-full py-2 bg-blue-100 text-blue-700 rounded-lg hover:bg-blue-200 transition mb-2 flex items-center justify-center font-semibold"
                >
                    "+ Add Meal"
                </A>
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
