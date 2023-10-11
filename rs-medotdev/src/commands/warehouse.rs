use leptos::*;
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
                <div class="w-full flex justify-center text-2xl">
                    <div class="px-1">
                        <p>
                            Hello! I am a recent graduate, software engineer, and open source contributor.

                        </p>
                        <p>
                            I am currently building projects in Rust. Previously, I interned at Toast, a restaurant point-of-sale company, where I worked on the waitlist and reservation service.
                        </p>
                        <br />
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
                        <br />
                        <p>
                            I train jiu jitsu and enjoy nature. I listen to country music,
                            <a
                                href="https://www.youtube.com/watch?v=2wGuHHY11SM"
                                target="_blank"
                                rel="noreferrer"
                                class="underline hover:decoration-dotted decoration-solid underline-offset-4 decoration-[#0b7261] decoration-4"
                            >
                                The Eagles
                            </a>, and
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
            view! {
                <div>
                    <p>
                        Picture
                    </p>
                </div>
            }
        }
        Command::Education => {
            view! {
                <div>
                    <p>I recieved a B.A. in Computer Science and Data Science, Wesleyan University, 2023 </p>
                </div>
            }
        }
        Command::Resume => {
            view! {
                <div> Resume </div>
            }
        }
        Command::Contact => {
            view! {
                <div>
                    <p>email: matthewkmkim@gmail.com</p>
                    <p>linkedin: /in/mat-thew</p>
                </div>
            }
        }
        Command::Help => {
            view! {
                <div> Help </div>
            }
        }
        Command::Ls => {
            view! {
                <div> Ls </div>
            }
        }
        Command::Clear => {
            view! {
                <div>clear</div>
            }
        }
        Command::Rogue => {
            view! {
                <div>
                    <p> Unrecognized command: {&curr_command} </p>
                </div>
            }
        }
    }
}

