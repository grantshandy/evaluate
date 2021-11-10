extern crate wasm_bindgen;
extern crate xxcalc;

use wasm_bindgen::prelude::*;

use xxcalc::linear_solver::LinearSolver;
use xxcalc::polynomial_calculator::PolynomialCalculator;
use xxcalc::calculator::Calculator;

#[wasm_bindgen]
pub fn eval_expression(exp: String) -> String {
    if exp.len() == 0 {
        return String::new();
    }

    match LinearSolver.process(exp.as_str()) {
        Ok(answer) => format!("Answer: {}", answer),
        Err(first_error) => {
            match PolynomialCalculator.process(exp.as_str()) {
                Ok(answer) => format!("Answer: {}", answer),
                Err(second_error) => format!("Linear Solver Error: {:?}, Polynomial Solver Error: {:?}", first_error, second_error),
            }
        }
    }
}

#[wasm_bindgen]
pub fn rust_says_hi() -> String {
    String::from("Rust Says Hi!")
}