const path = require("path");
const webpack = require("webpack"); // Import webpack

module.exports = {
  entry: "./src/index.ts",
  mode: "production",
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: [".ts", ".js"],
    fallback: {
      "buffer": require.resolve("buffer/") // Add fallback for buffer
    }
  },
  output: {
    filename: "bundle.js",
    path: path.resolve(__dirname, "dist"),
  },
  performance: {
    hints: "warning",
    maxAssetSize: 1024 * 1024,
    maxEntrypointSize: 1024 * 1024,
  },
  devServer: {
    static: {
      directory: path.join(__dirname, 'dist'),
    },
    compress: true,
    port: 9000,
    open: true,
  },
  plugins: [ // Add plugins section
    new webpack.ProvidePlugin({
      Buffer: ['buffer', 'Buffer'], // Provide Buffer global
    }),
  ],
};
