use leptos::*;

fn main() {
    mount_to_body(|cx| {
        view! {
            cx,
            <h1>"Hello, world!"</h1>
            <NiceAffirmation />
            <LuckyNumber />
        }
    });
}

#[component]
pub fn NiceAffirmation(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <p>"You look nice today!"</p>
    }
}
#[component]
fn LuckyNumber(cx: Scope) -> impl IntoView {
    let the_lucky_number = 42;
    view! {
        cx,
        <p>"Today's lucky number is " {the_lucky_number}</p>
    }
}
