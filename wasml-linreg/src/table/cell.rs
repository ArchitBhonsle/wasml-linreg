use wasm_bindgen::prelude::*;

#[derive(PartialEq, PartialOrd, Clone)]
pub enum Cell {
    String(String),
    Number(f64),
}

impl Cell {
    pub fn new(cell: &str) -> Cell {
        if let Ok(parsed) = cell.parse::<f64>() {
            return Cell::Number(parsed);
        }
        return Cell::String(cell.to_string());
    }
   
    pub fn new_from_string(cell: String) -> Cell {
        return Cell::String(cell);
    }
    
    pub fn new_from_number(cell: f64) -> Cell {
        return Cell::Number(cell);
    }

    pub fn is_number(&self) -> bool {
        match self {
            Cell::Number(_) => true,
            Cell::String(_) => false,
        }
    }

    pub fn to_js(&self) -> JsValue {
        match self {
            Cell::String(x) => JsValue::from_str(x),
            Cell::Number(x) => JsValue::from_f64(*x),
        }
    }
    
    pub fn from_js(value: JsValue) -> Result<Cell, JsValue> {
        if let Some(val) = value.as_string() {
            return Ok(Cell::String(val));
        }
        
        if let Some(val) = value.as_f64() {
            return Ok(Cell::Number(val));
        }

        return Err(JsValue::from_str("Unsupported datatype"))
    }
}
