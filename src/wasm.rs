use wasm_bindgen::prelude::*;

pub static mut JS_POINT_AWARED_CALLBACK: Option<js_sys::Function> = None;

// all javascript callbacks
pub mod js {
    use super::*;

    pub fn on_point_awared() {
        unsafe {
            if let Some(callback) = &JS_POINT_AWARED_CALLBACK {
                callback.call0(&JsValue::NULL).unwrap();
            }
        }
    }
}

// Declares the interface that rust can call foreign code by.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn marco() {
    console_error_panic_hook::set_once();
    alert("Polo");
}

#[wasm_bindgen]
pub fn hoops_init(point_awared_callback: &js_sys::Function) {
    console_error_panic_hook::set_once();

    // I don't _want_ to use unsafe here, but I don't see an
    // obvious alternative.
    // WARNING: Be careful about passing references!
    unsafe {
        JS_POINT_AWARED_CALLBACK = Some(point_awared_callback.clone());
    }
}
