use wasm_bindgen::prelude::*;
use super::cell::*;

#[wasm_bindgen]
pub struct Column {
    #[wasm_bindgen(skip)]
    pub header: String,
    
    #[wasm_bindgen(skip)]
    pub data: Vec<Cell>,
}

impl Column {
    pub fn new(header: String, data_as_string: Vec<String>) -> Result<Column, JsValue> {
        let data = data_as_string
            .iter()
            .map(|x| {
                Cell::new(x)
            })
            .collect();
        
        Ok(Column {
            header,
            data,    
        })
    }
}

#[wasm_bindgen]
impl Column {
    #[wasm_bindgen(constructor)]
    pub fn new_from_js(header: String, js_array: js_sys::Array) -> Result<Column, JsValue> {
        let data = js_array
            .iter()
            .map(|x| {
                Cell::from_js(x).unwrap_throw()
            })
            .collect();
        
        
        Ok(Column {
            header,
            data,    
        })
    }
    
    #[wasm_bindgen(getter, js_name = data)]
    pub fn data_to_js(&self) -> js_sys::Array {
        let column_array = js_sys::Array::new_with_length(self.data.len() as u32);

        self.data.iter().enumerate().for_each(|(idx, cell)| {
            column_array.set(idx as u32, cell.to_js());
        });
            
        column_array
    }
    
    #[wasm_bindgen(getter)]
    pub fn header(&self) -> String {
        self.header.clone()
    }
}