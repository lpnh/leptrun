/** @type {import('tailwindcss').Config} */
module.exports = {
    content: { 
        files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
        extend: {},
        fontFamily: {
            'fira-mono': ['"Fira Mono"', 'monospace'],
        },
    },
    plugins: [],
}

