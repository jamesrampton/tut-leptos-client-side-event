use leptos::*;

fn main() {
    let cat_names_func = vec!["Beans", "Basil", "Oliver", "Funcy"];
    let cat_names = vec!["Beans", "Basil", "Oliver", "Compy"];

    mount_to_body(|cx| {
        view! { cx,
            <h1>"Great cat names (func)"</h1>
            <ul>
                {list_names_func(cx,cat_names_func)}
            </ul>
            <ListNames cat_names />
        }
    })
}

fn list_names_func(cx: Scope, cat_names: Vec<&'static str>) -> Vec<View> {
    cat_names
        .into_iter()
        .map(move |name| view! {cx, <li>{name}</li>})
        .map(move |li| li.into_view(cx))
        .collect()
}

#[component]
fn ListNames(cx: Scope, cat_names: Vec<&'static str>) -> impl IntoView {
    let list_items: Vec<_> = cat_names
        .into_iter()
        .map(move |name| view! {cx, <li>{name}</li>})
        .collect();

    view! {cx,
        <h1>"Great cat names (component)"</h1>
        <ul>
            {list_items}
        </ul>
    }
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
