/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/**/*.{html,js}"],
  theme: {
    extend: {
      fontFamily: {
        playpen: ['"Playpen Sans"', "cursive"],
        montserrat: ["Montserrat", "sans-serif"],
      },
      keyframes: {
        fadein: {
          "0%": { opacity: 0 },
          "100%": { opacity: 1 },
        },
      },
      animation: {
        fadein: 'fadein 1s forwards',
      }
    },
    plugins: [],
  }
}
