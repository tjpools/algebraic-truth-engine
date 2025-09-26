use num::{BigInt, BigRational, Complex, Zero, One};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Represents different types of mathematical values
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Integer(BigInt),
    Rational(BigRational),
    Complex(Complex<BigRational>),
    Boolean(bool),
    Matrix(Vec<Vec<Value>>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Integer(n) => write!(f, "{}", n),
            Value::Rational(r) => {
                if r.denom() == &BigInt::one() {
                    write!(f, "{}", r.numer())
                } else {
                    write!(f, "{}/{}", r.numer(), r.denom())
                }
            }
            Value::Complex(c) => {
                if c.im.is_zero() {
                    write!(f, "{}", c.re)
                } else if c.re.is_zero() {
                    write!(f, "{}i", c.im)
                } else {
                    write!(f, "{}+{}i", c.re, c.im)
                }
            }
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Matrix(m) => {
                write!(f, "[")?;
                for (i, row) in m.iter().enumerate() {
                    if i > 0 { write!(f, "; ")?; }
                    write!(f, "[")?;
                    for (j, val) in row.iter().enumerate() {
                        if j > 0 { write!(f, ", ")?; }
                        write!(f, "{}", val)?;
                    }
                    write!(f, "]")?;
                }
                write!(f, "]")
            }
        }
    }
}

/// Algebraic expressions - the heart of our symbolic system
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    Value(Value),
    Variable(String),
    
    // Arithmetic operations
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Power(Box<Expression>, Box<Expression>),
    Modulo(Box<Expression>, Box<Expression>),
    
    // Unary operations
    Negate(Box<Expression>),
    
    // Comparison operations
    Equal(Box<Expression>, Box<Expression>),
    NotEqual(Box<Expression>, Box<Expression>),
    LessThan(Box<Expression>, Box<Expression>),
    GreaterThan(Box<Expression>, Box<Expression>),
    LessEqual(Box<Expression>, Box<Expression>),
    GreaterEqual(Box<Expression>, Box<Expression>),
    
    // Logical operations
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    
    // Function calls
    Function {
        name: String,
        args: Vec<Expression>,
    },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Value(v) => write!(f, "{}", v),
            Expression::Variable(name) => write!(f, "{}", name),
            Expression::Add(left, right) => write!(f, "({} + {})", left, right),
            Expression::Subtract(left, right) => write!(f, "({} - {})", left, right),
            Expression::Multiply(left, right) => write!(f, "({} * {})", left, right),
            Expression::Divide(left, right) => write!(f, "({} / {})", left, right),
            Expression::Power(base, exp) => write!(f, "({} ^ {})", base, exp),
            Expression::Modulo(left, right) => write!(f, "({} mod {})", left, right),
            Expression::Negate(expr) => write!(f, "(-{})", expr),
            Expression::Equal(left, right) => write!(f, "({} = {})", left, right),
            Expression::NotEqual(left, right) => write!(f, "({} != {})", left, right),
            Expression::LessThan(left, right) => write!(f, "({} < {})", left, right),
            Expression::GreaterThan(left, right) => write!(f, "({} > {})", left, right),
            Expression::LessEqual(left, right) => write!(f, "({} <= {})", left, right),
            Expression::GreaterEqual(left, right) => write!(f, "({} >= {})", left, right),
            Expression::And(left, right) => write!(f, "({} && {})", left, right),
            Expression::Or(left, right) => write!(f, "({} || {})", left, right),
            Expression::Function { name, args } => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
        }
    }
}

/// Context for variable assignments and function definitions
#[derive(Clone, Debug, Default)]
pub struct Context {
    pub variables: HashMap<String, Value>,
    pub constants: HashMap<String, Value>,
}

impl Context {
    pub fn new() -> Self {
        let mut ctx = Context {
            variables: HashMap::new(),
            constants: HashMap::new(),
        };
        
        // Add mathematical constants
        ctx.constants.insert("pi".to_string(), 
            Value::Rational(BigRational::from_float(std::f64::consts::PI).unwrap()));
        ctx.constants.insert("e".to_string(), 
            Value::Rational(BigRational::from_float(std::f64::consts::E).unwrap()));
        ctx.constants.insert("phi".to_string(), 
            Value::Rational(BigRational::new(BigInt::from(1618034), BigInt::from(1000000))));
        
        ctx
    }
    
    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name).or_else(|| self.constants.get(name))
    }
    
    pub fn set_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }
}

/// Represents a statement in our algebraic language
#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Expression(Expression),
    Assignment { variable: String, value: Expression },
}