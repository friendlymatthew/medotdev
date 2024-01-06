use crate::autocomplete::Trie;
use crate::commands::warehouse::{Command, CommandFactory};
use crate::Log;
use leptos::html::{Div, Input, Ul};
use leptos::web_sys::HtmlUListElement;
use leptos::*;
use strum::IntoEnumIterator;

#[component]
pub fn Terminal(log_signal: (ReadSignal<Log>, WriteSignal<Log>)) -> impl IntoView {
    let input_ref = create_node_ref::<Input>();
    let _autocomplete_ref = create_node_ref::<Div>();

    view! {
        <div
            class="font-lora flex flex-col h-full w-full cursor-text relative"
            on:click=move |_| {
                let input = input_ref.get().expect("input_ref should be loaded");
                input.focus().unwrap();
            }
        >

            <CommandLine log_signal=log_signal.0 set_log_signal=log_signal.1 _input_ref=input_ref/>
            <div class="flex-grow overflow-y-auto">
                <StackLog log_signal=log_signal.0 input_ref=input_ref/>
            </div>
        </div>
    }
}

#[component]
fn StackLog(log_signal: ReadSignal<Log>, input_ref: NodeRef<Input>) -> impl IntoView {
    let container_ref = create_node_ref::<Ul>();

    create_effect(move |_| {
        if let Some(container) = container_ref.get() {
            println!("wef");
            container.set_scroll_top(0);
        }
    });

    view! {
        <ul ref={container_ref} class="px-2 overflow-y-auto">
            <For
                each=log_signal
                key=|log| log.0
                children=move |(_id, log)| {
                    let curr_command = log.clone();
                    view! {
                        <li class="pb-12 text-lg font-robotomono">
                            <p class="font-lora pb-2">> {log}</p>
                            <CommandFactory curr_command=curr_command/>
                        </li>
                    }
                }
            />

        </ul>
    }
}

#[component]
fn CommandLine(
    log_signal: ReadSignal<Log>,
    set_log_signal: WriteSignal<Log>,
    _input_ref: NodeRef<Input>,
) -> impl IntoView {
    let (command_input, set_command_input) = create_signal("".to_string());
    let (suggestions, set_suggestions) = create_signal(Vec::new());
    let mut trie = Trie::new();
    let (is_focused, set_is_focused) = create_signal(false);

    for command in Command::iter() {
        if let Some(command) = Command::parse_command(&command) {
            trie.insert(command.to_string().to_lowercase().as_str());
        }
    }

    create_effect(move |_| {
        let prefix = command_input.get().to_lowercase();
        let trie_results = trie.search(&prefix);

        set_suggestions.set(trie_results)
    });

    let add_log = move || {
        set_log_signal.update(move |curr_log| {
            let id = curr_log.len();
            let curr_command = command_input.get();

            if curr_command.to_lowercase() == "clear" {
                curr_log.clear()
            } else {
                curr_log.push_front((id, curr_command))
            }

            set_command_input("".to_string());
        });
    };

    view! {
        <div class="w-full flex relative">
            <input
                ref=_input_ref
                type="text"
                class="flex-grow border-transparent text-2xl p-2 focus:outline-none"
                on:input=move |ev| {
                    set_command_input(event_target_value(&ev));
                }

                on:focus=move |_| {
                    set_is_focused(true);
                }

                on:blur=move |_| {
                    set_is_focused(false);
                }

                on:keydown=move |ev| {
                    if ev.key() == "Enter" && command_input.get().len() != 0 {
                        add_log();
                    }
                    if ev.key() == "ArrowRight" {
                        if suggestions.get().len() > 0 && suggestions.get().len() != 8 {
                            set_command_input(suggestions.get()[0].clone())
                        }
                    }
                }

                prop:value=command_input
            />
            <Show when=move || command_input.get().is_empty()>
                <div style="color: #2a3439;" class="absolute inset-y-0 left-2 flex items-center">
                    <p class="text-2xl bg-current w-3 animate-blink">l</p>
                </div>
            </Show>
        </div>
        <div class="absolute top-14 left-0 z-10 w-[10em] bg-[#2a3439] text-white">
            <Show when=move || {
                is_focused.get() && suggestions.get().len() != 0
                    && suggestions.get().len() != Command::iter().len() - 1
            }>

                {
                    let autoc = suggestions
                        .get()
                        .into_iter()
                        .map(|sug| {
                            let sug_clone = sug.clone();
                            view! {
                                <div class="p-1 group flex justify-between items-center">
                                    <p class="decoration-[2px] underline-offset-4 decoration-[#ffc832]">
                                        {sug.clone()}
                                    </p>
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                        stroke-width="2"
                                        stroke="currentColor"
                                        class="w-4 h-4"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            d="M4.5 12h15m0 0l-6.75-6.75M19.5 12l-6.75 6.75"
                                        ></path>
                                    </svg>

                                </div>
                            }
                        })
                        .collect::<Vec<_>>();
                    autoc
                }

            </Show>
        </div>
    }
}
