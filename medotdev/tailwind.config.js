/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      fontFamily: {
        'lora': ['Lora', 'serif'],
        'poppins': ['Poppins', 'sans-serif'],
      }
    },
  },
  plugins: [],
}

