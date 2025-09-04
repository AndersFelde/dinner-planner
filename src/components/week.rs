use crate::{app::RouteUrl, models::day::DayWithMeal};
use leptos::logging::log;
use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_query};

use crate::components::day_preview::*;

use chrono::{Datelike, Local, NaiveDate, Weekday};

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub struct Week {
    pub week: u32,
    pub year: i32,
}

fn dates_for_week(week_num: u32, year: i32) -> [NaiveDate; 7] {
    (0..7)
        .filter_map(|i| {
            let weekday = match i {
                0 => Weekday::Mon,
                1 => Weekday::Tue,
                2 => Weekday::Wed,
                3 => Weekday::Thu,
                4 => Weekday::Fri,
                5 => Weekday::Sat,
                6 => Weekday::Sun,
                _ => unreachable!(),
            };
            NaiveDate::from_isoywd_opt(year, week_num, weekday)
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

#[server]
pub async fn days_for_week(_week: Week) -> Result<[DayWithMeal; 7], ServerFnError> {
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
    let week_dates = dates_for_week(_week.week, _week.year);
    let days_query = days::table
        .filter(days::week.eq(_week.week as i32))
        .filter(days::year.eq(_week.year as i32));
    let num_days: i64 = days_query.count().get_result(db).unwrap();
    let mut days = vec![];

    if num_days < 7 {
        for _date in week_dates {
            match days::table
                .filter(days::date.eq(_date))
                .first::<Day>(db)
                .optional()
                .unwrap()
            {
                Some(_) => continue,
                None => {
                    let day_form = DayForm {
                        date: _date,
                        week: _date.iso_week().week() as i32,
                        year: _date.year(),
                        meal_id: None,
                    };
                    insert_into(days::table)
                        .values(&day_form)
                        .execute(db)
                        .unwrap();
                }
            }
        }
    }
    let days_rows: Vec<Day> = days_query
        .select(Day::as_select())
        .load(db)
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;
    assert_eq!(days_rows.len(), 7);
    for day in days_rows {
        if let Some(_meal_id) = day.meal_id {
            let meal: Meal = meals::table.filter(meals::id.eq(_meal_id)).first(db)?;

            let ingredients: Vec<Ingredient> = ingredients::table
                .filter(ingredients::meal_id.eq(_meal_id))
                .load(db)?;
            days.push(DayWithMeal {
                day,
                meal: Some(MealWithIngredients { meal, ingredients }),
            });
        } else {
            days.push(DayWithMeal { day, meal: None })
        }
    }
    Ok(days
        .try_into()
        .map_err(|_| ServerFnError::new("Expected 7 days in week"))?)
}

impl Week {
    pub fn new(week: u32, year: i32) -> Week {
        // let days = days_for_week(week, year).await.unwrap();
        // let d = &days[0].day;
        Week {
            week: week as u32,
            year: year,
            // days: days,
        }
    }
    pub fn current() -> Week {
        let w = Local::now().date_naive().iso_week();
        Week::new(w.week(), w.year())
    }

    pub fn next(self, n: i32) -> Week {
        let mut week = self.week as i32 + n;
        let mut year = self.year;
        let weeks_in_year = NaiveDate::from_isoywd_opt(year, 53, Weekday::Mon).map_or(52, |_| 53);

        // Wrap week and year forward/backward
        while week > weeks_in_year {
            week -= weeks_in_year;
            year += 1;
            let weeks_in_next_year =
                NaiveDate::from_isoywd_opt(year, 53, Weekday::Mon).map_or(52, |_| 53);
            week = week.min(weeks_in_next_year);
        }
        while week < 1 {
            year -= 1;
            let weeks_in_prev_year =
                NaiveDate::from_isoywd_opt(year, 53, Weekday::Mon).map_or(52, |_| 53);
            week += weeks_in_prev_year;
        }

        Week::new(week as u32, year)
    }
}
use leptos::Params;
use leptos_router::params::Params;
#[derive(Params, PartialEq, Clone)]
pub struct WeekQuery {
    pub week: u32,
    pub year: i32,
}
#[component]
pub fn Week() -> impl IntoView {
    let query = use_query::<WeekQuery>();

    let (week, set_week) = signal(Week::current());
    Effect::new(move || {
        if let Some(query) = query.read().as_ref().ok() {
            set_week(Week::new(query.week, query.year));
        }
    });
    let days_resource = Resource::new(move || week.get(), |week| days_for_week(week));
    let days_data = move || {
        days_resource.get().map(|val| {
            val.unwrap()
                .iter()
                .map(|day| {
                    view! { <DayPreview day=day.clone() /> }
                })
                .collect::<Vec<_>>()
        })
    };

    view! {
        <A href=RouteUrl::MealList.to_string()>
            <button
                type="button"
                class="fixed bottom-19 right-4 z-50 px-4 py-3 rounded-full bg-blue-500 text-white font-semibold text-base shadow-lg hover:bg-green-600 focus:outline-none focus:ring-2 focus:ring-green-400 transition flex items-center justify-center whitespace-nowrap"
                title="View shopping list"
            >
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
                        d="M11.42 15.17 17.25 21A2.652 2.652 0 0 0 21 17.25l-5.877-5.877M11.42 15.17l2.496-3.03c.317-.384.74-.626 1.208-.766M11.42 15.17l-4.655 5.653a2.548 2.548 0 1 1-3.586-3.586l6.837-5.63m5.108-.233c.55-.164 1.163-.188 1.743-.14a4.5 4.5 0 0 0 4.486-6.336l-3.276 3.277a3.004 3.004 0 0 1-2.25-2.25l3.276-3.276a4.5 4.5 0 0 0-6.336 4.486c.091 1.076-.071 2.264-.904 2.95l-.102.085m-1.745 1.437L5.909 7.5H4.5L2.25 3.75l1.5-1.5L7.5 4.5v1.409l4.26 4.26m-1.745 1.437 1.745-1.437m6.615 8.206L15.75 15.75M4.867 19.125h.008v.008h-.008v-.008Z"
                    />
                </svg>

            </button>
        </A>
        <A href=move || {
            let w = week.get();
            RouteUrl::ShoppingList {
                week: w.week,
                year: w.year,
            }
                .to_string()
        }>
            <button
                type="button"
                class="fixed bottom-4 right-4 z-50 px-4 py-3 rounded-full bg-blue-500 text-white font-semibold text-base shadow-lg hover:bg-green-600 focus:outline-none focus:ring-2 focus:ring-green-400 transition flex items-center justify-center whitespace-nowrap"
                title="View shopping list"
            >
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
                        d="M8.25 6.75h12M8.25 12h12m-12 5.25h12M3.75 6.75h.007v.008H3.75V6.75Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0ZM3.75 12h.007v.008H3.75V12Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm-.375 5.25h.007v.008H3.75v-.008Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Z"
                    />
                </svg>

            </button>
        </A>
        <div class="flex flex-col gap-4">
            <div class="flex justify-center items-center gap-4 mb-2 sticky top-0 z-10 bg-white dark:bg-gray-800 py-2 shadow">
                <button
                    class="w-32 px-3 py-2 rounded-lg bg-blue-500 text-white font-semibold text-base shadow hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-400 transition flex items-center justify-center whitespace-nowrap"
                    on:click=move |_| set_week.update(|w| *w = w.clone().next(-1))
                    title="Previous week"
                >
                    <svg
                        class="w-4 h-4 mr-2"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        viewBox="0 0 24 24"
                    >
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
                    </svg>
                    Previous
                </button>
                <span class="font-bold text-base text-gray-900 dark:text-white">
                    Week {move || format!("{}", week.get().week)}
                </span>
                <button
                    class="w-32 px-3 py-2 rounded-lg bg-blue-500 text-white font-semibold text-base shadow hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-400 transition flex items-center justify-center whitespace-nowrap"
                    on:click=move |_| set_week.update(|w| *w = w.clone().next(1))
                    title="Next week"
                >
                    Next
                    <svg
                        class="w-4 h-4 ml-2"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        viewBox="0 0 24 24"
                    >
                        <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7" />
                    </svg>
                </button>
            </div>
            // Centered vertical card list
            // TODO: add "Get shopping list button"
            <div class="flex flex-col gap-4 py-2 items-center">
                <Transition fallback=move || {
                    view! { <p>"Loading..."</p> }
                }>{move || days_data}</Transition>

            </div>
        </div>
    }
}
