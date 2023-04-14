use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

// #[wasm_bindgen]
// pub fn greet(who: &str) -> String {
//     format!("hello {who}")
// }

// #[wasm_bindgen]
// extern {
//     fn alert(text: &str);
// }
