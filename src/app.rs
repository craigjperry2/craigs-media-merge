use leptos::*;
use crate::config::Config;

#[component]
pub fn App() -> impl IntoView {
    let (show_config_editor, set_show_config_editor) = create_signal(false);

    view! {
        <main class="container">
            <h1>"Craig's Media Merge"</h1>

            <div class="row">
                <img src="public/cmm-logo.svg" class="logo cmm" alt="logo" />
            </div>

            {move || if show_config_editor.get() {
                let config = Config::new();
                view! {
                    <ConfigurationEditor config={config.file_exclude_patterns().clone()} />
                }} else {
                    view! {
                        <form class="row">
                            <button on:click=move |_| set_show_config_editor.update(|v| *v = !*v)>"Configuration"</button>
                        </form>
                    }.into_view()
                }
            }
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
