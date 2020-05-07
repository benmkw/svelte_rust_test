import App from './App.svelte';

import wasm from './../../rust_age_wasm/Cargo.toml';

const init = async () => {
    const wasmer = await wasm();

    const app = new App({
        target: document.body,
        props: {
            enc: wasmer.enc,
            dec: wasmer.dec,
            interpret: wasmer.interpret,
            sign: wasmer.sign,
            verify: wasmer.verify,
        }
    });

};

init();
