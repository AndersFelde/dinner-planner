use leptos::prelude::*;

use crate::components::day_preview::*;

#[derive(Clone, Debug)]
pub struct Day {
    pub date: String,
    pub weekday: String,
    pub meal_name: String,
    pub image_url: String,
    pub ingredients: Vec<String>,
}

use chrono::{Datelike, Local, NaiveDate, Weekday};

#[derive(Clone, Debug)]
struct Week {
    week: u32,
    year: i32,
    days: [Day; 7],
}

impl Week {
    pub fn new(week: u32, year: i32) -> Week {
        Week {
            week,
            year,
            days: Week::days_for_week(year, week),
        }
    }
    pub fn current() -> Week {
        let w = Local::now().date_naive().iso_week();
        Week::new(w.week(), w.year())
    }

    fn days_for_week(year: i32, week: u32) -> [Day; 7] {
        Week::dates_for_week(year, week).iter().map(|d| Day {
            date: format!("{:02}.{:02}", d.day(), d.month()),
            weekday: format!("{}", d.weekday()),
            meal_name: "Pasta Carbonara".to_string(),
            image_url: "https://images.stream.schibsted.media/users/vgtv/images/25d79ba4fa299a22055f4a0d930d9052.jpg?t[]=1440q80".to_string(),
            ingredients: vec![String::from("Pasta"), String::from("Egg"), String::from("Bacon")]
        })
                    .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    fn dates_for_week(year: i32, week_num: u32) -> [NaiveDate; 7] {
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
    fn next(self, n: i32) -> Week {
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

    view! {
        <div class="flex flex-col gap-4">
            <div class="flex justify-between items-center mb-2">
                <button
                    class="px-4 py-2 bg-gray-200 rounded"
                    on:click=move |_| set_week.update(|w| *w = w.clone().next(-1))
                >
                    "Previous"
                </button>
                <span class="font-bold text-lg">Week {move || format!("{} ({})", week.get().week, week.get().year)}</span>
                <button
                    class="px-4 py-2 bg-gray-200 rounded"
                    on:click=move |_| set_week.update(|w| *w = w.clone().next(1))
                >
                    "Next"
                </button>
            </div>
            <div class="flex flex-row gap-4 overflow-x-auto">
                {move || week
                    .get()
                    .days
                    .iter()
                    .map(|day| {
                        view! {
                            <DayPreview
                                date=day.date.clone()
                                weekday=day.weekday.clone()
                                meal_name=day.meal_name.clone()
                                image_url=day.image_url.clone()
                                ingredients=day.ingredients.clone()
                            />
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>
        </div>
    }
}
