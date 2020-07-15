import rust from "@wasm-tool/rollup-plugin-rust";

export default {
    input: {
        index: "./Cargo.toml",
    },
    output: {
        dir: "public/wasm/",
        format: "iife",
        sourcemap: false,
    },
    plugins: [
        rust({
            serverPath: `wasm/`,
			cargoArgs: ["--features", "release"],
            debug: false,
        }),
    ],
};
