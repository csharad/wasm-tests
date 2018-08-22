#[cfg(test)]
extern crate wasm_bindgen_test;

#[cfg(test)]
use wasm_bindgen_test::*;

#[cfg(test)]
wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
#[wasm_bindgen_test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[cfg(test)]
mod mod_test {
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn should_work() {
        assert_eq!(2 + 2, 4);
    }
}

mod file_mod;
