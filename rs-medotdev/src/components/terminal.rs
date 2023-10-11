use leptos::*;
use crate::commands::warehouse::{Command, CommandFactory};
use crate::Log;

#[component]
pub fn Terminal(
    log_signal: (ReadSignal<Log>, WriteSignal<Log>)
) -> impl IntoView {

    view! {
        <div class="flex-grow h-full w-full">
            <div class="flex flex-col w-full h-full">
                <CommandLine set_log_signal=log_signal.1 />
                <StackLog set_log_signal=log_signal.1 log_signal=log_signal.0 />
            </div>
        </div>
    }
}


#[component]
fn StackLog(
    set_log_signal: WriteSignal<Log>,
    log_signal: ReadSignal<Log>
) -> impl IntoView {
    view! {
        <ul
            class="overflow-y-auto flex-grow px-2"
        >
            <For
                each=log_signal
                key=|log| log.0
                children=move |(_id, log)| {

                    view! {
                        <li class="py-2">
                            <CommandFactory
                                curr_command=log
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
    set_log_signal: WriteSignal<Log>
) -> impl IntoView {
    let (command_input, set_command_input) = create_signal("Press Help".to_string());

    let add_log = move || {
        set_log_signal.update(move |curr_log| {
            let id = curr_log.len();
            let curr_command = command_input.get();
            curr_log.push_front(
                (id, curr_command)
            )
        });
    };

    view! {
        <input
            type="text"
            class="border-transparent text-2xl w-full p-2 focus:ring-0 focus:border-transparent"
            on:input=move |ev| {
                set_command_input(event_target_value(&ev));
            }

            prop:value=command_input
        />
        <button
            on:click =move |ev| {
                // here we would submit query
                ev.prevent_default();
                add_log();

            }
        >
           submit
        </button>
    }
}



