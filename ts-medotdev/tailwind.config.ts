import { type Config } from "tailwindcss";

export default {
  content: ["./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      fontFamily: {
        mono: "'Roboto Mono', monospace",
      },
      backgroundColor: {
        "input-cursor": "#A0A0A0",
      },
      caretColor: {
        transparent: "transparent",
      },
      keyframes: {
        blink: {
          "0%, 49%": { backgroundColor: "#A0A0A0" },
          "50%, 100%": { opacity: "transparent" },
        },
      },
      animation: {
        blink: "blink 1s infinite",
      },
    },
  },
  plugins: [],
} satisfies Config;
