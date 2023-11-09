use crate::components::sidebar::Sidebar;
use crate::components::terminal::Terminal;
use crate::Log;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/medotdev.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/:cmd" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (log, set_log) = create_signal(Log::new());

    let update_log = |cmds: Vec<String>| {
        let start_id = log.get().len(); // Starting ID based on the current log length

        for (index, cmd) in cmds.clone().into_iter().enumerate() {
            if cmd != "clear" {
                let id = start_id + cmds.len() - 1 - index;
                set_log.update(move |curr_log| {
                    curr_log.push_front((id, cmd.to_lowercase()));
                });
            }
        }
    };

    let params = use_params_map();
    let pcmd = move || params.with(|p| {
        p.get("cmd")
            .map(|cmd| cmd.split('+').map(String::from).collect())
            .unwrap_or_default()
    });


    update_log(pcmd());

    view! {
        <main class="flex justify-center h-screen w-screen">
            <div class="my-2 flex-none w-full md:flex w-11/12 md:w-10/12 md:divide-x-2 border-x-2 md:border-l-0 md:border-r-2">
                <Sidebar set_log_signal=set_log/>
                <Terminal log_signal=(log, set_log)/>
            </div>
        </main>
    }
}
