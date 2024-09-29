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
      keyframes: {
        fadeIn: {
          "0%": { opacity: "0" },
          "100%": { opacity: "1" },
        },
      },
      animation: {
        "fade-in": "fadeIn 0.5s ease-in-out",
      },
      animation: {
        fadein: "fadein 1s forwards",
      },
    },
    plugins: [],
  },
};
