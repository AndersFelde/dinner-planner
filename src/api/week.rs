use crate::models::day::{DayForm};
use crate::models::days_ingredients::{DayWithMealAndIngredients, IngredientWithBought};
use chrono::{Datelike, Local, NaiveDate, Weekday};
use leptos::prelude::*;
use leptos::logging::log;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub struct Week {
    pub week: u32,
    pub year: i32,
}

impl Week {
    pub fn new(week: u32, year: i32) -> Week {
        Week {
            week: week as u32,
            year: year,
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
    pub fn dates(&self) -> [NaiveDate; 7] {
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
                NaiveDate::from_isoywd_opt(self.year, self.week, weekday)
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }
}

#[server]
pub async fn days_for_week(week: Week) -> Result<[DayWithMealAndIngredients; 7], ServerFnError> {
    use crate::api::ssr::*;
    let db = &mut get_db()?;
    let days_query = days::table
        .filter(days::week.eq(week.week as i32))
        .filter(days::year.eq(week.year as i32));
    let num_days: i64 = server_err!(
        days_query.count().get_result(db),
        "Could not get days for week {week:?}"
    )?;
    let mut days = vec![];

    if num_days < 7 {
        for date in week.dates() {
            if server_err!(
                days::table
                    .filter(days::date.eq(date))
                    .first::<Day>(db)
                    .optional(),
                "Could not get day for date {date}",
            )?
            .is_none()
            {
                let day_form = DayForm {
                    date: date,
                    week: date.iso_week().week() as i32,
                    year: date.year(),
                    meal_id: None,
                };
                server_err!(
                    insert_into(days::table).values(&day_form).execute(db),
                    "Could not create day {day_form:?}"
                )?;
            }
        }
    }
    let days_rows = server_err!(
        days_query
            .left_join(meals::table)
            .select((Day::as_select(), Option::<Meal>::as_select()))
            .load::<(Day, Option<Meal>)>(db),
        "Could not get days for week {week:?}"
    )?;
    if days_rows.len() != 7 {
        return Err(ServerFnError::new(format!(
            "Only got {} days. How is this possible?",
            days_rows.len()
        )));
    }

    for (day, meal) in days_rows {
        if let Some(meal) = meal {
            let ingredients = server_err!(
                DayIngredient::belonging_to(&day)
                    .inner_join(ingredients::table)
                    .select((DayIngredient::as_select(), Ingredient::as_select()))
                    .load::<(DayIngredient, Ingredient)>(db),
                "Could not get ingredeints for {day:?}"
            )?
            .into_iter()
            .map(|(di, ingredient)| IngredientWithBought {
                day_id: di.day_id,
                ingredient,
                bought: di.bought,
            })
            .collect();
            days.push(
                DayWithMealAndIngredients {
                    day: day,
                    meal: Some((meal, ingredients)),
                },
            );
        } else {
            days.push(DayWithMealAndIngredients { day: day, meal: None })
        }
    }
    Ok(days
        .try_into()
        .map_err(|_| ServerFnError::new("Expected 7 days in week"))?)
}