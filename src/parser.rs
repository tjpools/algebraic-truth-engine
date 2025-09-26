use pest::Parser;
use pest_derive::Parser;
use num::{BigInt, BigRational, Complex, Zero, One};
use anyhow::{Result, anyhow};
use crate::types::{Expression, Value, Statement};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct AlgebraicParser;

pub fn parse_program(input: &str) -> Result<Vec<Statement>> {
    let pairs = AlgebraicParser::parse(Rule::program, input)
        .map_err(|e| anyhow!("Parse error: {}", e))?;
    
    let mut statements = Vec::new();
    
    for pair in pairs {
        match pair.as_rule() {
            Rule::program => {
                for inner_pair in pair.into_inner() {
                    match inner_pair.as_rule() {
                        Rule::statement => {
                            statements.push(parse_statement(inner_pair)?);
                        }
                        Rule::EOI => break,
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    
    Ok(statements)
}

fn parse_statement(pair: pest::iterators::Pair<Rule>) -> Result<Statement> {
    let inner = pair.into_inner().next().unwrap();
    
    match inner.as_rule() {
        Rule::assignment => parse_assignment(inner),
        Rule::expression => Ok(Statement::Expression(parse_expression(inner)?)),
        _ => Err(anyhow!("Unexpected statement type")),
    }
}

fn parse_assignment(pair: pest::iterators::Pair<Rule>) -> Result<Statement> {
    let mut inner = pair.into_inner();
    let var_name = inner.next().unwrap().as_str().to_string();
    let expr = parse_expression(inner.next().unwrap())?;
    
    Ok(Statement::Assignment {
        variable: var_name,
        value: expr,
    })
}

fn parse_expression(pair: pest::iterators::Pair<Rule>) -> Result<Expression> {
    match pair.as_rule() {
        Rule::expression | Rule::logical_or | Rule::logical_and | 
        Rule::comparison | Rule::sum | Rule::term => {
            let mut inner = pair.into_inner();
            let mut left = parse_expression(inner.next().unwrap())?;
            
            while let Some(op_pair) = inner.next() {
                let right = parse_expression(inner.next().unwrap())?;
                
                left = match op_pair.as_str() {
                    "+" => Expression::Add(Box::new(left), Box::new(right)),
                    "-" => Expression::Subtract(Box::new(left), Box::new(right)),
                    "*" => Expression::Multiply(Box::new(left), Box::new(right)),
                    "/" => Expression::Divide(Box::new(left), Box::new(right)),
                    "mod" => Expression::Modulo(Box::new(left), Box::new(right)),
                    "=" => Expression::Equal(Box::new(left), Box::new(right)),
                    "!=" => Expression::NotEqual(Box::new(left), Box::new(right)),
                    "<" => Expression::LessThan(Box::new(left), Box::new(right)),
                    ">" => Expression::GreaterThan(Box::new(left), Box::new(right)),
                    "<=" => Expression::LessEqual(Box::new(left), Box::new(right)),
                    ">=" => Expression::GreaterEqual(Box::new(left), Box::new(right)),
                    "&&" | "and" => Expression::And(Box::new(left), Box::new(right)),
                    "||" | "or" => Expression::Or(Box::new(left), Box::new(right)),
                    _ => return Err(anyhow!("Unknown operator: {}", op_pair.as_str())),
                };
            }
            
            Ok(left)
        }
        
        Rule::power => {
            let mut inner = pair.into_inner();
            let mut left = parse_expression(inner.next().unwrap())?;
            
            while let Some(right_pair) = inner.next() {
                let right = parse_expression(right_pair)?;
                left = Expression::Power(Box::new(left), Box::new(right));
            }
            
            Ok(left)
        }
        
        Rule::unary => {
            let mut inner = pair.into_inner();
            let first = inner.next().unwrap();
            
            if first.as_str() == "-" {
                let expr = parse_expression(inner.next().unwrap())?;
                Ok(Expression::Negate(Box::new(expr)))
            } else if first.as_str() == "+" {
                parse_expression(inner.next().unwrap())
            } else {
                parse_expression(first)
            }
        }
        
        Rule::primary => {
            let inner = pair.into_inner().next().unwrap();
            match inner.as_rule() {
                Rule::expression => parse_expression(inner),
                Rule::function_call => parse_function_call(inner),
                Rule::constant => parse_constant(inner),
                Rule::number => parse_number(inner),
                Rule::variable => Ok(Expression::Variable(inner.as_str().to_string())),
                _ => Err(anyhow!("Unexpected primary expression")),
            }
        }
        
        Rule::function_call => parse_function_call(pair),
        Rule::number => parse_number(pair),
        Rule::variable => Ok(Expression::Variable(pair.as_str().to_string())),
        Rule::constant => parse_constant(pair),
        
        _ => Err(anyhow!("Unexpected rule: {:?}", pair.as_rule())),
    }
}

fn parse_function_call(pair: pest::iterators::Pair<Rule>) -> Result<Expression> {
    let mut inner = pair.into_inner();
    let function_name = inner.next().unwrap().as_str().to_string();
    
    let mut args = Vec::new();
    for arg_pair in inner {
        args.push(parse_expression(arg_pair)?);
    }
    
    Ok(Expression::Function {
        name: function_name,
        args,
    })
}

fn parse_number(pair: pest::iterators::Pair<Rule>) -> Result<Expression> {
    let inner = pair.into_inner().next().unwrap();
    
    match inner.as_rule() {
        Rule::integer => {
            let int_str = inner.as_str();
            let big_int = int_str.parse::<BigInt>()
                .map_err(|_| anyhow!("Invalid integer: {}", int_str))?;
            Ok(Expression::Value(Value::Integer(big_int)))
        }
        Rule::float => {
            let float_str = inner.as_str();
            let float_val: f64 = float_str.parse()
                .map_err(|_| anyhow!("Invalid float: {}", float_str))?;
            let rational = BigRational::from_float(float_val)
                .ok_or_else(|| anyhow!("Cannot convert float to rational: {}", float_val))?;
            Ok(Expression::Value(Value::Rational(rational)))
        }
        _ => Err(anyhow!("Unknown number type")),
    }
}

fn parse_constant(pair: pest::iterators::Pair<Rule>) -> Result<Expression> {
    match pair.as_str() {
        "pi" => Ok(Expression::Variable("pi".to_string())),
        "e" => Ok(Expression::Variable("e".to_string())),
        "i" => Ok(Expression::Value(Value::Complex(
            Complex::new(BigRational::zero(), BigRational::one())
        ))),
        "phi" => Ok(Expression::Variable("phi".to_string())),
        _ => Err(anyhow!("Unknown constant: {}", pair.as_str())),
    }
}