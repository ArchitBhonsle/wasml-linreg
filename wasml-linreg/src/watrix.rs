use na::DMatrix;
use wasm_bindgen::prelude::*;
use rand::Rng;

use crate::table::table::*;

#[wasm_bindgen]
pub struct Watrix {
    #[wasm_bindgen(skip)]
    pub data: DMatrix<f64>,
}

#[wasm_bindgen]
impl Watrix {
    #[wasm_bindgen(js_name = newFromTable)]
    pub fn new_from_table(table: &Table) -> Watrix {
        let (height, width) = table.dims().unwrap();
        let data = 
            DMatrix::from_fn(    
                height,
                width,
                |i, j| table.index(i, j).unwrap());

        Watrix {
            data
        }
    }

    #[wasm_bindgen(getter)]
    pub fn nrows(&self) -> usize {
        self.data.nrows()
    }
    
    #[wasm_bindgen(getter)]
    pub fn ncols(&self) -> usize {
        self.data.ncols()
    }

    #[wasm_bindgen(getter, js_name = dims)]
    pub fn dims_to_js(&self) -> js_sys::Array {
        let array = js_sys::Array::new_with_length(2);
        array.set(0, JsValue::from(self.nrows() as u32));
        array.set(1, JsValue::from(self.ncols() as u32));
        
        array
    }   
    

    #[wasm_bindgen(getter, js_name = data)]
    pub fn data(&self) -> js_sys::Array {
        let (height, width) = (self.nrows() as u32, self.ncols() as u32);
        let array = js_sys::Array::new_with_length(height);

        for i in 0..height {
            let subarray = js_sys::Array::new_with_length(width);
            for j in 0..width {
                subarray.set(j, JsValue::from_f64(self.data[(i as usize, j as usize)]));
            }
            array.set(i, subarray.into());
        }

        array
    }
    
    pub fn shuffle(&self) -> Watrix {
        let mut shuffled_watrix = Watrix { data: self.data.clone() };
        shuffled_watrix.shuffle_mut();

        shuffled_watrix
    }

    pub fn shuffle_mut(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 1..self.nrows() {
            let j: usize = rng.gen_range(0..i);
            self.data.swap_rows(i, j);
        }
    }

    pub fn row_slice(&self, start: usize, end: usize) -> Watrix {
        Watrix {
            data: self.data.rows_range(start..end).clone_owned()
        }
    }

    pub fn col_slice(&self, start: usize, end: usize) -> Watrix {
        Watrix {
            data: self.data.columns_range(start..end).clone_owned()
        }
    }
    
}

// impl Watrix {
//     pub fn log(&self, name: &str) {
//         utils::log(&format!("{}: ({}, {})", name, self.nrows(), self.ncols()));
//     }
// }