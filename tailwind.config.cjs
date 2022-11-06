/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "index.html",
    "./src/**/*",
  ],
  theme: {
    extend: {},
  },
  plugins: [require('daisyui')],
}
