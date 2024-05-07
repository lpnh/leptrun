/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs"],
  theme: {
    extend: {},
    fontFamily: {
      'fira-mono': ['"Fira Mono"', 'monospace'],
    },
  },
  plugins: [],
}

