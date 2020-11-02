const path = require('path');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");


module.exports = {
    webpack: (config, { defaultLoaders }) => {
        config.plugins = [
            ...config.plugins,
            new WasmPackPlugin({
                crateDirectory: path.resolve(__dirname, "./renderer")
            })
        ]

        return config
    }
}