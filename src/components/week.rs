use crate::models::day::DayWithMeal;
use leptos::logging::log;
use leptos::prelude::*;

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
    let mut a = vec![];

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
                    log!(
                        "Inserting day: {}",
                        insert_into(days::table)
                            .values(&day_form)
                            .execute(db)
                            .unwrap()
                    )
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
            a.push(DayWithMeal {
                day,
                meal: Some(MealWithIngredients { meal, ingredients }),
            });
        } else {
            a.push(DayWithMeal { day, meal: None })
        }
    }
    Ok(a.try_into()
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

#[component]
pub fn Week() -> impl IntoView {
    let (week, set_week) = signal(Week::current());
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
        <div class="flex flex-col gap-4">
            // Sticky navigation bar with nice SVG arrows
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
            <div class="flex flex-col gap-4 py-2 items-center">
                <Transition fallback=move || {
                    view! { <p>"Loading..."</p> }
                }>{move || days_data}</Transition>

            </div>
        </div>
    }
}
