/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs", "./input.css"],
  },
  theme: {
    extend: {
      fontFamily: {
        'lora': ['Lora', 'serif'],
        'poppins': ['Poppins', 'sans-serif'],
        'robotomono': ['Roboto Mono', 'monospace']
      }
    },
  },
  plugins: [],
}

