use wasm_bindgen::prelude::*;

// all javascript callbacks
pub mod js {
    use super::*;

    pub fn on_point_awared() {
        onBasket();
    }
}

// Declares the interface that rust can call foreign code by.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn onBasket();
}

#[wasm_bindgen]
pub fn marco() {
    console_error_panic_hook::set_once();
    alert("Polo");
}

#[wasm_bindgen]
pub fn hoops_init() {
    console_error_panic_hook::set_once();
}
