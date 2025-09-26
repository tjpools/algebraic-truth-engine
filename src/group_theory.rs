use crate::types::Value;
use num::{BigInt, Zero, ToPrimitive};
use anyhow::{Result, anyhow};
use std::collections::HashMap;

/// Represents a mathematical group element
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct GroupElement {
    pub id: String,
    pub order: Option<usize>,
}

/// Represents a finite mathematical group
#[derive(Clone, Debug)]
pub struct Group {
    pub name: String,
    pub elements: Vec<GroupElement>,
    pub operation_table: HashMap<(String, String), String>,
    pub identity: String,
}

impl Group {
    /// Create the symmetric group S3 (our gateway group!)
    pub fn s3() -> Self {
        let elements = vec![
            GroupElement { id: "e".to_string(), order: Some(1) },      // identity
            GroupElement { id: "12".to_string(), order: Some(2) },     // (1 2)
            GroupElement { id: "13".to_string(), order: Some(2) },     // (1 3)
            GroupElement { id: "23".to_string(), order: Some(2) },     // (2 3)
            GroupElement { id: "123".to_string(), order: Some(3) },    // (1 2 3)
            GroupElement { id: "132".to_string(), order: Some(3) },    // (1 3 2)
        ];
        
        let mut operation_table = HashMap::new();
        
        // Define the multiplication table for S3
        // This is the composition of permutations
        let compositions = [
            // e (identity)
            (("e", "e"), "e"), (("e", "12"), "12"), (("e", "13"), "13"),
            (("e", "23"), "23"), (("e", "123"), "123"), (("e", "132"), "132"),
            
            // (1 2)
            (("12", "e"), "12"), (("12", "12"), "e"), (("12", "13"), "123"),
            (("12", "23"), "132"), (("12", "123"), "13"), (("12", "132"), "23"),
            
            // (1 3)
            (("13", "e"), "13"), (("13", "12"), "132"), (("13", "13"), "e"),
            (("13", "23"), "123"), (("13", "123"), "23"), (("13", "132"), "12"),
            
            // (2 3)
            (("23", "e"), "23"), (("23", "12"), "123"), (("23", "13"), "132"),
            (("23", "23"), "e"), (("23", "123"), "12"), (("23", "132"), "13"),
            
            // (1 2 3)
            (("123", "e"), "123"), (("123", "12"), "23"), (("123", "13"), "12"),
            (("123", "23"), "13"), (("123", "123"), "132"), (("123", "132"), "e"),
            
            // (1 3 2)
            (("132", "e"), "132"), (("132", "12"), "13"), (("132", "13"), "23"),
            (("132", "23"), "12"), (("132", "123"), "e"), (("132", "132"), "123"),
        ];
        
        for ((a, b), result) in compositions {
            operation_table.insert((a.to_string(), b.to_string()), result.to_string());
        }
        
        Group {
            name: "S3".to_string(),
            elements,
            operation_table,
            identity: "e".to_string(),
        }
    }
    
    /// Create the cyclic group Z/nZ
    pub fn cyclic(n: usize) -> Self {
        let elements: Vec<GroupElement> = (0..n)
            .map(|i| GroupElement {
                id: i.to_string(),
                order: if i == 0 { Some(1) } else { Some(n / gcd(i, n)) },
            })
            .collect();
        
        let mut operation_table = HashMap::new();
        for i in 0..n {
            for j in 0..n {
                let result = (i + j) % n;
                operation_table.insert(
                    (i.to_string(), j.to_string()),
                    result.to_string()
                );
            }
        }
        
        Group {
            name: format!("Z/{}", n),
            elements,
            operation_table,
            identity: "0".to_string(),
        }
    }
    
    /// Get the order of the group
    pub fn order(&self) -> usize {
        self.elements.len()
    }
    
    /// Check if this is a cyclic group
    pub fn is_cyclic(&self) -> bool {
        // A group is cyclic if there exists a generator
        for element in &self.elements {
            if let Some(order) = element.order {
                if order == self.order() {
                    return true;
                }
            }
        }
        false
    }
    
