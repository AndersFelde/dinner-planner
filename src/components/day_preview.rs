use leptos::prelude::*;

#[component]
pub fn DayPreview(
    date: String,
    weekday: String,
    meal_name: String,
    image_url: String,
    ingredients: Vec<String>,
) -> impl IntoView {
    // Creates a reactive value to update the button
    // let weekday = "Wednesday";
    // let date = "07.08";
    // let image_url = "https://images.stream.schibsted.media/users/vgtv/images/25d79ba4fa299a22055f4a0d930d9052.jpg?t[]=1440q80";
    // let meal_name = "Pasta Carbonara";
    // let ingredients = vec!["Pasta", "egg", "bacon"];
    view! {
        <div class="max-w-sm bg-white border border-gray-200 rounded-lg shadow-sm dark:bg-gray-800 dark:border-gray-700 flex flex-col">
            // Header
            <div class="p-4 border-b border-gray-200 dark:border-gray-700">
                <h4 class="text-lg font-semibold text-gray-900 dark:text-white">
                    {weekday}- {date}
                </h4>
                <h5 class="text-xl font-bold text-blue-700 dark:text-blue-400">{meal_name.clone()}</h5>
            </div>
            // Image
            <img
                class="w-full h-48 object-cover rounded-b-none rounded-t-lg"
                src=image_url
                alt=meal_name
            />
            // Footer: Ingredients
            <div class="p-4 border-t border-gray-200 dark:border-gray-700 mt-auto">
                <h6 class="text-md font-semibold text-gray-900 dark:text-white mb-2">
                    Ingredients
                </h6>
                <ul class="list-disc list-inside text-gray-700 dark:text-gray-400">
                    {ingredients
                        .into_iter()
                        .map(|ingredient| view! { <li>{ingredient}</li> })
                        .collect::<Vec<_>>()}
                </ul>
            </div>
        </div>
    }
}
