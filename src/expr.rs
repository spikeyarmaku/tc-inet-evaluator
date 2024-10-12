#[derive(Debug, Clone)]
pub struct Expr {
    pub children: Vec<Expr>,
}

impl Expr {
    pub fn new(children: Vec<Expr>) -> Self {
        Self { children }
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
