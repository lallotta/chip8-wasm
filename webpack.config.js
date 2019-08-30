const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyPlugin = require('copy-webpack-plugin');

module.exports = {
    entry: './bootstrap.js',
    output: {
        path: __dirname + '/dist',
        filename: 'bootstrap.js',
    },
    plugins: [
        new CopyPlugin([
            { from: 'roms/', to: 'roms/' },
            'index.html'
        ]),
        new WasmPackPlugin({
            crateDirectory: __dirname,
            extraArgs: "--no-typescript",
            outName: "chip8_wasm"
        })
    ],
    mode: 'production'
};