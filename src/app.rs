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

            <ConfigurationEditor config={vec![
                String::from("*.tmp"),
                String::from("*.bak"),
                String::from("*.swp"),
            ]} />
        </main>
    }
}

#[component]
pub fn ConfigurationEditor(config: Vec<String>) -> impl IntoView {
    view! {
        <div>
            <h2>"Configuration Editor"</h2>
            <form class="config-editor">
                {config.iter().map(|pattern| view! {
                    <div>
                        <input type="text" value={pattern} />
                        <button>"Remove"</button>
                    </div>
                }).collect::<Vec<_>>()}
                <div>
                    <button>"Add"</button>
                </div>
            </form>
        </div>
    }
}
