const path = require('path');

module.exports = {
  mode: process.env.NODE_ENV === 'production' ? 'production' : 'development',
  devtool: process.env.NODE_ENV === 'production' ? undefined : undefined,
  entry: './dist-esm/index.js',
  target: 'node',
  output: {
    path: path.join(__dirname, 'dist/'),
    filename: 'bundle.js',
    libraryTarget: 'commonjs2',
  },
  module: {
    rules: [
      {
        test: /\.wasm$/,
        use: [
          {
            loader: 'wasm-loader',
            options: {
              dce: 1,
            },
          },
        ],
        type: 'javascript/auto',
      },
      {
        test: /\.rs$/,
        use: [
          {
            loader: 'wasm-loader',
            options: {
              dce: 1,
            },
          },
          {
            loader: 'rust-native-wasm-loader',
            options: {
              release: process.env.NODE_ENV === 'production',
            },
          },
        ],
      },
    ],
  },
};
