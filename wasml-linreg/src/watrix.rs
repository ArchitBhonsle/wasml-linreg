use ndarray::{Array2, ArrayBase, s};
use wasm_bindgen::prelude::*;

use crate::table::table::*;

#[wasm_bindgen]
pub struct Watrix {
    #[wasm_bindgen(skip)]
    pub data: Array2<f64>,
}

#[wasm_bindgen]
impl Watrix {
    #[wasm_bindgen(constructor)]
    pub fn new_from_table(table: &Table) -> Watrix {
        let (height, width) = table.dims().unwrap();
        let array = 
            Array2::from_shape_fn(
                (height, width),
                |(i, j)| table.index(i, j).unwrap());

        Watrix {
            data: array
        }
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> usize {
        self.data.nrows()
    }
    
    #[wasm_bindgen(getter)]
    pub fn width(&self) -> usize {
        self.data.ncols()
    }

    #[wasm_bindgen(getter, js_name = data)]
    pub fn data(&self) -> js_sys::Array {
        let (height, width) = (self.height() as u32, self.width() as u32);
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
    
    // pub fn splice(&mut self, fraction: f64) -> Result<Watrix, JsValue> {
    //     if fraction <= 1.0 {
    //         return Err(JsValue::from_str("fraction has to be less than or equal to 1"));
    //     }
        
    //     let (height, width) = (self.height(), self.width());
        
    //     let first_len = (height as f64 * fraction) as usize;
    //     let (mut first_slice, mut second_slice) = self.data
    //         .multi_slice_mut((s![..first_len,..], s![first_len.., ..]));


    //     let mut first = ArrayBase::from_shape((first_len, width), &mut first_slice).unwrap();
    //     let mut second = ArrayBase::from_shape((height - first_len, width), &mut second_slice).unwrap();
        
    //     Ok(Watrix {
    //         data: second
    //     })
    // }
}