use wasm_bindgen::prelude::*;

pub enum WarrayType {
    Boolean,
    Number,
    String,
}

pub trait Warrable {
    fn to_js(&self) -> js_sys::Array;
    fn get_type(&self) -> WarrayType;
}

impl Warrable for Vec<bool> {
    fn to_js(&self) -> js_sys::Array {
        let js_array = js_sys::Array::new_with_length(self.len() as u32);

        self.iter().enumerate().for_each(|(idx, ele)| {
            js_array.set(idx as u32, JsValue::from(*ele));
        });

        js_array
    }

    fn get_type(&self) -> WarrayType {
        WarrayType::Boolean
    }
}

impl Warrable for Vec<f64> {
    fn to_js(&self) -> js_sys::Array {
        let js_array = js_sys::Array::new_with_length(self.len() as u32);

        self.iter().enumerate().for_each(|(idx, ele)| {
            js_array.set(idx as u32, JsValue::from(*ele));
        });

        js_array
    }
    
    fn get_type(&self) -> WarrayType {
        WarrayType::Number
    }
}

impl Warrable for Vec<String> {
    fn to_js(&self) -> js_sys::Array {
        let js_array = js_sys::Array::new_with_length(self.len() as u32);

        self.iter().enumerate().for_each(|(idx, ele)| {
            js_array.set(idx as u32, JsValue::from(ele));
        });

        js_array
    }
    
    fn get_type(&self) -> WarrayType {
        WarrayType::String
    }
}

#[wasm_bindgen]
pub struct Warray {
    #[wasm_bindgen(skip)]
    pub warray: Box<dyn Warrable>,
}

#[wasm_bindgen]
impl Warray {
    #[wasm_bindgen(constructor)]
    pub fn new(js_array: js_sys::Array, type_code: &str) -> Result<Warray, JsValue> {
        match type_code {
            "b" | "bool" | "boolean" => Ok(Warray {
                warray: Box::new(
                    js_array.iter()
                    .map(|x| x.as_bool().expect_throw("mismatched types"))
                    .collect::<Vec<bool>>())
            }),
            "n" | "num" | "number" => Ok(Warray {
                warray: Box::new(
                    js_array.iter()
                    .map(|x| x.as_f64().expect_throw("mismatched types"))
                    .collect::<Vec<f64>>())
            }),
            "s" | "str" | "string" => Ok(Warray {
                warray: Box::new(
                    js_array.iter()
                    .map(|x| x.as_string().expect_throw("mismatched types"))
                    .collect::<Vec<String>>())
            }),
            _ => Err(JsValue::from_str("invalid type"))
        }
    }
}