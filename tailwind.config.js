/** @type {import('tailwindcss').Config} */
module.exports = {
  content: { 
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      fontFamily: {
        'audiowide': ['Audiowide'],
        'baskerville': ['Baskerville Regular'],
        'baskerville-bold': ['Baskerville Bold'],
        'baskerville-italic': ['Baskerville Italic'],
        'clickerscript': ['Clicker Script'],
        'indieflower': ['IndieFlower'],
        'menlo': ['Menlo'],
        'pollerone': ['PollerOne'],
        'princessjohn': ['PrincessJohn'],
        'songmyung': ['SongMyung'],
        'tomorrow': ['Tomorrow']
      },
      width: {
        '128': '32.5rem',
        '192': '42rem',
        '256': '64rem',
        '300': '100rem',
        '512': '128rem',
        '3/1': '300%'
      },
      strokeWidth: {
        '1': '1.5px',
        '2': '3px',
      },
      animation: {
        appear: 'appear 0.3s ease 0s 1 normal forwards'
      },
      keyframes: {
        appear: {
          '0%': { opacity: '0%' },
          '100%': { opacity: '100%' },
        }
      }
    },
  },
  plugins: [],
}
