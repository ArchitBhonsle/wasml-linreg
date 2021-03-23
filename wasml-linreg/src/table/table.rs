use std::collections::HashMap;
use wasm_bindgen::prelude::*;

use super::cell::*;
use super::column::*;
use super::transform::*;

#[wasm_bindgen]
pub struct Table {
    #[wasm_bindgen(skip)]
    pub data: HashMap<String, Column>,

    #[wasm_bindgen(skip)]
    pub headers: Vec<String>,
}

#[wasm_bindgen]
impl Table {
    #[wasm_bindgen(getter, js_name = headers)]
    pub fn headers_to_js(&self) -> js_sys::Array {
        let headers = js_sys::Array::new_with_length(self.headers.len() as u32);

        self.headers
            .iter()
            .enumerate()
            .for_each(|(idx, header)| headers.set(idx as u32, JsValue::from_str(header)));

        headers
    }

    #[wasm_bindgen(getter, js_name = data)]
    pub fn to_js(&self) -> js_sys::Map {
        let data = js_sys::Map::new();

        self.headers.iter().for_each(|header| {
            let column = self.data.get(header).unwrap_throw();
            let column_array = column.data_to_js();

            data.set(&JsValue::from_str(header), &column_array);
        });

        data
    }

    pub fn pop(&mut self, header: String) -> Option<Column> {
        let removed = self.data.remove_entry(&header);

        match removed {
            Some((header, column)) => {
                self.headers.retain(|x| *x != header);
                Some(column)
            }
            None => None,
        }
    }
    
    #[wasm_bindgen(js_name = pushColumn)]
    pub fn push_column(&mut self, column: Column) -> Result<(), JsValue> {
        if self.headers.contains(&column.header) {
            return Err(JsValue::from_str("header already exists"));
        }
        
        self.headers.push(column.header.clone());
        self.data.insert(column.header.clone(), column);        
        
        Ok(())
    }

    #[wasm_bindgen(js_name = isNumber)]
    pub fn is_number(&self) -> bool {
        self.data.values().into_iter().all(|e| e.is_number())
    }

    #[wasm_bindgen(js_name = isUniform)]
    pub fn is_uniform(&self) -> bool {
        self.data.values().into_iter().all(|e| e.is_uniform())
    }

    #[wasm_bindgen(js_name = applyTransform)]
    pub fn apply_transform(&self, trans: &Transform) -> Result<Table, JsValue> {
        if !self.is_uniform() {
            return Err(JsValue::from_str("Table has to be uniform"));
        }

        let mut headers: Vec<String> = Vec::new();
        let mut data: HashMap<String, Column> = HashMap::new();
        self.data.iter().for_each(|(header, column)| {
            if column.is_number() {
                let normalizer = trans.normalize.get(header).unwrap_throw().clone();
                let normalized_data: Vec<Cell> = column.data
                    .iter()
                    .map(|c| {
                        if let Cell::Number(v) = c {
                            return Cell::new_from_number(normalizer(*v));
                        } else {
                            return Cell::new_from_number(0.0);
                        }
                    })
                    .collect();
                
                headers.push(header.clone());
                data.insert(header.clone(), Column {
                    header: header.clone(),
                    data: normalized_data,
                });
            } else {
                let encoder = trans.encoding.get(header).unwrap_throw();
                let encoded_data: Vec<Cell> = column.data
                    .iter()
                    .map(|c| {
                        if let Cell::String(v) = c {
                            return Cell::new_from_number(encoder(v.clone()));
                        } else {
                            return Cell::new_from_number(0.0);
                        }
                    })
                    .collect();
                
                headers.push(header.clone());
                data.insert(header.clone(), Column {
                    header: header.clone(),
                    data: encoded_data,
                });
            }
        });

        Ok(Table {
            headers,
            data,
        })
    }
}

#[wasm_bindgen(js_name = readCSV)]
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

    let mut data_as_strings: HashMap<String, Vec<String>> = HashMap::new();
    reader.records().for_each(|row_result| {
        let row = row_result.unwrap_throw();
        
        row.iter().enumerate().for_each(|(idx, cell_str)| {
            let header = &headers[idx];
            let cell = cell_str.clone();
            if data_as_strings.contains_key(header) {
                data_as_strings.get_mut(header).unwrap_throw().push(cell.to_string());
            } else {
                data_as_strings.insert(header.to_string(), vec![cell.to_string()]);
            }
        });
    });
    
    let mut data: HashMap<String, Column> = HashMap::new();
    data_as_strings.iter().for_each(|(header, data_vec)| {
        let column = Column::new(header.clone(), data_vec.clone()).unwrap_throw();
        data.insert(header.clone(), column);
    });

    Ok(Table { data, headers })
}
