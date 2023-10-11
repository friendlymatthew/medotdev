use leptos::*;
use crate::Log;

#[derive(PartialEq)]
pub enum Command {
    About,
    Picture,
    Education,
    Resume,
    Contact,
    Help,
    Clear,
    Rogue,
    Ls,
}

impl Command {
    pub fn from_str(s: &str) -> Command {
        match s.to_lowercase().as_str() {
            "about" => Command::About,
            "picture" => Command::Picture,
            "education" => Command::Education,
            "resume" => Command::Resume,
            "contact" => Command::Contact,
            "help" => Command::Help,
            "ls" => Command::Ls,
            "clear" => Command::Clear,
            _ => Command::Rogue,
        }
    }
}

#[component]
pub fn command_factory(
    curr_command: String
) -> impl IntoView {
    let command = Command::from_str(&curr_command);
    match command {
        Command::About => {
            view! {
                <div>
                 <p>wef</p>
                </div>
            }
        }
        Command::Picture => {
            view! {
                <div>videokilledtheradiostar</div>
            }

        }
        Command::Education => {
            view! {
                <div> </div>
            }
        }
        Command::Resume => {
            view! {
                <div> </div>
            }
        }
        Command::Contact => {
            view! {
                <div> </div>
            }
        }
        Command::Help => {
            view! {
                <div> </div>
            }
        }
        Command::Ls => {
            view! {
                <div> </div>
            }
        }
        Command::Clear => {
            view! {
                <div></div>
            }
        }
        Command::Rogue => {
            view! {
                <div>outofsight</div>
            }
        }
    }
}

