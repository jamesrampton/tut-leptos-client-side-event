use leptos::*;

fn main() {
    mount_to_body(|cx| {
        view! {
            cx,
            <h1>"Hello, world!"</h1>
            <NiceAffirmation />
            <LuckyNumber the_lucky_number=43 />
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
fn LuckyNumber(cx: Scope, the_lucky_number: i32) -> impl IntoView {
    view! {
        cx,
        <p>"Today's lucky number is " {the_lucky_number}</p>
    }
}
