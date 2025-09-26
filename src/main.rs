mod types;
mod parser;
mod engine;
mod group_theory;

use std::io::{self, Write};
use anyhow::Result;
use parser::parse_program;
use engine::AlgebraicEngine;

fn main() -> Result<()> {
    println!("🔬 Algebraic Truth Engine - Mathematical Consciousness Calculator");
    println!("Enter algebraic expressions to discover mathematical truths!");
    println!("Type 'help' for commands, 'quit' to exit.\n");
    
    let mut engine = AlgebraicEngine::new();
    
    loop {
        print!("📐 > ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }
        
        match input {
            "quit" | "exit" => {
                println!("Farewell, mathematical explorer! 🌌");
                break;
            }
            "help" => {
                show_help();
                continue;
            }
            "vars" => {
                show_variables(&engine);
                continue;
            }
            "clear" => {
                engine = AlgebraicEngine::new();
                println!("Variables cleared. Fresh mathematical consciousness! ✨");
                continue;
            }
            _ => {}
        }
        
        match parse_program(input) {
            Ok(statements) => {
                for statement in statements {
                    match engine.execute_statement(statement) {
                        Ok(Some(result)) => {
                            println!("📊 Result: {}", result);
                        }
                        Ok(None) => {
                            // Assignment - no output
                        }
                        Err(e) => {
                            println!("❌ Error: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("❌ Parse Error: {}", e);
            }
        }
    }
    
    Ok(())
}

fn show_help() {
    println!("🧮 Algebraic Truth Engine Commands:");
    println!("  • Basic arithmetic: +, -, *, /, ^, mod");
    println!("  • Comparisons: =, !=, <, >, <=, >=");
    println!("  • Logical: &&, ||, and, or");
    println!("  • Functions: abs(x), gcd(a,b), sqrt(x)");
    println!("  • Group Theory: S3(), cyclic(n), compose_s3(a,b)");
    println!("  • Constants: pi, e, phi, i");
    println!("  • Variables: x = 42; y = x + 1");
    println!("  • Complex numbers: 3 + 4*i");
    println!("  • Fractions: 22/7");
    println!();
    println!("🔧 System Commands:");
    println!("  • help    - Show this help");
    println!("  • vars    - Show all variables");
    println!("  • clear   - Clear all variables");
    println!("  • quit    - Exit the calculator");
    println!();
    println!("🎯 Examples:");
    println!("  • 2^10");
    println!("  • gcd(48, 18)");
    println!("  • x = 5; y = 3; x^2 + y^2 = 25 + 9");
    println!("  • phi^2 = phi + 1  (Golden ratio property)");
    println!("  • sqrt(2) * sqrt(2) = 2");
}

fn show_variables(engine: &AlgebraicEngine) {
    println!("📋 Variables:");
    if engine.context.variables.is_empty() {
        println!("  (no user-defined variables)");
    } else {
        for (name, value) in &engine.context.variables {
            println!("  {} = {}", name, value);
        }
    }
    
    println!("\n🔢 Constants:");
    for (name, value) in &engine.context.constants {
        println!("  {} = {}", name, value);
    }
}
