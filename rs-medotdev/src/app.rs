use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::sidebar::Sidebar;
use crate::components::terminal::Terminal;
use crate::{Log};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rs-medotdev.css"/>

        // sets the document title
        <Title text="Matthew Kim"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {

    let (log, set_log) = create_signal(Log::new());

    view! {
        <main class="flex justify-center h-screen w-screen">
            <div class="p-4 flex-none md:flex w-10/12 divide-x-2 divide-x-black">
                <Sidebar
                    set_log_signal=set_log
                />
                <Terminal
                    log_signal = (log, set_log)
                />
            </div>
        </main>
    }
}


