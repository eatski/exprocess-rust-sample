const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const CopyPlugin = require("copy-webpack-plugin");
const webpack = require("webpack");

const BUILD_MODE = "BUILD_MODE";

/** @type import('webpack').Configuration */
module.exports = {
  mode: process.env[BUILD_MODE] === "dev" ? "development" : "production",
  entry: {
    app: path.resolve(__dirname, "index.ts"),
  },
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "[name].[contenthash].js",
  },
  module: {
    rules: [
      {
        test: [/\.js$/, /\.ts$/],
        exclude: /node_modules/,
        loader: "esbuild-loader",
        options: {
            loader: 'ts',
        }
      }
    ],
  },
  resolve: {
    extensions: [".js", ".jsx", ".ts", ".tsx"],
  },
  experiments: {
    asyncWebAssembly: true,
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, "index.html"),
      filename: "index.html",
    }),
    new webpack.EnvironmentPlugin({
      [BUILD_MODE]: null
    }),
    new CopyPlugin({
      patterns: [
        { from: "assets", to: "assets" },
        { from: "node_modules/bulma/css", to: "bulma" },
      ],
    }),
  ],
};
