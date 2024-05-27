module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
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
