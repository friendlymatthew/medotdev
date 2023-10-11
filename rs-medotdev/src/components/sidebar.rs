use leptos::*;
use crate::Log;

#[component]
fn StaticList(
    commands: Vec<String>,
    set_log: WriteSignal<Log>,
) -> impl IntoView {
    let command_list: Vec<_> = commands.into_iter().map(|cmd| {
        let cmd_copy = cmd.clone();
        view! {
            <li>
                <button
                    on:click = move |_| {
                        let cmd_clone = cmd.clone();  // Clone the command just before using it.
                        set_log.update(move |curr_log| {
                            let id = curr_log.len();
                            curr_log.push_front(
                                (id, cmd_clone)  // Use the freshly cloned command.
                            )
                        });
                    }
                >
                    {cmd_copy}
                </button>
            </li>
        }
    })
        .collect();

    view! {
        <ul>
            {command_list}
        </ul>
    }
}

#[component]
pub fn Sidebar(
    set_log_signal: WriteSignal<Log>
) -> impl IntoView {


    let me_commands = vec![
        "About".to_string(),
        "Picture".to_string(),
        "Education".to_string(),
        "Resume".to_string(),
        "Contact".to_string(),
    ];

    let terminal_commands = vec![
        "Help".to_string(),
        "Clear".to_string(),
    ];



    view! {
        <div>
            <div class="md:w-[20em]">
                <a
                    href="https://github.com/friendlymatthew"
                    target="_blank"
                    rel="noreferrer"
                >
                    <p
                        class="text-3xl font-semibold"
                    >
                        Matthew Kim
                    </p>
                </a>
            </div>
            <div class="divide-y-2">
                <StaticList
                    commands=me_commands
                    set_log=set_log_signal
                />
                <StaticList
                    commands=terminal_commands
                    set_log=set_log_signal
                />
            </div>
        </div>
    }
}