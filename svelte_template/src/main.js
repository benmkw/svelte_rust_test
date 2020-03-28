import App from './App.svelte';
import wasm from './../../rust_wasm/Cargo.toml';


const init = async () => {
    const wasmer = await wasm();
    const w_add = wasmer.add;

    const app = new App({
        target: document.body,
        props: {
            add: w_add
        }
    });

};

init();
