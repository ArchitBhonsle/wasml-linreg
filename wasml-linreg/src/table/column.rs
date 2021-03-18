use wasm_bindgen::prelude::*;

use crate::warray::Warray;

#[wasm_bindgen]
pub struct Column {
    #[wasm_bindgen(skip)]
    pub warray: Warray,
    
    #[wasm_bindgen(skip)]
    pub header: String,
}

#[wasm_bindgen]
impl Column {
    #[wasm_bindgen(constructor)] 
    pub fn new(header: String, warray: Warray) -> Column {
        Column {
            warray,
            header,
        }
    }
    
    #[wasm_bindgen(getter)]
    pub fn header(&self) -> String {
        self.header.clone()
    }

    #[wasm_bindgen(getter, js_name = warray)]
    pub fn warray_to_js(&self) -> js_sys::Array {
        self.warray.to_js()
    }
    
    
    #[wasm_bindgen(getter, js_name = data)]
    pub fn to_js(&self) -> js_sys::Map {
        let js_map = js_sys::Map::new();

        js_map.set(&JsValue::from_str(&self.header), &self.warray_to_js());

        js_map
    }
}


