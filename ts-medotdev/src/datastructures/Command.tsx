import type { ReactNode } from "react";
import { LogStack } from "./LogStack";
import LinkButton from "~/components/LinkButton";
import Image from "next/image";

export enum PALETTE {
  PROGRAMMING = "programming",
  PICTURE = "picture",
  EDUCATION = "education",
  RESUME = "resume",
  CONTACT = "contact",
}

export abstract class Command {
  abstract execute(logStack?: LogStack): ReactNode;
}

// this is overkill but i love enums
export enum COMMANDS {
  LIST = "ls",
  CLEAR = "clear",
  HELP = "help",
  RESUME = "resume",
  PROGRAMMING = "programming",
  EDUCATION = "education",
  CONTACT = "contact",
  PICTURE = "picture",
}

export function generatePalette(): Record<string, string> {
  const paletteMap: Record<string, string> = {};

  const commandSet = Object.values(COMMANDS);
  const keySet = Object.values(PALETTE) as string[];

  for (const cmd of commandSet) {
    if (keySet.includes(cmd)) {
      paletteMap[cmd] = "text-[#b07db3]";
    } else {
      paletteMap[cmd] = "text-[#e87912]";
    }
  }

  return paletteMap;
}

class LSCommand extends Command {
  execute(): ReactNode {
    return (
      <div>
        {Object.values(PALETTE).map((item, index) => {
          return (
            <div key={index}>
              <p>{item}</p>
            </div>
          );
        })}
      </div>
    );
  }
}

class ClearCommand extends Command {
  execute(logStack: LogStack): ReactNode {
    logStack.clear();

    return <></>;
  }
}

class UnrecognizedCommand extends Command {
  private commandString: string;

  constructor(commandString: string) {
    super();
    this.commandString = commandString;
  }

  execute(): ReactNode {
    return (
      <div>
        <div>command not found: {this.commandString}</div>
      </div>
    );
  }
}

class EducationCommand extends Command {
  execute(): ReactNode {
    return (
      <div>
        <div>
          <p>
            Wesleyan University | BA Computer Science, Data Science Minor | May
            2023
          </p>
        </div>
      </div>
    );
  }
}

const images = [
  { title: "Me", href: "/rock.png" },
  { title: "My friend Cisco and I hiking Shenandoah", href: "/oldrag.png" },
  { title: "Family", href: "/fam.png" },
  { title: "My friend Nelson", href: "/menelson.png" },
  { title: "Grad", href: "/aye.png" },
  { title: "Johnny Cash Museum", href: "/cashman.png" },
];

class PictureCommand extends Command {
  static counter = 0;

  execute(_logStack?: LogStack | undefined): ReactNode {
    const index = PictureCommand.counter;

    PictureCommand.counter = (PictureCommand.counter + 1) % images.length;

    const { title, href } = images[index]!;

    return (
      <div className="w-3/4 lg:w-1/4">
        <Image
          src={href}
          alt={title}
          width={500} // You can specify a default width and height here.
          height={500}
          className="h-full w-full"
          loading="eager"
        />
      </div>
    );
  }
}

const data = [
  {
    title: "Software Engineer Intern",
    team: "Toast",
    date: "Jun 2023 - Aug 2023",
    desc: [
      {
        title: null,
        source: null,
        demo: null,
        value:
          "Shipped features to production in Kotlin and Typescript for Toast Tables, a waitlist and reservation service.",
      },
    ],
  },
  {
    title: "Software Engineer",
    team: "Delta Lab (Wesleyan University ML Research Lab)",
    date: "Aug 2021 - Sep 2022",
    desc: [
      {
        title: "SeekWMP",
        source: "https://github.com/friendlymatthew/seekwmp",
        demo: "https://seekwmp.vercel.app/?market=Portland&station=WPFO&title=DailyMailTV&snippet=be+taking+to+make+the+risks+are+worth+their+rewards+medical+staff+housekeeping+and+that+of+course+--+jesse:+jay+jacobs+the+ceo+of+the+timberlake+family+of+camps+is+talking+about+what+it+takes+to+operate+a+summer+camp+in+the+time+of+covid-19+before+the+recently+announced+new+york+ban+jacobs+had+planned+to+open+some+of+his&coder=eraab&url=covid/xWPFO_20200619_1100PM.mp4&id=4&seek=1357",
        value:
          "Engineered data labeling platform that delegates tasks to project members, facilitates data labeling production, and stores training data; ready to be consumed by ML model.",
      },
      {
        title: "2020 US Presidential Election Spending Interface",
        source: "https://github.com/friendlymatthew/facebookad",
        demo: "https://presidentwmp.vercel.app/",
        value:
          "Created interactive timeline visualizing 33-week Facebook ad data for the 2020 US election, analyzed spending data and imaged results with custom choropleth maps.",
      },
    ],
  },
  {
    title: "Machine Learning Teaching Assistant",
    team: "QAC239",
    date: "Jan 2023 - May 2023",
    desc: [
      {
        title: null,
        source: null,
        demo: null,
        value:
          "Conducted weekly office hours (4 hrs) assisting 20 students in ML theory and application proseminar. Helped shape 12-week curriculum, wrote and solved problem sets using Python, Pandas, Sci-kit Learn, Tensorflow.",
      },
    ],
  },
];

const skills = [
  {
    title: "Languages",
    desc: "Typescript, Kotlin, Java, Python",
  },
  {
    title: "Frameworks",
    desc: "Nextjs, Reactjs, Nodejs",
  },
  {
    title: "Cloud",
    desc: "AWS",
  },
  {
    title: "Databases",
    desc: "DynamoDB, MongoDB, PostgreSQL",
  },
  {
    title: "Tools",
    desc: "Vercel, Jenkins, Splunk, Git, Datadog, Sonarqube",
  },
];

