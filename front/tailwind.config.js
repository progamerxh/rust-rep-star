/** @type {import('tailwindcss').Config} */
module.exports = {
    mode: "all",
    content: [
        // Include all Rust, HTML, and CSS files in the src directory
        "./src/**/*.{rs,html,css}",
        // Include all HTML files in the output (dist) directory
        "./dist/**/*.html",
    ],
    theme: {
        extend: {
            animation: {
                'border': 'border 4s linear infinite',
            },
            keyframes: {
                'border': {
                    to: { '--border-angle': '360deg' },
                }
            }
        },
    },
    plugins: [
        require('tailwindcss-animated')
    ]
}