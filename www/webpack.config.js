const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
      new CopyWebpackPlugin({
          patterns: [
              { from: "index.html", to: "index.html" },
              { from: "index.css", to: "index.css" },
          ],
      }),
  ],
    experiments: {
        // asyncWebAssembly: true,
        syncWebAssembly: true,
    },
};
