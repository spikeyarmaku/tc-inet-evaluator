#[derive(PartialEq)]
pub enum ExprType {
    Program,
    Application,
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub children: Vec<Expr>,
}

impl Expr {
    pub fn get_type(&self) -> ExprType {
        if self.children.len() < 3 {
            ExprType::Program
        } else {
            ExprType::Application
        }
    }

    pub fn make_leaf() -> Self {
        Self { children: vec![] }
    }

    pub fn make_stem(expr: Expr) -> Self {
        Self {
            children: vec![expr],
        }
    }

    pub fn make_fork(expr0: Expr, expr1: Expr) -> Self {
        Self {
            children: vec![expr0, expr1],
        }
    }

    pub fn to_string(&self) -> String {
        let mut str = String::from("t");
        for c in &self.children {
            let child_str = c.to_string();
            if c.children.len() > 0 {
                str += "(";
                str += &child_str;
                str += ")";
            } else {
                str += &child_str;
            }
        }
        str
    }
}
