use na::{DMatrix, Matrix, U1, Dynamic, VecStorage};
use wasm_bindgen::prelude::*;

use crate::watrix::Watrix;
use crate::utils;

#[wasm_bindgen]
pub struct LinearRegression {
    #[wasm_bindgen(skip)]
    pub learning_rate: f64,

    #[wasm_bindgen(skip)]
    pub epochs: u32,

    #[wasm_bindgen(skip)]
    pub theta: Option<DMatrix<f64>>
}

#[wasm_bindgen]
impl LinearRegression {
    #[wasm_bindgen(constructor)]
    pub fn new(learning_rate: f64, epochs: u32) -> LinearRegression {
        LinearRegression {
            learning_rate,
            epochs,
            theta: None
        }
    }

    pub fn fit(&mut self, x_train: &Watrix, y_train: &Watrix) {
        // input.ncols() is number of features, input.nrows() is number of examples
        let x = x_train.data.clone().insert_column(0, 1.0);
        let y = y_train.data.clone();
        
        let mut theta = DMatrix::from_element(1, x_train.ncols()+1, 0.5);
        
        for iter in 1..(self.epochs+1) {
            if iter % 10 == 0 {
                utils::log(&format!("Iteration {}: {}", iter, calculate_cost(&x, &y, &theta)));
            }
            let derivative = calculate_derivative(&x, &y, &theta);
            theta -= self.learning_rate * derivative;
        }

        self.theta = Some(theta)
    }

    pub fn predict(&self, x_test: &Watrix, y_test: &Watrix) -> Result<js_sys::Map, JsValue> {
        if let Some(theta) = self.theta.clone() {
            let map = js_sys::Map::new();
            
            let x = x_test.data.clone().insert_column(0, 1.0);
            let y = y_test.data.clone();
            
            let hypo = &x * theta.transpose();
            let hypo_watrix = Watrix { data: hypo.transpose() };
            map.set(&JsValue::from_str("predictions"), &hypo_watrix.data());

            // Calculate r2 score
            let y = y.column(0);
            let f = hypo.column(0);
            
            let y_mean = y.mean();
            let ss_tot = y.map(|x| (x - y_mean).powi(2)).sum();
            let ss_res = (&y - &f).map(|x| x.powi(2)).sum();
            
            let r2_score = 1.0 - (ss_res / ss_tot);
            map.set(&JsValue::from_str("r2_score"), &JsValue::from_f64(r2_score));
            map.set(&JsValue::from_str("ss_tot"), &JsValue::from_f64(ss_tot));
            map.set(&JsValue::from_str("ss_res"), &JsValue::from_f64(ss_res));

            // Mean Square Error
            let mse = ((&y - &f).map(|x| x.powi(2)).sum()) * (1.0 / y.nrows() as f64);
            map.set(&JsValue::from_str("mse"), &JsValue::from_f64(mse));
            

            return Ok(map);
        }
        
        Err(JsValue::from_str("Model not trained yet"))
    }
}

fn calculate_derivative(x: &DMatrix<f64>, y: &DMatrix<f64>, theta: &DMatrix<f64>) -> Matrix<f64, U1, Dynamic, VecStorage<f64, U1, Dynamic>> {
    let diff = (x * theta.transpose()) - y;
    
    let mut grad = x.clone();
    for (i, mut row) in grad.row_iter_mut().enumerate() {
        row *= diff[(i, 0)];
    }

    grad.row_mean()
}

fn calculate_cost(x: &DMatrix<f64>, y: &DMatrix<f64>, theta: &DMatrix<f64>) -> f64 {
    let diff = (x * theta.transpose()) - y;
    (diff.map(|x| x.powi(2)).column_mean() * (0.5))[(0, 0)]
}
