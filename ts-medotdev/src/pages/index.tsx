import React, { useEffect, useState, useRef, FocusEvent } from "react";
import Head from "next/head";
import Link from "next/link";
import { CommandFactory, generatePalette } from "~/datastructures/Command";
import { LogStack } from "~/datastructures/LogStack";
import { Trie } from "~/datastructures/Autocomplete";

import { PALETTE } from "~/datastructures/Command";

type Color = {
  id: PALETTE;
  title: string;
  component: React.FC;
};

const commandSet = new Set<string>([
  "help",
  "ls",
  "clear",
  "education",
  "resume",
  "picture",
  "contact",
  "programming",
]);

const suggestionsLexer = {
  help: "further clarification and details how to navigate",
  ls: "lists key commands",
  clear: "clean slates current terminal view",
  education: "details education",
  picture: "picture of me",
  contact: "lists contact detail",
  programming: "view programming experience",
  resume: "download and view resume",
};

// if we're good and content with how it looks, the goal is to make it so that you only need to update data and types

export default function Home() {
  const transformToNav = Object.values(PALETTE).filter((color) =>
    isNaN(Number(color))
  ) as PALETTE[];

  const [navbar, setNavbar] = useState<string[]>(transformToNav);
  const inputRef = useRef<HTMLInputElement | null>(null);
  const logStackRef = useRef(new LogStack());
  const logContainerRef = useRef<HTMLDivElement | null>(null);
  const [_, forceUpdate] = useState({});
  const [inputVal, setInputVal] = useState("");
  const [suggestions, setSuggestions] = useState<string[]>([]);
  const [trie, setTrie] = useState<Trie>(() => {
    const t = new Trie();
    commandSet.forEach((command) => t.insert(command));
    return t;
  });
  const [selectedSuggestionIndex, setSelectedSuggestionIndex] = useState(-1);
  const [discourageSuggestion, setDiscourageSuggestion] = useState(false);
  const paletteMap = generatePalette();

  const handleCommandSubmit = (cmd: string) => {
    const commandInstance = CommandFactory.createCommand(cmd.toLowerCase());
    const res = commandInstance.execute(logStackRef.current);

    if (cmd.toLowerCase() !== "clear") {
      logStackRef.current.push({ command: cmd, response: res });
    }

    forceUpdate({});

    if (logContainerRef.current) {
      logContainerRef.current.scrollTop = 0;
    }
  };

  const handleKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    switch (event.key) {
      case "Enter":
        if (
          selectedSuggestionIndex > -1 &&
          selectedSuggestionIndex < suggestions.length
        ) {
          handleCommandSubmit(suggestions[selectedSuggestionIndex]!);
          setInputVal("");
          setSelectedSuggestionIndex(-1);
        } else if (inputRef.current) {
          const inputValue = inputRef.current.value;
          handleCommandSubmit(inputValue);
          setInputVal("");
          setDiscourageSuggestion(false);
        }
        break;
      case "ArrowDown":
        setSelectedSuggestionIndex((prevIndex) =>
          Math.min(prevIndex + 1, suggestions.length - 1)
        );
        break;
      case "ArrowUp":
        setSelectedSuggestionIndex((prevIndex) => Math.max(prevIndex - 1, -1));
        break;
      case "Escape":
        setDiscourageSuggestion(true);
        break;
      case "Tab":
        event.preventDefault();
        if (suggestions.length > 0 ) {
          setInputVal(suggestions[0] ?? '');
          setSelectedSuggestionIndex(-1);
        }
        break;
      default:
        break;
    }
  };

  function splitSuggestion(
    suggestion: string,
    inputValue: string
  ): [string, string] {
    return [inputValue, suggestion.substr(inputValue.length)];
  }

  useEffect(() => {
    if (inputVal) {
      setSuggestions(trie.search(inputVal.toLowerCase()));
      setDiscourageSuggestion(false);
    } else {
      setSuggestions([]);
    }
  }, [inputVal, trie]);

  return (
    <>
      <Head>
        <title>Matthew Kim</title>
        <meta name="description" content="mat-thew" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <link
          rel="apple-touch-icon"
          sizes="180x180"
          href="/apple-touch-icon.png"
        />
        <link
          rel="icon"
          type="image/png"
          sizes="32x32"
          href="/favicon-32x32.png"
        />
        <link
          rel="icon"
          type="image/png"
          sizes="16x16"
          href="/favicon-16x16.png"
        />
        <link rel="manifest" href="/site.webmanifest" />
        <link rel="preconnect" href="https://fonts.googleapis.com" />
        <link
          rel="preconnect"
          href="https://fonts.gstatic.com"
          crossOrigin="anonymous"
        />
        <link
          href="https://fonts.googleapis.com/css2?family=Roboto+Mono:wght@300;400;600;700&display=swap"
          rel="stylesheet"
        />
      </Head>
      <main className="cursor-custom flex h-screen flex-col items-center overflow-auto bg-[#383a3c] py-4 font-mono font-normal tracking-tight text-[#e1e4e7] lg:py-8">
        <div className="flex h-full w-11/12 flex-col justify-start space-y-2 lg:grid lg:w-10/12 lg:flex-none lg:grid-cols-6">
          <div className="flex flex-col space-y-1 md:space-y-8">
            <div>
              <p className="text-lg font-semibold md:text-3xl">Matthew Kim</p>
            </div>
            <div className="grid grid-cols-2 space-y-4 divide-[#606060] text-lg lg:grid-cols-1 lg:divide-y lg:text-xl ">
              <div>
                {navbar.map((color, index) => {
                  return (
                    <div key={index}>
                      <button
                        className="font-normal hover:underline"
                        key={index}
                        onClick={() => {
                          setInputVal("");
                          handleCommandSubmit(color);
                        }}
                      >
                        {color}
                      </button>
                    </div>
                  );
                })}
              </div>
              <div className="flex flex-col items-start lg:pt-4">
                <button
                  className="font-normal hover:underline"
                  onClick={() => {
                    setInputVal("");
                    handleCommandSubmit("help");
                  }}
                >
                  <p>help</p>
                </button>
                <button
                  className="font-normal hover:underline"
                  onClick={() => {
                    setInputVal("");
                    handleCommandSubmit("clear");
                  }}
                >
                  <p>clear</p>
                </button>
              </div>
            </div>
          </div>
          <div
            onClick={() => {
              inputRef.current?.focus();
            }}
            className="relative flex h-full flex-col border-0 border-[#606060] bg-[#313131] p-1 md:col-span-5 lg:border-l"
          >
            <div className="flex w-full items-center space-x-2 px-1 text-lg font-medium">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                strokeWidth={3}
                stroke="currentColor"
                className="h-4 w-4"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  d="M8.25 4.5l7.5 7.5-7.5 7.5"
                />
              </svg>

              <div className="relative inline-block h-8 flex-grow">
                <input
                  type="text"
                  placeholder={`${
                    logStackRef.current.getAll().length === 0
                      ? "type 'help' to get started"
                      : ""
                  }`}
                  value={inputVal.toLowerCase()}
                  onChange={(e) => setInputVal(e.target.value)}
                  ref={inputRef}
                  onKeyDown={handleKeyDown}
                  className="w-full bg-[#313131] text-lg focus:outline-none lg:text-xl"
                />
              </div>
            </div>
            {suggestions.length > 0 && !discourageSuggestion && (
              <div className="absolute left-0 top-8 z-10 flex w-fit flex-col justify-start border border-l-0 border-t-0 border-black bg-[#505254]">
                {suggestions.map((suggestion, index) => {
                  const [match, afterMatch] = splitSuggestion(
                    suggestion,
                    inputVal
                  );
                  return (
                    <div
                      key={suggestion}
                      className={` max-w-8/12 flex cursor-pointer justify-between space-x-3 text-lg font-normal ${
                        index === selectedSuggestionIndex
                          ? "bg-[#5b5f62] italic"
                          : ""
                      }`}
                      onClick={() => {
                        setInputVal(suggestion);
                        setTimeout(() => {
                          setInputVal("");
                        }, 20);
                        handleCommandSubmit(suggestion);
                      }}
                    >
                      <div className="flex px-2 ">
                        <p className={`font-bold text-green-400`}>
                          {match.toLowerCase()}
                        </p>
                        <p>{afterMatch.toLowerCase()}</p>
                      </div>
                      <div className="px-2">
                        {suggestionsLexer[
                          suggestion as keyof typeof suggestionsLexer
                        ] != null &&
                          suggestionsLexer[
                            suggestion as keyof typeof suggestionsLexer
                          ]}
                      </div>
                    </div>
                  );
                })}
              </div>
            )}
            <div
              ref={logContainerRef}
              className="h-10 flex-grow cursor-text overflow-y-auto p-1 pt-2 text-lg lg:max-h-[calc(100vh-8rem)]"
            >
              {logStackRef.current
                .getAll()
                .map(({ command, response }, index) => (
                  <div key={index} className="py-[0.5px]">
                    <div className="flex items-center space-x-2 font-medium">
                      <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        strokeWidth={3}
                        stroke="currentColor"
                        className="h-4 w-4"
                      >
                        <path
                          strokeLinecap="round"
                          strokeLinejoin="round"
                          d="M8.25 4.5l7.5 7.5-7.5 7.5"
                        />
                      </svg>
                      <p
                        className={`${
                          Object.keys(paletteMap).includes(command)
                            ? "text-green-300"
                            : "text-red-300"
                        }`}
                      >
                        {" "}
                        {command}
                      </p>
                    </div>
                    <div className="w-full">{response}</div>
                  </div>
                ))}
            </div>
          </div>
        </div>
      </main>
    </>
  );
}