    /// Check if this is an abelian group
    pub fn is_abelian(&self) -> bool {
        for a in &self.elements {
            for b in &self.elements {
                let ab = self.operation_table.get(&(a.id.clone(), b.id.clone()));
                let ba = self.operation_table.get(&(b.id.clone(), a.id.clone()));
                if ab != ba {
                    return false;
                }
            }
        }
        true
    }
    
    /// Compose two group elements
    pub fn compose(&self, a: &str, b: &str) -> Option<String> {
        self.operation_table.get(&(a.to_string(), b.to_string())).cloned()
    }
    
    /// Find the inverse of an element
    pub fn inverse(&self, element: &str) -> Option<String> {
        for candidate in &self.elements {
            if let Some(result) = self.compose(element, &candidate.id) {
                if result == self.identity {
                    return Some(candidate.id.clone());
                }
            }
        }
        None
    }
    
    /// Get all subgroups of this group
    pub fn subgroups(&self) -> Vec<Vec<String>> {
        let mut subgroups = Vec::new();
        
        // Trivial subgroup (just identity)
        subgroups.push(vec![self.identity.clone()]);
        
        // Cyclic subgroups generated by each element
        for element in &self.elements {
            let mut subgroup = vec![self.identity.clone()];
            let mut current = element.id.clone();
            
            while current != self.identity {
                if !subgroup.contains(&current) {
                    subgroup.push(current.clone());
                }
                if let Some(next) = self.compose(&current, &element.id) {
                    current = next;
                } else {
                    break;
                }
            }
            
            subgroup.sort();
            if !subgroups.contains(&subgroup) {
                subgroups.push(subgroup);
            }
        }
        
        // The whole group is always a subgroup
        let mut full_group: Vec<String> = self.elements.iter()
            .map(|e| e.id.clone())
            .collect();
        full_group.sort();
        if !subgroups.contains(&full_group) {
            subgroups.push(full_group);
        }
        
        subgroups.sort_by_key(|sg| sg.len());
        subgroups
    }
}

/// Group theory operations for our algebraic engine
pub fn evaluate_group_operation(name: &str, args: &[Value]) -> Result<Value> {
    match name {
        "S3" => {
            if !args.is_empty() {
                return Err(anyhow!("S3() takes no arguments"));
            }
            let group = Group::s3();
            Ok(Value::Matrix(vec![
                vec![Value::Integer(BigInt::from(group.order()))],
                vec![Value::Boolean(group.is_abelian())],
                vec![Value::Boolean(group.is_cyclic())],
            ]))
        }
        
        "cyclic" => {
            if args.len() != 1 {
                return Err(anyhow!("cyclic(n) takes exactly 1 argument"));
            }
            match &args[0] {
                Value::Integer(n) => {
                    if let Some(n_usize) = n.to_usize() {
                        if n_usize > 0 {
                            let group = Group::cyclic(n_usize);
                            Ok(Value::Matrix(vec![
                                vec![Value::Integer(BigInt::from(group.order()))],
                                vec![Value::Boolean(group.is_abelian())],
                                vec![Value::Boolean(group.is_cyclic())],
                            ]))
                        } else {
                            Err(anyhow!("Group order must be positive"))
                        }
                    } else {
                        Err(anyhow!("Group order too large"))
                    }
                }
                _ => Err(anyhow!("cyclic() requires integer argument")),
            }
        }
        
        "compose_s3" => {
            if args.len() != 2 {
                return Err(anyhow!("compose_s3(a, b) takes exactly 2 arguments"));
            }
            
            // For now, we'll use a simple string representation
            let group = Group::s3();
            
            // This would need proper parsing of group element representations
            // For now, return a placeholder
            Ok(Value::Integer(BigInt::zero()))
        }
        
        _ => Err(anyhow!("Unknown group theory function: {}", name)),
    }
}

/// Helper function for GCD
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}