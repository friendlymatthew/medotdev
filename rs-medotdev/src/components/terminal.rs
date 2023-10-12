use leptos::*;
use leptos::html::{Div, Input};
use strum::IntoEnumIterator;
use crate::autocomplete::Trie;
use crate::commands::warehouse::{Command, CommandFactory};
use crate::Log;

#[component]
pub fn Terminal(
    log_signal: (ReadSignal<Log>, WriteSignal<Log>)
) -> impl IntoView {

    let input_ref = create_node_ref::<Input>();
    let autocomplete_ref = create_node_ref::<Div>();

    view! {
        <div
            class="flex flex-col h-full w-full cursor-text relative"
            on:click=move |_| {
                let input = input_ref.get().expect("input_ref should be loaded");
                input.focus().unwrap();
            }
        >
                <CommandLine
                    log_signal=log_signal.0
                    set_log_signal=log_signal.1
                    _input_ref=input_ref
                />
                <div class="flex-grow overflow-y-auto">
                    <StackLog
                        log_signal=log_signal.0
                        input_ref=input_ref
                    />
                </div>
        </div>
    }
}


#[component]
fn StackLog(
    log_signal: ReadSignal<Log>,
    input_ref: NodeRef<Input>
) -> impl IntoView {
    view! {
        <ul
            class="px-2"

        >
            <For
                each=log_signal
                key=|log| log.0
                children=move |(_id, log)| {
                    let curr_command = log.clone();

                    view! {
                        <li class="pb-6">
                            > {log}
                            <CommandFactory
                                curr_command=curr_command
                            />
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
    _input_ref: NodeRef<Input>
) -> impl IntoView {
    let (command_input, set_command_input) = create_signal("".to_string());
    let (suggestions, set_suggestions) = create_signal(Vec::new());
    let mut trie = Trie::new();

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
                curr_log.push_front(
                    (id, curr_command)
                )
            }

            set_command_input("".to_string());
        });
    };

    view! {
        <div class="w-full flex">
            <input
                ref=_input_ref
                placeholder=move || if log_signal.get().len() == 0 {
                    "Enter help to get started"
                } else {
                    ""
                }
                type="text"
                class="flex-grow border-transparent text-2xl p-2 focus:outline-none"
                on:input=move |ev| {
                    set_command_input(event_target_value(&ev));
                }
                on:keydown=move |ev| {
                    if ev.key() == "Enter" {
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
        </div>
        <div class="absolute top-14 left-0 z-10 w-[30em] bg-orange-300">
            <Show when=move || suggestions.get().len() != 0 && suggestions.get().len() != 8 >
            {
                let autoc = suggestions.get().into_iter().map(|sug| {
                    view! {
                        <div class="p-1">
                            <p>
                                {sug}
                            </p>
                        </div>
                    }
                }).collect::<Vec<_>>();

                {autoc}
            }
            </Show>
        </div>
    }
}
