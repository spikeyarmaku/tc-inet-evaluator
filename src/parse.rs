use crate::expr::*;

pub fn parse_tree(str: String) -> Option<Expr> {
    let mut expr: Option<Expr> = None;
    let mut stack: Vec<Option<Expr>> = Vec::new();
    for c in str.chars() {
        match c {
            '(' => {
                stack.push(expr.take());
            }
            ')' => {
                if let Some(mut temp) = stack.pop().flatten() {
                    if let Some(some_expr) = expr.take() {
                        temp.children.push(some_expr)
                    }
                    expr = Some(temp);
                }
            }
            _ => {
                match expr {
                    None => {
                        expr = Some(Expr {children: Vec::new()});
                    }
                    Some(ref mut some_expr) => {
                        some_expr.children.push(Expr {children: Vec::new()});
                    }
                }
            }
        }
    }
    expr
}

