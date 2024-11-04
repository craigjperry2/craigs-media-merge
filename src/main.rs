mod app;

use app::*;
use leptos::*;

mod orchestrator;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
