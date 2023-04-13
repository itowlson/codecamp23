use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(left: i32, right: i32) -> i32 {
    alert(&format!("Adding {left} and {right}"));
    left + right
}

#[wasm_bindgen]
extern {
    fn alert(text: &str);
}
