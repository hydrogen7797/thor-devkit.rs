/** @type {import('tailwindcss').Config} */
export default {
  content: { 
    files: ["*.html", "./src/**/*.rs"],
  },
  // corePlugins: {
  //   preflight: false,
  // },
  theme: {
    extend: {},
  },
  plugins: [require("daisyui"), require('flowbite/plugin')],
}

