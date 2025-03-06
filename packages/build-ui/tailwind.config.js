/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      colors: {
        primary: {
          DEFAULT: "#30D4A0"
        },
        background: "#171717",
      }
    },
  },
  plugins: [],
};
