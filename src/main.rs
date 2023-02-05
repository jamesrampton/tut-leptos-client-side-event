use leptos::*;

fn main() {
    mount_to_body(|cx| {
        view! {
            cx,
            <h1>"Hello, world!"</h1>
        }
    });
}
