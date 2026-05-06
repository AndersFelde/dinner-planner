use crate::api::day::update_attendance_for_day;
use crate::models::day::Day;
use leptos::prelude::*;

use crate::components::toasts::ToastStore;

#[component]
pub fn Attendance<'a>(day: &'a Day) -> impl IntoView {
    let anders_attend = RwSignal::new(day.anders_attend);
    let ac_attend = RwSignal::new(day.ac_attend);
    let andreas_attend = RwSignal::new(day.andreas_attend);
    let day_id = day.id;

    let update_attendance_action = Action::new(move |_| {
        update_attendance_for_day(
            day_id,
            anders_attend.get(),
            ac_attend.get(),
            andreas_attend.get(),
        )
    });
    let toast_store = expect_context::<ToastStore>();
    let update_attendance_value = update_attendance_action.clone();
    Effect::new(move || {
        if let Some(Err(err)) = update_attendance_value.value().get() {
            toast_store.push_error(err.to_string());
        }
    });
    let default_class =
        "inline-block px-1 py-1 rounded-full border shadow-sm text-sm font-sm min-w-[50px] text-center";
    let green_class = "bg-green-200 text-green-900 border-green-400";
    let red_class = "bg-red-100 text-red-900 border-red-300 line-through";
    // THIS SUCKS!
    view! {
        <span
            class=move || {
                format!(
                    "{default_class} {}",
                    if anders_attend.get() { green_class } else { red_class },
                )
            }
            on:click=move |_| {
                anders_attend.update(|a| *a = !*a);
                update_attendance_action.dispatch(());
            }
        >
            {"Anders"}
        </span>
        <span
            class=move || {
                format!("{default_class} {}", if ac_attend.get() { green_class } else { red_class })
            }
            on:click=move |_| {
                ac_attend.update(|a| *a = !*a);
                update_attendance_action.dispatch(());
            }
        >
            {"AC"}
        </span>
        <span
            class=move || {
                format!(
                    "{default_class} {}",
                    if andreas_attend.get() { green_class } else { red_class },
                )
            }
            on:click=move |_| {
                andreas_attend.update(|a| *a = !*a);
                update_attendance_action.dispatch(());
            }
        >
            {"Andreas"}
        </span>
    }
}
