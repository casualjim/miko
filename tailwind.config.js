const SIDEBAR_WIDTH = 320;

module.exports = {
  content: ["*.html", "./src/**/*.rs",],
  safeList: [
    "bg-primary",
    "text-primary-content",
    "bg-secondary",
    "text-secondary-content",
  ],
  theme: {
    extend: {
      typography: () => ({
        DEFAULT: {
          css: {
            pre: {
              textWrap: "pretty",
            }
          }
        }
      }),
      width: {
        sidebar: SIDEBAR_WIDTH
      },
      minWidth: {
        sidebar: SIDEBAR_WIDTH
      },
      animation: {
        'fade-in': 'fade 300ms ease-in-out forwards',
        'fade-out': 'fade 300ms ease-in-out reverse forwards',
        'slide-down': 'slide-down 500ms ease-in-out forwards',
        'slide-up': 'slide-up 500ms ease-in-out forwards'
      },
      keyframes: {
        'fade': {
          "0%": { opacity: 0 },
          "100%": { opacity: 1 }
        },
        'slide-up': {
          "0%": { transform: 'translateY(10%)', opacity: 0 },
          "100%": { transform: 'translateY(0%)', opacity: 1 }
        },
        'slide-down': {
          "0%": { transform: 'translateY(-10%)', opacity: 0 },
          "100%": { transform: 'translateY(0%)', opacity: 1 }
        },
      }
    },
  },
  daisyui: {
    logs: false,
    themes: ["night", "pastel", "fantasy", "light", "dark"]
  },
  plugins: [
    require("@tailwindcss/typography"),
    require("daisyui")
  ],
}
