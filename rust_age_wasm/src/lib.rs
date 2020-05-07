use wasm_bindgen::prelude::*;

use secrecy::Secret;
use std::io::{Read, Write};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// https://rustwasm.github.io/docs/wasm-bindgen/examples/console-log.html?highlight=console.log#consolelog
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
}

#[wasm_bindgen]
pub fn interpret(input: String) -> i64 {
    log(&format!("hello from rust {}", &input));

    let res = scheme_lib::run(&input).unwrap();
    if let scheme_lib::Expression::Number(i) = *res {
        return i;
    } else {
        panic!("result could not be computed")
    }
}

// #[wasm_bindgen]
// pub fn sign(input: String) -> String {
//     log(&format!("hello from rust signify {}", &input));
//     // pub fn sign_lib(seckey: &[u8], msg: &[u8], embed: bool) -> Result<Vec<u8>> {
//     let seckey = "some secrecy";
//     String::from_utf8(signify_lib::sign_lib(&seckey.as_bytes(), &input.as_bytes(), true).unwrap())
//         .unwrap()
// }

// #[wasm_bindgen]
// pub fn verify(input: String) -> String {
//     log(&format!("hello from rust signify {}", &input));
//     // pub fn verify_lib(pubkey: &[u8], msg: &[u8], signature_data: &[u8], embed: bool) -> Result<()> {

//     let pubkey = "some pubkey";
//     let signature_data = "some signature_data";

//     let res = signify_lib::verify_lib(
//         &pubkey.as_bytes(),
//         &input.as_bytes(),
//         &signature_data.as_bytes(),
//         true,
//     );

//     match res {
//         Ok(()) => "verfied as ok".to_string(),
//         Err(e) => format!("not ok {}", e),
//     }
// }

#[wasm_bindgen]
pub fn enc(plaintext: String, passphrase: String) -> Vec<u8> {
    let encryptor = age::Encryptor::with_user_passphrase(Secret::new(passphrase.to_owned()));

    log(&format!("hello from rust"));

    let mut encrypted = vec![];
    // https://docs.rs/age/0.4.0/age/struct.Encryptor.html#method.wrap_output
    let mut writer = encryptor
        .wrap_output(&mut encrypted, age::Format::Binary)
        .unwrap();

    writer.write_all(plaintext.as_bytes()).unwrap();
    writer.finish().unwrap();

    encrypted
    // vec![1, 2, 3]
}

// https://rustwasm.github.io/docs/wasm-bindgen/reference/types/number-slices.html?highlight=vec,u8#number-slices--u8---i8---u16---i16---u32---i32---u64---i64---f32--and--f64
#[wasm_bindgen]
pub fn dec(encrypted: &[u8], passphrase: String) -> String {
    let decryptor = match age::Decryptor::new(&encrypted[..]).unwrap() {
        age::Decryptor::Passphrase(d) => d,
        _ => unreachable!(),
    };

    let mut decrypted = vec![];
    let mut reader = decryptor
        .decrypt(&Secret::new(passphrase.to_owned()), None)
        .unwrap();
    reader.read_to_end(&mut decrypted);

    // // TODO maye turn into vec u8 again
    std::str::from_utf8(&decrypted).unwrap().to_string()
    // "entschlüsselt".to_string()
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
