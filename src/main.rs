use leptos::*;

#[derive(Clone)]
struct Cat {
    name: String,
    rating: u8,
}

fn main() {
    let cat_names = vec!["Beans", "Basil", "Oliver", "Funcy"];

    let cats = vec![
        Cat {
            name: "Beans".to_string(),
            rating: 1,
        },
        Cat {
            name: "Basil".to_string(),
            rating: 2,
        },
        Cat {
            name: "Oliver".to_string(),
            rating: 3,
        },
        Cat {
            name: "Compy".to_string(),
            rating: 4,
        },
    ];
    mount_to_body(|cx| {
        view! { cx,
            <h1>"Great cat names (func)"</h1>
            <ul>
                {list_names_func(cx,cat_names)}
            </ul>
            <ListNames cats />
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
fn ListNames(cx: Scope, cats: Vec<Cat>) -> impl IntoView {
    view! {cx,
        <h1>"Great cat names (component)"</h1>
        <ul>
            <For
                each={move || cats.clone()}
                key={|cat| cat.rating}
                view={
                    move |cat| {
                        view! {
                            cx,
                            <li id={cat.rating}>{cat.name}</li>
                        }
                    }
                }
            />
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
