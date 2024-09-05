/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  safelist: ["border", "text", "bg"]
    .flatMap((s) => ["red", "blue", "gray"].map((c) => `${s}-${c}`))
    .flatMap((s) =>
      ["400", "500", "600", "700", "800", "900", "950"].map((n) => `${s}-${n}`)
    )
    .flatMap((s) => ["", "hover:", "active:", "focus:"].map((m) => `${m}${s}`))
    .flatMap((s) => ["", "dark:"].map((m) => `${m}${s}`)),
  theme: {
    extend: {
      boxShadow: {
        right: "4px 0 6px -1px rgb(0 0 0 / 0.1)",
      },
      colors: {
        neutral: {
          750: "#333333",
        },
      },
    },
  },
  plugins: [],
};
