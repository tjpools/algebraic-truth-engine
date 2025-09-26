use num::{BigInt, BigRational, Zero, One, ToPrimitive, Signed, Integer};
use anyhow::{Result, anyhow};
use crate::types::{Expression, Value, Context, Statement};
use crate::group_theory::evaluate_group_operation;

pub struct AlgebraicEngine {
    pub context: Context,
}

impl AlgebraicEngine {
    pub fn new() -> Self {
        Self {
            context: Context::new(),
        }
    }
    
    pub fn execute_statement(&mut self, statement: Statement) -> Result<Option<Value>> {
        match statement {
            Statement::Expression(expr) => {
                let result = self.evaluate(&expr)?;
                Ok(Some(result))
            }
            Statement::Assignment { variable, value } => {
                let evaluated_value = self.evaluate(&value)?;
                self.context.set_variable(variable, evaluated_value);
                Ok(None)
            }
        }
    }
    
    pub fn evaluate(&self, expr: &Expression) -> Result<Value> {
        match expr {
            Expression::Value(val) => Ok(val.clone()),
            
            Expression::Variable(name) => {
                self.context.get_variable(name)
                    .cloned()
                    .ok_or_else(|| anyhow!("Undefined variable: {}", name))
            }
            
            Expression::Add(left, right) => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                self.add_values(&left_val, &right_val)
            }
            
            Expression::Subtract(left, right) => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                self.subtract_values(&left_val, &right_val)
            }
            
