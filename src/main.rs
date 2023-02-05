use leptos::*;

fn main() {
    mount_to_body(|cx| {
        view! {
            cx,
            <h1>"Hello, world!"</h1>
            <NiceAffirmation />
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
