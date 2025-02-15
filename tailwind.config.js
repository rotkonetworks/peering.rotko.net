/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: [
    "./src/**/*.{rs,html,css}",
    "./dist/**/*.html",
    "./**/src/**/*.{rs,html,css}"
  ],
  theme: {
    extend: {},
  },
  safelist: [
    "text-white",
    "hover:bg-blue-700"
  ],
  plugins: [],
};
