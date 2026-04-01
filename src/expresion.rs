use std::fmt;

pub enum Expr {
    Var(String),  // e.g., "a", "b"
    Not(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Var(name) => write!(f, "{}", name),
            Expr::Not(e) => write!(f, "{}'", e),
            Expr::And(lhs, rhs) => write!(f, "{}{}", lhs, rhs),
            Expr::Or(lhs, rhs) => write!(f, "{} + {}", lhs, rhs),
        }
    }
}


#[test]
fn test_expression(){
    let a = Expr::Var("a".to_string());
    let b = Expr::Var("b".to_string());
    let c = Expr::Var("c".to_string());
    let l = Expr::And(Box::new(b) , Box::new(c));
    let a1 = Expr::And(Box::new(a), Box::new(l));


    println!("the result is: {}",a1 ) 
}
