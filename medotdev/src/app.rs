use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::Log;
use crate::components::terminal::Terminal;
use crate::components::sidebar::Sidebar;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/medotdev.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path=":cmd" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (log, set_log) = create_signal(Log::new());

    let update_log = |cmd : Option<String>| {
        if let Some(cmd) = cmd {
            if cmd != "clear" && cmd.len() > 0 {
                set_log.update(|curr_log| {
                    curr_log.push_front((0, cmd.to_lowercase()));
                })
            }
        }
    };

    let params = use_params_map();
    let pcmd = move || {
        params.with(|p| p.get("cmd").cloned().unwrap_or_default())
    };

    update_log(Some(pcmd()));


    view! {
        <main class="flex justify-center h-screen w-screen">
            <div class="my-2 flex-none md:flex w-11/12 md:w-10/12 md:divide-x-2 border-x-2 md:border-l-0 md:border-r-2">
                <Sidebar set_log_signal=set_log/>
                <Terminal log_signal=(log, set_log)/>
            </div>
        </main>
    }
}


