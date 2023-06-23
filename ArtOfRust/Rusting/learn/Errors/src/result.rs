mod checked {
    // Mathematical "errors" we want to catch
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NonPositionLogarithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            // THis operation would `fail`, instead let's return the reasion of 
            // the failure wrapped in `Err`.
            Err(MathError::DivisionByZero)
        } else {
            // This operation is valid, return the result wrapped in `Ok`
            Ok(x/y)
        }
    }

    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0  {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    pub fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositionLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    // Intermediate function
    //  for result operator, example
    pub fn _op(x: f64, y: f64) -> MathResult {
        // if `div` "fails", then `DivisionByZero` wil be `return`ed
        let ratio = div(x, y)?;

        // if `ln` "fails", then `NonPositiveLogarithm` will be `return`ed
        let ln = ln(ratio)?;
        sqrt(ln)
    }

    pub fn using_result_operator(x:f64, y:f64) {
        match  _op(x, y) {
            Err(why) => panic!(
                "{}", match  why {
                    MathError::NonPositionLogarithm => "Logarithm of non-positive number",
                    MathError::DivisionByZero => "division by zero",
                    MathError::NegativeSquareRoot => "square root of negative number",
                }
            ),
            Ok(value) => println!("{}", value),
        }
    }
    
}

// `op(x, y)` === `sqrt(ln(x/y))`
fn op(x: f64, y: f64) -> f64 {
    // This is a three level match pyramid
    match checked::div(x, y) {
        Err(why) => panic!("{:?}", why),
        Ok(ratio) => match checked::ln(ratio) {
            Err(why) => panic!("{:?}", why),
            Ok(ln) => match checked::sqrt(ln) {
                Err(why) => panic!("{:?}", why),
                Ok(sqrt) => sqrt
            }
        }
    }
}

pub fn Main() {
    // println!("{}", op(100.0, 10.0));
    checked::using_result_operator(100.0, 10.0);
}