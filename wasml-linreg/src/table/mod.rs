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

        self.headers
            .iter()
            .enumerate()
            .for_each(|(idx, header)| headers.set(idx as u32, JsValue::from_str(header)));

        headers
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> js_sys::Map {
        let data = js_sys::Map::new();

        self.headers.iter().for_each(|header| {
            let column = self.data.get(header).unwrap_throw();
            let column_array = js_sys::Array::new_with_length(column.len() as u32);

            column.iter().enumerate().for_each(|(idx, cell)| {
                column_array.set(idx as u32, cell.value());
            });

            data.set(&JsValue::from_str(header), &column_array);
        });

        data
    }
}

#[wasm_bindgen(js_name = tableFromCSV)]
pub async fn table_from_csv(file: web_sys::File) -> Result<Table, JsValue> {
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
        row.iter().enumerate().for_each(|(idx, cell)| {
            let value = Cell::new(cell);
            let header = &headers[idx];
            if data.contains_key(header) {
                data.get_mut(header).unwrap_throw().push(value);
            } else {
                data.insert(header.to_string(), vec![value]);
            }
        });
    }

    Ok(Table { data, headers })
}
