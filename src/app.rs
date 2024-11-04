use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="container">
            <h1>"Craig's Media Merge"</h1>

            <div class="row">
                <img src="public/cmm-logo.svg" class="logo cmm" alt="logo" />
            </div>

            <form class="row">
                <button>"Configuration"</button>
            </form>

        </main>
    }
}
