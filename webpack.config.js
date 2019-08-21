const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
    entry: './bootstrap.js',
    output: {
        path: __dirname + '/dist',
        filename: 'bundle.js',
    },
    plugins: [
        new HtmlWebpackPlugin(),
        new WasmPackPlugin({
            crateDirectory: ".",
            extraArgs: "--no-typescript",
            outName: "chip8_wasm"
        })
    ],
    mode: 'development'
};