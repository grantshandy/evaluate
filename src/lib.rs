extern crate wasm_bindgen;
extern crate xxcalc;

use wasm_bindgen::prelude::*;

use xxcalc::linear_solver::LinearSolver;
use xxcalc::polynomial;
use xxcalc::polynomial_calculator::PolynomialCalculator;
use xxcalc::calculator::Calculator;

#[wasm_bindgen]
pub fn eval_expression(exp: String) -> String {
    if exp.len() == 0 {
        return String::new();
    }

    let answer = match LinearSolver.process(exp.as_str()) {
        Ok(polynomial) => match polynomial.as_f64() {
            Ok(answer) => answer,
            Err(_) => return polynomial.to_string(),
        },
        Err(first_error) => {
            match PolynomialCalculator.process(exp.as_str()) {
                Ok(polynomial) => match polynomial.as_f64() {
                    Ok(answer) => answer,
                    Err(_) => return polynomial.to_string(),
                },
                Err(second_error) => return format!("Linear Solver Error: {:?}, Polynomial Solver Error: {:?}", first_error, second_error),
            }
        }
    };

    return answer.to_string();
}