class ProgrammingCommand extends Command {
  execute(_logStack?: LogStack | undefined): ReactNode {
    return (
      <div className="space-y-6 pt-2">
        <div className="">
          <div className="flex flex-col space-x-2 text-lg">
            <p className="text-xl font-semibold italic">Github</p>
            <LinkButton
              title="github.com/friendlymatthew"
              src="https://github.com/friendlymatthew"
            />
          </div>
        </div>
        <div className="flex flex-col space-x-2">
          <div className="w-fit text-xl font-semibold italic">Skills</div>
          <div className="text-lg lg:text-xl">
            {skills.map(({ title, desc }, index) => {
              return (
                <div key={index} className="flex space-x-2 text-base lg:text-lg">
                  <p className="font-normal">{title}:</p>
                  <p className="">{desc}</p>
                </div>
              );
            })}
          </div>
        </div>
        <div className="flex flex-col space-x-2">
          <div className="w-fit text-xl font-semibold italic">Experience</div>
          <div>
            {data.map(({ title, team, date, desc }, index) => {
              return (
                <div key={index} className="py-2">
                  <div className="text-lg font-normal lg:text-xl">
                    <p>
                      {title} | <span className="font-semibold ">{team}</span> |{" "}
                      {date}
                    </p>
                  </div>
                  <ul className="w-full space-x-1 text-base lg:w-3/4 lg:text-lg">
                    {desc.map(({ value, source, demo, title }, index) => {
                      return (
                        <li className="pb-2" key={index}>
                          <p>- {value}</p>
                          {source && demo && title && (
                            <div className="flex font-normal ">
                              <LinkButton title={title} src={demo} />
                              <span className="mx-2">|</span>
                              <LinkButton title="Source Code" src={source} />
                            </div>
                          )}
                        </li>
                      );
                    })}
                  </ul>
                </div>
              );
            })}
          </div>
        </div>
      </div>
    );
  }
}

async function downloadResume() {
  try {
    const response = await fetch("RESUME.pdf");
    const blob = await response.blob();
    const fileURL = window.URL.createObjectURL(blob);
    const alink = document.createElement("a");
    alink.href = fileURL;
    alink.download = "Matthew Kim Resume.pdf";
    alink.click();
  } catch (error) {
    console.error("Error downloading the resume:", error);
  }
}

class ResumeCommand extends Command {
  execute(_logStack?: LogStack | undefined): ReactNode {
    downloadResume()
      .then(() => {
        console.log("downloaded");
      })
      .catch((e) => {
        console.error(e);
      });

    return (
      <div className="">
        <div>
          <p>...Resume successfully downloaded...</p>
        </div>
        <div className="flex w-full justify-start">
          <p
            onClick={() => {
              downloadResume()
                .then(() => {
                  console.log("redownloaded");
                })
                .catch((e) => {
                  console.error(e);
                });
            }}
            className="font-semibold text-blue-400 hover:underline"
          >
            Click on me to redownload resume
          </p>
        </div>
      </div>
    );
  }
}

class ContactCommand extends Command {
  execute(_logStack?: LogStack | undefined): ReactNode {
    const data = [
      {
        title: "My Email",
        social: "matthewkmkim@gmail.com",
        href: "mailto:matthewkmkim@gmail.com",
      },
      {
        title: "My Linkedin",
        social: "/in/mat-thew",
        href: "https://linkedin.com/in/mat-thew",
      },
    ];
    return (
      <div>
        {data.map(({ title, social, href }, index) => {
          return (
            <div key={index} className="flex flex-col pb-2 text-lg">
              <p className="text-xl font-semibold italic">{title}</p>{" "}
              <LinkButton src={href} title={social} />
            </div>
          );
        })}
      </div>
    );
  }
}

class HelpCommand extends Command {
  execute(): ReactNode {
    const paletteMap = generatePalette();

    return (
      <div className="space-y-4 py-2">
        <div className="space-y-2">
          <p className="font-normal">Typed commands: </p>
          <div className="grid w-11/12 grid-cols-2 pl-4 lg:w-2/3 lg:grid-cols-3">
            {Object.keys(paletteMap).map((command: string, index) => {
              return (
                <p key={index} className="font-semibold text-orange-300">
                  {command}
                </p>
              );
            })}
          </div>
          <div className="flex flex-col py-4 text-lg">
            <p>
              Start typing commands, press{" "}
              <span className="text-xl font-bold text-purple-300">tab</span> to
              autocomplete.
            </p>
            <p>
              Use arrow keys to toggle between suggestions. Press{" "}
              <span className="text-xl font-bold text-purple-300">enter</span>{" "}
              on a suggestion to autocomplete.
            </p>
          </div>
        </div>
        <div>
          <p>
            You can also use the sidebar to{" "}
            <span className="font-bold">search</span>.
          </p>
        </div>
      </div>
    );
  }
}

export class CommandFactory {
  static createCommand(type: string): Command {
    switch (type) {
      case COMMANDS.LIST:
        return new LSCommand();
      case COMMANDS.CLEAR:
        return new ClearCommand();
      case COMMANDS.HELP:
        return new HelpCommand();
      case COMMANDS.PICTURE:
        return new PictureCommand();
      case COMMANDS.PROGRAMMING:
        return new ProgrammingCommand();
      case COMMANDS.RESUME:
        return new ResumeCommand();
      case COMMANDS.CONTACT:
        return new ContactCommand();
      case COMMANDS.EDUCATION:
        return new EducationCommand();
      default:
        return new UnrecognizedCommand(type);
    }
  }
}
