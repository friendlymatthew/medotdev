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
    let bro_css = "md:text-base text-sm";
    let link_css = "underline hover:decoration-dotted decoration-solid underline-offset-4 decoration-4";


    match command {
        Command::Now => {
            let title = "What I'm doing ";
            let last_updated = "Last updated October 15, 2023";

            let content = [
                "taking time off to learn and build projects in Rust",
                "writing a lexer, parser, diagrammer for the untyped lambda calculus",
                "contributing to open source, building workflow tools to internationalize textbooks",
                "planning a backpacking trip in New Hampshire",
            ];

            view! {
                <div class="space-y-2">
                    <p class=format!(
                        "font-medium {}",
                        title_css,
                    )>
                        {title}
                        <a
                            href="https://nownownow.com/about"
                            target="_blank"
                            rel="noreferrer"
                            class=format!("{} decoration-[#0b7261]", link_css)
                        >
                            now
                        </a>
                    </p>
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
                            I am passionate about Rust and the community, you can see my work here:
                            <a
                                href="https://github.com/friendlymatthew"
                                target="_blank"
                                rel="noreferrer"
                                class=format!("{} decoration-[#2e2459]", link_css)
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
                                class=format!("{} decoration-[#ffc832]", link_css)
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
                                class=format!("{} decoration-[#0b7261]", link_css)
                            >
                                The Eagles
                            </a> , and
                            <a
                                href="https://www.youtube.com/watch?v=ELoXiuDA_sQ"
                                target="_blank"
                                rel="noreferrer"
                                class=format!("{} decoration-[#a72145]", link_css)
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

            let image_css = "w-[20em]";
            view! {
                <div class="">
                    <picture>
                        <source
                            srcset=format!("webpg/{}.webp", image_name)
                            class=format!("{}", image_css)
                            type="image/webp"
                        />
                        <img
                            class=format!("{}", image_css)
                            src=format!("pngs/{}.png", image_name)
                            alt=alt_text
                        />
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
            let experience = [
                (
                    "Open Source Contributor",
                    "Aug 2023 - Now",
                    "Mdbook i18n, Twenty",
                    "Mdbook-i18n provides translation support for markdown textbooks in Rust. Twenty is a modern CRM with a Notion-like feel.",
                    vec![
                        "Wrote standalone binaries that extracts and translates markdown files",
                        "Built out unselect feature for tables and kanban boards, improving user experience and workflow",
                        "Improved photo uploader by integrating GraphQL Apollo upload controller and displaying upload errors"
                    ]
                ),
                (
                    "Software Engineer Intern",
                    "June 2023 - Aug 2023",
                    "Toast",
                    "Toast is a restaurant point of sales company. Toast Tables is a waitlist and reservations service.",
                    vec![
                        "Built deposit booking and 'table ready' SMS notifications using Apache Pulsar and Twilio",
                        "Fixed critical bug in waitlist time estimator that incorrectly displayed 0 min wait times during restaurant service",
                        "Reduced API requests by revamping booking sidebar resulting in $5,000 in monthly savings",
                        "Refactored SMS service and implemented Sonarqube compliance by reducing cognitive complexity and raising test coverage from 22% to 92%",
                        "Developed product usage panel that tracks and informs free-tier restaurants of monthly booking quotas",
                        "Extended DateTime library and refactored codebase to compute precise restaurant closeout time",
                        "Solved customer care bugs, using Splunk and Datadog to debug and monitor production systems"
                    ]
                ),
                (
                    "Founder",
                    "April 2023 - Now",
                    "XYZ",
                    "XYZ is an end of school tradition where people send messages, play a matching game, and say goodbye.",
                    vec![
                        "Architected and built 1:1 social chat platform; lead 3-person engineering team and managed 10-person team",
                        "Managed viral growth, handled 783 users with 1200+ messages corresponding and 200+ chat rooms created daily",
                        "Scaled product with Vercel CI/CD, rolled out key features: message encryption, notifications, community board, user blocking, confetti animations, chat room deletion, chat harassment policing",
                    ]
                ),
                (
                    "Software Engineer I",
                    "Sept. 2021 - Sept. 2022",
                    "Wesleyan Media Project",
                    "WMP is a political advertising research lab",
                    vec![
                        "Engineered data labeling platform streamlining task delegation and data annotation for ML models",
                        "Wrote tooling using Facebook ad data, mapped state-by-state campaign spending's for the 2020 US presidential election, tabled data, and charted custom choropleth scale maps",
                        "Resolved storage and speed inefficiencies by migrating to cloud microservice and rebuilding lab tools",
                        "Automated administrator duties by configuring IAM roles, managing CloudWatch logs, and curating billing invoices"
                    ]
                )
            ];

            let teaching = [
                (
                    "Wesleyan University Teaching Assistant",
                    "QAC 239 - Machine Learning Proseminar",
                    "Jan. 2023 - May 2023"
                )
            ];
            view! {
                <div class="space-y-4">
                    <div>
                        <p class=format!("font-semibold pb-1 {}", title_css)>Work Experience</p>
                        <ul class="space-y-4">

                            {
                                let content = experience
                                    .into_iter()
                                    .map(|(title, date, work, work_desc, bullets)| {
                                        view! {
                                            <li class="pl-2">
                                                <div class="">
                                                    <div class="w-full flex flex-wrap justify-between">
                                                        <div class="">
                                                            <div class=format!("flex {} font-medium", body_css)>
                                                                <p class="font-normal">{title}</p>
                                                                <p>, {work}</p>
                                                            </div>
                                                            <p class="md:text-sm text-xs italic">{work_desc}</p>
                                                        </div>
                                                        <p class="py-1 sm:py-0 md:text-sm text-xs">{date}</p>
                                                    </div>
                                                </div>
                                                <ul class="text-sm md:text-base w-11/12 md:w-5/6">

                                                    {
                                                        let detail = bullets
                                                            .into_iter()
                                                            .map(|b| {
                                                                view! { <li>- {b}</li> }
                                                            })
                                                            .collect::<Vec<_>>();
                                                        detail
                                                    }

                                                </ul>
                                            </li>
                                        }
                                    })
                                    .collect::<Vec<_>>();
                                content
                            }

                        </ul>
                    </div>
                    <div>
                        <p class=format!("font-semibold pb-1 {}", title_css)>Teaching Experience</p>
                        <ul class="space-y-2">

                            {
                                let content = teaching
                                    .into_iter()
                                    .map(|(title, class, date)| {
                                        view! {
                                            <li class="pl-2 space-y-2">
                                                <div class="space-y-1">
                                                    <div class="w-full flex flex-wrap items-end justify-between">
                                                        <p>{title}</p>
                                                        <p class="md:text-sm text-xs">{date}</p>
                                                    </div>
                                                    <p>{class}</p>
                                                </div>
                                            </li>
                                        }
                                    })
                                    .collect::<Vec<_>>();
                                content
                            }

                        </ul>
                    </div>
                </div>
            }
        }
        Command::Contact => {
            let link_padding_css = "italic font-semibold";
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
                                class=format!(
                                    "{} {} decoration-[#0b7261]",
                                    link_padding_css,
                                    link_css,
                                )
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
                                class=format!(
                                    "{} {} decoration-[#2e2459]",
                                    link_padding_css,
                                    link_css,
                                )
                            >

                                /in/mat-thew
                            </a>
                        </p>
                    </div>
                </div>
            }
        }
        Command::Help => {
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
                <div class="space-y-4">
                    <div>
                        <div class=format!("{} font-medium", title_css)>
                            <p>Actions</p>
                        </div>
                        <div class=format!(
                            "{}",
                            body_css,
                        )>

                            {
                                let content = [
                                    "You can type commands and use the right arrow key to autocomplete recognized commands.",
                                    "Don't want to type? Click on any of the commands to the left.",
                                ]
                                    .into_iter()
                                    .map(|phr| {
                                        view! { <p>{phr}</p> }
                                    })
                                    .collect::<Vec<_>>();
                                content
                            }

                        </div>
                    </div>
                    <div>
                        <div class=format!("{} font-medium", title_css)>
                            <p>Commands</p>
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
                                                                class=format!(
                                                                    "{} decoration-[#a72145]",
                                                                    link_css,
                                                                )
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
                    <div class="text-base md:text-lg">

                        {
                            let ps = "For anyone interested, here's the ";
                            view! {
                                <p>
                                    {ps}
                                    <a
                                        class=format!(
                                            "{} decoration-[#2e2459]",
                                            link_css
                                        )
                                        href="https://github.com/friendlymatthew/medotdev"
                                        target="_blank"
                                        rel="noreferrer"
                                    >
                                        source code
                                    </a>
                                </p>
                            }
                        }

                    </div>
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

