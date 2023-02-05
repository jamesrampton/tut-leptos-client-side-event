use leptos::*;

fn main() {
    let cat_names = vec!["Beans", "Basil", "Oliver"];

    mount_to_body(|cx| {
        view! { cx,
            <h1>"Great cat names"</h1>
            <ul>
                {list_names(cx,cat_names)}
            </ul>
        }
    })
}

fn list_names(cx: Scope, cat_names: Vec<&'static str>) -> Vec<View> {
    cat_names
        .into_iter()
        .map(move |name| view! {cx, <li>{name}</li>})
        .map(move |li| li.into_view(cx))
        .collect()
}

// #[component]
// pub fn NiceAffirmation(cx: Scope) -> impl IntoView {
//     view! {
//         cx,
//         <p>"You look nice today!"</p>
//     }
// }
// #[component]
// fn LuckyNumber(cx: Scope, the_lucky_number: i32) -> impl IntoView {
//     view! {
//         cx,
//         <p>"Today's lucky number is " {the_lucky_number}</p>
//     }
// }
