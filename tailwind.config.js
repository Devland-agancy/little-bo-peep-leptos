/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs", "**/*.emu"],
  },
  corePlugins: {
    preflight: false,
  },
  theme: {
    extend: {
      fontFamily: {
        baskerville: ["Baskerville Regular"],
        "baskerville-bold": ["Baskerville Bold"],
        "baskerville-italic": ["Baskerville Italic"],
        clickerscript: ["Clicker Script"],
        merriweather: ["Merriweather"],
        "merriweather-bold": ["Merriweather Bold"],
        "merriweather-black": ["Merriweather Black"],
        "lora-italic": ["Lora Italic"],
        lora: ["Lora"],
        BowlbyOne: ["BowlbyOne"],
      },
      width: {
        128: "32.5rem",
        192: "42rem",
        256: "64rem",
        300: "100rem",
        512: "128rem",
        "3/1": "300%",
      },
      strokeWidth: {
        1: "1.5px",
        2: "3px",
      },
      animation: {
        appear: "appear 0.3s ease 0s 1 normal forwards",
        "appear-slow": "appear 1s ease 0s 1 normal forwards",

        /*  height_increase: "height_increase 1s ease 0.2s 1 normal forwards" */
      },
      transitionProperty: {
        dropdown: "all 1s",
        "image-scale": "all 300ms ease-in-out",
      },
      keyframes: {
        appear: {
          "0%": { opacity: "0%" },
          "100%": { opacity: "100%" },
        },
        /* height_increase: {
          "0%": { height: "0" },
          "100%": { height: "100%" },
        }, */
      },
      screens: {
        sm: "520px",
        "menu-button-bg": "580px",
      },
    },
  },
  plugins: [],
};
