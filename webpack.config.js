const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const webpack = require("webpack");

const BUILD_MODE = "BUILD_MODE";

/** @type import('webpack').Configuration */
module.exports = {
  mode: ["dev"].includes(process.env[BUILD_MODE]) ? "development" : "production",
  entry: {
    app: path.resolve(__dirname, "index.ts"),
  },
  output: {
    path: path.resolve(__dirname, "public"),
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
      template: path.resolve(__dirname, "index.ejs"),
      filename: "index.html",
      inject: false,
      templateParameters(_a,_b,tags) {
        return {
          scripts: tags.headTags
        }
      }
    }),
    new webpack.EnvironmentPlugin({
      [BUILD_MODE]: null
    }),
  ],
};
