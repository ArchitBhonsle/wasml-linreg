use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub enum Cell {
    String(String),
    Number(f64),
    Boolean(bool),
}

impl Cell {
    pub fn new(cell: &str) -> Cell {
        if let Ok(parsed) = cell.parse::<f64>() {
            return Cell::Number(parsed);
        }
        if let Ok(parsed) = cell.parse::<bool>() {
            return Cell::Boolean(parsed);
        }
        return Cell::String(cell.to_string());
    }

    pub fn value(&self) -> JsValue {
        match self {
            Cell::String(x) => JsValue::from_str(x),
            Cell::Number(x) => JsValue::from_f64(x.clone()),
            Cell::Boolean(x) => JsValue::from_bool(x.clone()),
        }
    }
}
