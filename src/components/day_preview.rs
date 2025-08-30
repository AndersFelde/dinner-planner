use crate::models::day::*;
use crate::models::ingredient::*;
use crate::models::meal::*;
use chrono::{Datelike, Local, NaiveDate, Weekday};
use leptos::{
    either::Either,
    html::{Div, HtmlElement, Input},
    prelude::*,
};
use web_sys::ScrollIntoViewOptions;
// #[derive(Clone, Debug)]
// pub struct Ingredient {
//     pub name: String,
//     pub amount: u32,
//     pub bought: bool,
// }

// impl Ingredient {
//     pub fn new_with_default(name: String) -> Self {
//         Self {
//             name,
//             amount: 2,
//             bought: true,
//         }
//     }
// }

// #[derive(Clone, Debug)]
// pub struct Meal {
//     pub name: String,
//     pub image: Option<String>,
//     pub ingredients: Vec<Ingredient>,
// }

// #[derive(Clone, Debug)]
// pub struct Day {
//     pub date: NaiveDate,
//     pub meal: Option<Meal>,
// }

// impl Day {
//     pub fn header(&self) -> String {
//         format!(
//             "{} - {:02}.{:02}",
//             self.date.weekday(),
//             self.date.day(),
//             self.date.month()
//         )
//     }
// }
#[component]
pub fn IngredientBox(ingredient: Ingredient) -> impl IntoView {
    let box_classes = if ingredient.bought {
        "inline-block px-3 py-1 m-1 rounded-full bg-green-200 text-green-900 border border-green-400 shadow-sm text-sm font-medium"
    } else {
        "inline-block px-3 py-1 m-1 rounded-full bg-red-100 text-red-900 border border-red-300 shadow-sm text-sm font-medium"
    };

    let label = if ingredient.amount > 1 {
        format!("{} x{}", ingredient.name, ingredient.amount)
    } else {
        ingredient.name.clone()
    };

    view! { <span class=box_classes>{label}</span> }
}

#[component]
pub fn DayPreview(day: DayWithMeal) -> impl IntoView {
    // Creates a reactive value to update the button
    // let weekday = "Wednesday";
    // let date = "07.08";
    // let image_url = "https://images.stream.schibsted.media/users/vgtv/images/25d79ba4fa299a22055f4a0d930d9052.jpg?t[]=1440q80";
    // let meal_name = "Pasta Carbonara";
    // let ingredients = vec!["Pasta", "egg", "bacon"];
    let today = Local::now().date_naive();
    let node_ref = NodeRef::<Div>::new();
    let is_today = today == day.day.date;
    if is_today {
        Effect::new(move || {
            if let Some(node) = node_ref.get() {
                let options = ScrollIntoViewOptions::new();
                options.set_behavior(web_sys::ScrollBehavior::Auto);
                options.set_block(web_sys::ScrollLogicalPosition::Center);
                options.set_inline(web_sys::ScrollLogicalPosition::Center);
                node.scroll_into_view_with_scroll_into_view_options(&options);
            }
        });
    }
    let card_classes = if is_today {
        "w-80 max-w-sm bg-white border-4 rounded-xl shadow-lg border-gray-200 rounded-xl shadow-sm dark:bg-gray-800 dark:border-blue-500 flex flex-col transition-all duration-300"
    } else {
        "w-80 max-w-sm bg-white border border-gray-200 rounded-xl shadow-sm dark:bg-gray-800 dark:border-gray-700 flex flex-col transition-all duration-300"
    };

    let header = format!(
        "{} - {:02}.{:02}",
        day.day.date.weekday(),
        day.day.date.day(),
        day.day.date.month()
    );
    match day.meal {
        Some(meal) => Either::Left(view! {
            <div class=card_classes node_ref=node_ref>
                // Header
                <div class="p-4 border-b border-gray-200 dark:border-gray-700">
                    <h4 class="text-lg font-semibold text-gray-900 dark:text-white">{header}</h4>
                    <h5 class="text-xl font-bold text-blue-700 dark:text-blue-400 font-underline">
                        {meal.meal.name.clone()}
                    </h5>
                </div>
                // Image
                <img
                    class="w-full h-48 object-cover rounded-b-none rounded-t-lg"
                    src=meal.meal.image
                    alt=meal.meal.name
                />
                // Footer: Ingredients
                <div class="p-4 border-t border-gray-200 dark:border-gray-700 mt-auto">
                    <h6 class="text-md font-semibold text-gray-900 dark:text-white mb-2">
                        Ingredients
                    </h6>
                    <div class="flex flex-wrap gap-1">
                        {meal
                            .ingredients
                            .into_iter()
                            .map(|ingredient| view! { <IngredientBox ingredient=ingredient /> })
                            .collect::<Vec<_>>()}
                    </div>
                </div>
            </div>
        }),
        None => Either::Right(view! {
            <div class=card_classes node_ref=node_ref>
                // Header
                <div class="p-4 border-b border-gray-200 dark:border-gray-700">
                    <h4 class="text-lg font-semibold text-gray-900 dark:text-white">{header}</h4>
                </div>
                // Image area with big "+" button
                <div class="w-full h-48 flex items-center justify-center bg-gray-100 dark:bg-gray-800 rounded-b-none rounded-t-lg">
                    <button
                        class="text-6xl text-gray-400 dark:text-blue-400 hover:text-blue-500 dark:hover:text-blue-300 transition-colors bg-white dark:bg-gray-900 rounded-full w-20 h-20 flex items-center justify-center shadow-lg border-2 border-gray-300 dark:border-gray-700"
                        title="Add meal"
                    >
                        "+"
                    </button>
                </div>
            // Footer: Ingredients (empty)
            </div>
        }),
    }
}
