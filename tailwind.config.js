/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["tpl/**/*.tpl", "./src/**/*.{html,js}"],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
};
