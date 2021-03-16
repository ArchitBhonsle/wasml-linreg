pub mod cell;

use std::collections::HashMap;
use wasm_bindgen::prelude::*;

use cell::Cell;

#[wasm_bindgen]
pub struct Table {
    #[wasm_bindgen(skip)]
    pub data: HashMap<String, Vec<Cell>>,

    #[wasm_bindgen(skip)]
    pub headers: Vec<String>,
}

#[wasm_bindgen]
impl Table {
    #[wasm_bindgen(getter)]
    pub fn headers(&self) -> js_sys::Array {
        let headers = js_sys::Array::new_with_length(self.headers.len() as u32);

        for (idx, header) in self.headers.iter().enumerate() {
            headers.set(idx as u32, JsValue::from_str(header))
        }

        headers
    }
}

#[wasm_bindgen(js_name = newTable)]
pub async fn new_table(file: web_sys::File) -> Result<Table, JsValue> {
    let text_jsvalue = wasm_bindgen_futures::JsFuture::from(file.text())
        .await
        .unwrap_throw();
    let text = text_jsvalue.as_string().unwrap_throw();

    let mut reader = csv::Reader::from_reader(text.as_bytes());

    let headers: Vec<String> = reader
        .headers()
        .unwrap_throw()
        .clone()
        .into_iter()
        .map(|x| x.to_string())
        .collect();

    let mut data: HashMap<String, Vec<Cell>> = HashMap::new();

    for row_result in reader.records() {
        let row = row_result.unwrap_throw();
        for (idx, cell) in row.iter().enumerate() {
            let value = Cell::new(cell);
            let header = &headers[idx];
            if data.contains_key(header) {
                data.get_mut(header).unwrap_throw().push(value);
            } else {
                data.insert(header.to_string(), vec![value]);
            }
        }
    }

    Ok(Table { data, headers })
}
