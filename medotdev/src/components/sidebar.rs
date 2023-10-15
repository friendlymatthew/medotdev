use leptos::*;
use crate::Log;

#[component]
fn StaticList(
    bg_color: String,
    commands: Vec<String>,
    set_log: WriteSignal<Log>,
) -> impl IntoView {
    let command_list: Vec<_> = commands.into_iter().map(|cmd| {
        let cmd_copy = cmd.clone();
        view! {
            <li class="md:w-full">
                <button
                    class="md:w-full flex justify-start group"
                    on:click=move |_| {
                        let cmd_clone = cmd.clone();
                        set_log
                            .update(move |curr_log| {
                                let id = curr_log.len();
                                if cmd_clone == "Clear" {
                                    curr_log.clear();
                                } else {
                                    curr_log.push_front((id, cmd_clone))
                                }
                            });
                    }
                >

                    <p class="group-hover:underline decoration-[3px] hover:decoration-dotted underline-offset-4 decoration-[#ffc832]">
                        {cmd_copy}
                    </p>
                </button>
            </li>
        }
    })
        .collect();

    view! {
        <ul class=format!(
            "flex flex-wrap justify-center space-x-8 md:space-x-0 md:flex-col px-8 md:px-2 py-2 md:space-y-2 {}",
            bg_color,
        )>{command_list}</ul>
    }
}

#[component]
pub fn Sidebar(
    set_log_signal: WriteSignal<Log>
) -> impl IntoView {

    let me_commands = vec![
        "Now".to_string(),
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
        <div class="lg:w-[20em] divide-y-2 text-white font-semibold font-poppins">
            <div class="bg-[#2e2459] py-2 group font-lora">
                <a
                    href="https://github.com/friendlymatthew"
                    target="_blank"
                    rel="noreferrer"
                    class="group-hover:underline underline-offset-4 decoration-[3px] decoration-[#ffc832]"
                >
                    <p class="pl-2 text-xl md:text-3xl">Matthew Kim</p>
                </a>
            </div>
            <div class="divide-y-2 text-base md:text-xl">
                <StaticList
                    bg_color="bg-[#0b7261]".to_string()
                    commands=me_commands
                    set_log=set_log_signal
                />
                <StaticList
                    bg_color="bg-[#a72145]".to_string()
                    commands=terminal_commands
                    set_log=set_log_signal
                />
            </div>
        </div>
    }
}