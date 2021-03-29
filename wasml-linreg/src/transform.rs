use wasm_bindgen::prelude::*;
use std::collections::HashMap;

use crate::table::table::*;
use crate::table::cell::*;

#[wasm_bindgen]
pub struct Transform {
    #[wasm_bindgen(skip)]
    pub normalize: HashMap<String, Box<dyn Fn(f64) -> f64>>,
    #[wasm_bindgen(skip)]
    pub encoding: HashMap<String, Box<dyn Fn(String) -> f64>>,
}

#[wasm_bindgen]
impl Transform {
    #[wasm_bindgen(constructor)]
    pub fn new(table: &Table) -> Result<Transform, JsValue>  {
        if !table.is_uniform() {
            return Err(JsValue::from_str("Table has to be uniform"));
        }
        
        let mut normalize: HashMap<String, Box<dyn Fn(f64) -> f64>> = HashMap::new();
        let mut encoding: HashMap<String, Box<dyn Fn(String) -> f64>> = HashMap::new();

        table.data
            .iter()
            .for_each(|(header, column)| {
                if column.is_number() {
                    if let (Cell::Number(min), Cell::Number(max)) 
                        = (column.min(), column.max()) {
                        normalize.insert(
                            header.clone(),
                            Box::new(move |n| {
                                (n - min) / (max - min)
                            })
                        );
                    } 
                } else {
                    let mut encoding_map: HashMap<String, f64> = HashMap::new();
                    let mut next_code: f64 = 0.0;
                    column.data.iter().for_each(|cell| {
                        if let Cell::String(s) = cell.clone() {
                            if !encoding_map.contains_key(&s) {
                                encoding_map.insert(s, next_code);
                                next_code += 1.0;
                            }
                        }
                    });
                    encoding.insert(
                        header.clone(),
                        Box::new(move |s| {
                            encoding_map.get(&s).unwrap_throw().clone()
                        })
                    );
                }
            });

        Ok(Transform {
            normalize,
            encoding,
        })
    }
}

