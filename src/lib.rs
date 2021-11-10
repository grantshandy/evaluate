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

    let polynomial = match LinearSolver.process(exp.as_str()) {
        Ok(polynomial) => polynomial,
        Err(first_error) => {
            match PolynomialCalculator.process(exp.as_str()) {
                Ok(polynomial) => polynomial,
                Err(second_error) => return format!("Linear Solver Error: {:?}, Polynomial Solver Error: {:?}", first_error, second_error),
            }
        }
    };

    let num = match polynomial.as_f64() {
        Ok(num) => num,
        Err(error) => return format!("Number Formatting Error {:?}", error),
    };

    return num.to_string();
}