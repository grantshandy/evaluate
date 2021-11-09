extern crate wasm_bindgen;
extern crate xxcalc;

use wasm_bindgen::prelude::*;

use xxcalc::linear_solver::LinearSolver;
use xxcalc::calculator::Calculator;

#[wasm_bindgen]
pub fn eval_expression(exp: String) -> String {
    match LinearSolver.process(exp.as_str()) {
        Ok(answer) => format!("Answer: {}", answer),
        Err(error) => format!("Error: {:?}", error),
    }
}