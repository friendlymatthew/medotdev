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
        <Stylesheet
            id="google-fonts"
            href="https://fonts.googleapis.com/css2?family=Lora:wght@400;500&family=Poppins:ital,wght@0,400;0,500;1,400;1,500&display=swap"
        />
        <Stylesheet id="leptos" href="/pkg/rs-medotdev.css"/>

        // sets the document title
        <Title text="Matthew Kim"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path=":cmd" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[derive(Params, PartialEq)]
struct CmdParam {
    cmd: String
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (log, set_log) = create_signal(Log::new());

    let update_log = |cmd : Option<String>| {
        if let Some(cmd) = cmd {
            if cmd != "clear" {
                set_log.update(|curr_log| {
                    curr_log.push_front((0, cmd.to_lowercase()));
                })
            }
        }
    };

    let params = use_params_map();
    let pcmd = move || {
        params.with(|p| p.get("cmd").cloned())
    };

    update_log(pcmd());


    view! {
        <main class="flex justify-center h-screen w-screen">
            <div class="my-2 flex-none md:flex w-11/12 md:w-10/12 md:divide-x-2 border-x-2 md:border-l-0 md:border-r-2">
                <Sidebar set_log_signal=set_log/>
                <Terminal log_signal=(log, set_log)/>
            </div>
        </main>
    }
}