            Expression::Multiply(left, right) => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                self.multiply_values(&left_val, &right_val)
            }
            
            Expression::Divide(left, right) => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                self.divide_values(&left_val, &right_val)
            }
            
            Expression::Power(base, exp) => {
                let base_val = self.evaluate(base)?;
                let exp_val = self.evaluate(exp)?;
                self.power_values(&base_val, &exp_val)
            }
            
            Expression::Modulo(left, right) => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                self.modulo_values(&left_val, &right_val)
            }
            
            Expression::Negate(expr) => {
                let val = self.evaluate(expr)?;
                self.negate_value(&val)
            }
            
            Expression::Equal(left, right) => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                Ok(Value::Boolean(self.values_equal(&left_val, &right_val)))
            }
            
            Expression::NotEqual(left, right) => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                Ok(Value::Boolean(!self.values_equal(&left_val, &right_val)))
            }
            
            Expression::LessThan(left, right) => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                self.compare_values(&left_val, &right_val).map(|cmp| Value::Boolean(cmp < 0))
            }
            
            Expression::GreaterThan(left, right) => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                self.compare_values(&left_val, &right_val).map(|cmp| Value::Boolean(cmp > 0))
            }
            
            Expression::LessEqual(left, right) => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                self.compare_values(&left_val, &right_val).map(|cmp| Value::Boolean(cmp <= 0))
            }
            
            Expression::GreaterEqual(left, right) => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                self.compare_values(&left_val, &right_val).map(|cmp| Value::Boolean(cmp >= 0))
            }
            
            Expression::And(left, right) => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                self.logical_and(&left_val, &right_val)
            }
            
            Expression::Or(left, right) => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                self.logical_or(&left_val, &right_val)
            }
            
            Expression::Function { name, args } => {
                self.evaluate_function(name, args)
            }
        }
    }
    
    // Mathematical operations
    fn add_values(&self, left: &Value, right: &Value) -> Result<Value> {
        match (left, right) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
            (Value::Rational(a), Value::Rational(b)) => Ok(Value::Rational(a + b)),
            (Value::Complex(a), Value::Complex(b)) => Ok(Value::Complex(a + b)),
            
            // Type conversions
            (Value::Integer(a), Value::Rational(b)) => {
                Ok(Value::Rational(BigRational::from(a.clone()) + b))
            }
            (Value::Rational(a), Value::Integer(b)) => {
                Ok(Value::Rational(a + BigRational::from(b.clone())))
            }
            
            _ => Err(anyhow!("Cannot add {:?} and {:?}", left, right)),
        }
    }
    
    fn subtract_values(&self, left: &Value, right: &Value) -> Result<Value> {
        match (left, right) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
            (Value::Rational(a), Value::Rational(b)) => Ok(Value::Rational(a - b)),
            (Value::Complex(a), Value::Complex(b)) => Ok(Value::Complex(a - b)),
            
            // Type conversions
            (Value::Integer(a), Value::Rational(b)) => {
                Ok(Value::Rational(BigRational::from(a.clone()) - b))
            }
            (Value::Rational(a), Value::Integer(b)) => {
                Ok(Value::Rational(a - BigRational::from(b.clone())))
            }
            
            _ => Err(anyhow!("Cannot subtract {:?} from {:?}", right, left)),
        }
    }
    
    fn multiply_values(&self, left: &Value, right: &Value) -> Result<Value> {
        match (left, right) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a * b)),
            (Value::Rational(a), Value::Rational(b)) => Ok(Value::Rational(a * b)),
            (Value::Complex(a), Value::Complex(b)) => Ok(Value::Complex(a * b)),
            
            // Type conversions
            (Value::Integer(a), Value::Rational(b)) => {
                Ok(Value::Rational(BigRational::from(a.clone()) * b))
            }
            (Value::Rational(a), Value::Integer(b)) => {
                Ok(Value::Rational(a * BigRational::from(b.clone())))
            }
            
            _ => Err(anyhow!("Cannot multiply {:?} and {:?}", left, right)),
        }
    }
    
    fn divide_values(&self, left: &Value, right: &Value) -> Result<Value> {
        match (left, right) {
            (Value::Integer(a), Value::Integer(b)) => {
                if b.is_zero() {
                    Err(anyhow!("Division by zero"))
                } else {
                    Ok(Value::Rational(BigRational::new(a.clone(), b.clone())))
                }
            }
            (Value::Rational(a), Value::Rational(b)) => {
                if b.is_zero() {
                    Err(anyhow!("Division by zero"))
                } else {
                    Ok(Value::Rational(a / b))
                }
            }
            (Value::Complex(a), Value::Complex(b)) => {
                if b.re.is_zero() && b.im.is_zero() {
                    Err(anyhow!("Division by zero"))
                } else {
                    Ok(Value::Complex(a / b))
                }
            }
            
            // Type conversions
            (Value::Integer(a), Value::Rational(b)) => {
                if b.is_zero() {
                    Err(anyhow!("Division by zero"))
                } else {
                    Ok(Value::Rational(BigRational::from(a.clone()) / b))
                }
            }
            (Value::Rational(a), Value::Integer(b)) => {
                if b.is_zero() {
                    Err(anyhow!("Division by zero"))
                } else {
                    Ok(Value::Rational(a / BigRational::from(b.clone())))
                }
            }
            
            _ => Err(anyhow!("Cannot divide {:?} by {:?}", left, right)),
        }
    }
    
    fn power_values(&self, base: &Value, exp: &Value) -> Result<Value> {
        match (base, exp) {
            (Value::Integer(base), Value::Integer(exp)) => {
                if let Some(exp_i32) = exp.to_i32() {
                    if exp_i32 >= 0 {
                        Ok(Value::Integer(base.pow(exp_i32 as u32)))
                    } else {
                        // Negative exponent -> rational result
                        let positive_result = base.pow((-exp_i32) as u32);
                        Ok(Value::Rational(BigRational::new(BigInt::one(), positive_result)))
                    }
                } else {
                    Err(anyhow!("Exponent too large: {}", exp))
                }
            }
            _ => Err(anyhow!("Power operation not implemented for {:?} ^ {:?}", base, exp)),
        }
    }
    
    fn modulo_values(&self, left: &Value, right: &Value) -> Result<Value> {
        match (left, right) {
            (Value::Integer(a), Value::Integer(b)) => {
                if b.is_zero() {
                    Err(anyhow!("Modulo by zero"))
                } else {
                    Ok(Value::Integer(a % b))
                }
            }
            _ => Err(anyhow!("Modulo operation not supported for {:?} mod {:?}", left, right)),
        }
    }
    
    fn negate_value(&self, val: &Value) -> Result<Value> {
        match val {
            Value::Integer(n) => Ok(Value::Integer(-n)),
            Value::Rational(r) => Ok(Value::Rational(-r)),
            Value::Complex(c) => Ok(Value::Complex(-c)),
            _ => Err(anyhow!("Cannot negate {:?}", val)),
        }
    }
    
    fn values_equal(&self, left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Rational(a), Value::Rational(b)) => a == b,
            (Value::Complex(a), Value::Complex(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            
            // Cross-type comparisons
            (Value::Integer(a), Value::Rational(b)) => &BigRational::from(a.clone()) == b,
            (Value::Rational(a), Value::Integer(b)) => a == &BigRational::from(b.clone()),
            
            _ => false,
        }
    }
    
    fn compare_values(&self, left: &Value, right: &Value) -> Result<i32> {
        match (left, right) {
            (Value::Integer(a), Value::Integer(b)) => {
                Ok(if a < b { -1 } else if a > b { 1 } else { 0 })
            }
            (Value::Rational(a), Value::Rational(b)) => {
                Ok(if a < b { -1 } else if a > b { 1 } else { 0 })
            }
            
            // Cross-type comparisons
            (Value::Integer(a), Value::Rational(b)) => {
                let a_rat = BigRational::from(a.clone());
                Ok(if a_rat < *b { -1 } else if a_rat > *b { 1 } else { 0 })
            }
            (Value::Rational(a), Value::Integer(b)) => {
                let b_rat = BigRational::from(b.clone());
                Ok(if *a < b_rat { -1 } else if *a > b_rat { 1 } else { 0 })
            }
            
            _ => Err(anyhow!("Cannot compare {:?} and {:?}", left, right)),
        }
    }
    
    fn logical_and(&self, left: &Value, right: &Value) -> Result<Value> {
        match (left, right) {
            (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(*a && *b)),
            _ => Err(anyhow!("Logical AND requires boolean operands")),
        }
    }
    
    fn logical_or(&self, left: &Value, right: &Value) -> Result<Value> {
        match (left, right) {
            (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(*a || *b)),
            _ => Err(anyhow!("Logical OR requires boolean operands")),
        }
    }
    
    fn evaluate_function(&self, name: &str, args: &[Expression]) -> Result<Value> {
        let evaluated_args: Result<Vec<Value>> = args.iter()
            .map(|arg| self.evaluate(arg))
            .collect();
        let args = evaluated_args?;
        
        match name {
            "abs" => {
                if args.len() != 1 {
                    return Err(anyhow!("abs() takes exactly 1 argument"));
                }
                match &args[0] {
                    Value::Integer(n) => Ok(Value::Integer(n.abs())),
                    Value::Rational(r) => Ok(Value::Rational(r.abs())),
                    _ => Err(anyhow!("abs() requires numeric argument")),
                }
            }
            
            "gcd" => {
                if args.len() != 2 {
                    return Err(anyhow!("gcd() takes exactly 2 arguments"));
                }
                match (&args[0], &args[1]) {
                    (Value::Integer(a), Value::Integer(b)) => {
                        Ok(Value::Integer(a.gcd(b)))
                    }
                    _ => Err(anyhow!("gcd() requires integer arguments")),
                }
            }
            
            "sqrt" => {
                if args.len() != 1 {
                    return Err(anyhow!("sqrt() takes exactly 1 argument"));
                }
                match &args[0] {
                    Value::Integer(n) => {
                        if n.is_negative() {
                            Err(anyhow!("sqrt() of negative number"))
                        } else {
                            // For now, return a rational approximation
                            let n_f64 = n.to_f64().unwrap_or(0.0);
                            let sqrt_f64 = n_f64.sqrt();
                            let rational = BigRational::from_float(sqrt_f64)
                                .ok_or_else(|| anyhow!("Cannot represent sqrt as rational"))?;
                            Ok(Value::Rational(rational))
                        }
                    }
                    _ => Err(anyhow!("sqrt() requires numeric argument")),
                }
            }
            
            // Group theory operations - our mathematical consciousness extensions!
            "S3" | "cyclic" | "compose_s3" => {
                evaluate_group_operation(name, &args)
            }
            
            _ => Err(anyhow!("Unknown function: {}", name)),
        }
    }
}

impl Default for AlgebraicEngine {
    fn default() -> Self {
        Self::new()
    }
}