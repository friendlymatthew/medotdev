use std::sync::Mutex;
use lazy_static::lazy_static;
use leptos::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(PartialEq, EnumIter, Debug)]
pub enum Command {
    Now,
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
            "now" => Command::Now,
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

    pub fn parse_command(cmd: &Command) -> Option<String> {
        match cmd {
            Command::Now => Some("now".to_string()),
            Command::About => Some("about".to_string()),
            Command::Picture => Some("picture".to_string()),
            Command::Education => Some("education".to_string()),
            Command::Resume => Some("resume".to_string()),
            Command::Contact => Some("contact".to_string()),
            Command::Help => Some("help".to_string()),
            Command::Clear => Some("clear".to_string()),
            Command::Ls => Some("ls".to_string()),
            _ => None
        }
    }
}

lazy_static! {
    static ref CURRENT_INDEX: Mutex<usize> = Mutex::new(0);
}

#[component]
pub fn command_factory(
    curr_command: String
) -> impl IntoView {
    let command = Command::from_str(&curr_command);
    let title_css = "md:text-xl text-lg";
    let body_css = "md:text-lg text-base";
    match command {
        Command::Now => {
            let title = "What I'm doing now";
            let last_updated = "Last updated October 15, 2023";

            let content = [
                "taking time off to learn and build projects in Rust",
                "writing a lexer, parser, diagrammer for the untyped lambda calculus",
                "contributing to open source, building workflow tools to internationalize textbooks",
                "planning a backpacking trip in New Hampshire",
            ];

            view! {
                <div class="space-y-2">
                    <p class=format!("font-medium {}", title_css)>{title}</p>
                    <ul class="pl-2">

                        {
                            let content = content
                                .into_iter()
                                .map(|thing| {
                                    view! { <li>{thing}</li> }
                                })
                                .collect::<Vec<_>>();
                            content
                        }

                    </ul>
                    <br/>
                    <p class="italic text-xs md:text-sm opacity-90">{last_updated}</p>
                </div>
            }
        }
        Command::About => {
            view! {
                <div class=format!("w-full flex justify-center {}", title_css)>
                    <div class="px-1 md:w-2/3">
                        <p>
                            Hello! I am a recent graduate, software engineer, and open source contributor.

                        </p>
                        <br/>
                        <p>
                            I am pasionate about Rust and the community, you can see my work here:
                            <a
                                href="https://github.com/friendlymatthew"
                                target="_blank"
                                rel="noreferrer"
                                class="underline hover:decoration-dotted decoration-solid underline-offset-4 decoration-[#ffc832] decoration-4"
                            >
                                github.com/friendlymatthew.
                            </a>
                        </p>
                        <br/>
                        <p>
                            Previously, I interned at Toast, a restaurant point-of-sale company, where I worked on the waitlist and reservation service.
                        </p>
                        <br/>
                        <p>
                            I care about democratizing food and advancing agriculture.
                            <a
                                href="https://www.nass.usda.gov/Publications/Highlights/2019/2017Census_Farm_Producers.pdf"
                                target="_blank"
                                rel="noreferrer"
                                class="underline hover:decoration-dotted decoration-solid underline-offset-4 decoration-[#2e2459] decoration-4"
                            >
                                Our farmers are aging
                            </a>
                            and will need modern solutions to address the challenges posed by our rapidly changing climate.
                        </p>
                        <br/>
                        <p>
                            I train jiu jitsu and enjoy nature. I listen to country music,
                            <a
                                href="https://www.youtube.com/watch?v=2wGuHHY11SM"
                                target="_blank"
                                rel="noreferrer"
                                class="underline hover:decoration-dotted decoration-solid underline-offset-4 decoration-[#0b7261] decoration-4"
                            >
                                The Eagles
                            </a> , and
                            <a
                                href="https://www.youtube.com/watch?v=ELoXiuDA_sQ"
                                target="_blank"
                                rel="noreferrer"
                                class="underline hover:decoration-dotted decoration-solid underline-offset-4 decoration-[#a72145] decoration-4"
                            >
                                Norm.
                            </a>
                        </p>
                    </div>
                </div>
            }
        }
        Command::Picture => {

            let images = [
                ("boulder", "Harpers Ferry, WV"),
                ("momsis", "Graduation at Wes"),
                ("shenandoah", "Me and my friend Cisco at Shenandoah"),
                ("goose", "My friend Joseph's dog, Goose"),
                ("nelson", "Me and my friend Nelson"),
                ("rock", "Old Rag Mountain, Shenandoah, VA"),
            ];

            let index;
            {
                let mut curr = CURRENT_INDEX.lock().unwrap();
                index = *curr;
                *curr = (*curr + 1) % images.len();
            }

            let (image_name, alt_text) = images[index];

            view! {
                <div class="">
                    <picture class="">
                        <source
                            srcset=format!("public/webpg/{}.webp", image_name)
                            type="image/webp"
                        />
                        <img src=format!("public/pngs/{}.png", image_name) alt=alt_text/>
                    </picture>
                </div>
            }
        }
        Command::Education => {
            view! {
                <div>
                    <p class=format!(
                        "{}",
                        title_css,
                    )>
                        May 2023 - I recieved a B.A. in Computer Science and Data Science from Wesleyan University.
                    </p>
                </div>
            }
        }
        Command::Resume => {
            view! { <div>Resume</div> }
        }
        Command::Contact => {
            view! {
                <div class=format!("{}", title_css)>
                    <p>You can reach me via:</p>
                    <div class=format!("pl-2 space-y-1 {}", body_css)>
                        <p>
                            email:
                            <a
                                href="mailto:matthewkmkim@gmail.com"
                                target="_blank"
                                rel="noreferrer"
                                class="italic font-semibold underline hover:decoration-dotted decoration-solid underline-offset-4 decoration-[#2e2459] decoration-4"
                            >
                                matthewkmkim@gmail.com
                            </a>
                        </p>
                        <p>
                            linkedin:
                            <a
                                href="https://linkedin.com/in/mat-thew"
                                target="_blank"
                                rel="noreferrer"
                                class="italic font-semibold underline hover:decoration-dotted decoration-solid underline-offset-4 decoration-[#0b7261] decoration-4"
                            >
                                /in/mat-thew
                            </a>
                        </p>
                    </div>
                </div>
            }
        }
        Command::Help => {
            let title = "Commands";
            let commands = [
                ("now", "Inspired by Sean McArthur and Derek Siver's", Some(("https://nownownow.com/about", "/now"))),
                ("about", "Quick excerpt about me", None),
                ("picture", "Slideshow of me in the wild with my friends", None),
                ("education", "My formal education", None),
                ("resume", "My CV", None),
                ("contact", "My email and linkedin, happy to connect!", None),
                ("clear", "Clean slates the terminal view", None),
                ("ls", "Lists out all commands", None),
                ("help", "Print help", None),
            ];

            view! {
                <div>
                    <div class=format!("{} font-medium", title_css)>
                        <p>{title}</p>
                    </div>
                    <ul class=format!(
                        "space-y-2 {}",
                        body_css,
                    )>

                        {
                            let help_dash = commands
                                .into_iter()
                                .map(|(cmd, desc, link)| {
                                    view! {
                                        <li class="grid grid-cols-3 md:grid-cols-5 lg:w-[36em]">
                                            <div>
                                                <p>{cmd}</p>
                                            </div>
                                            <div class="col-span-2 md:col-span-4">
                                                <p class="">
                                                    {desc} {" "} <Show when=move || link.is_some()>
                                                        <a
                                                            href=link.unwrap().0
                                                            target="_blank"
                                                            rel="noreferrer"
                                                            class="underline hover:decoration-dotted decoration-4 underline-offset-4 decoration-[#0b7261]"
                                                        >
                                                            {link.unwrap().1}
                                                        </a>

                                                    </Show>
                                                </p>
                                            </div>
                                        </li>
                                    }
                                })
                                .collect::<Vec<_>>();
                            help_dash
                        }

                    </ul>
                </div>
            }
        }
        Command::Ls => {
            let all_commands = Command::iter().map(|cmd| {
                view! {
                    <li>
                        <p>{Command::parse_command(&cmd)}</p>
                    </li>
                }
            }).collect::<Vec<_>>();

            view! {
                <div class=format!("{}", title_css)>
                    <ul>{all_commands}</ul>
                </div>
            }
        }
        Command::Clear => {
            view! { <div></div> }
        }
        Command::Rogue => {
            view! {
                <div class=format!("{}", title_css)>
                    <p class=format!("{}", body_css)>Unrecognized command: {&curr_command}</p>
                </div>
            }
        }
    }
}