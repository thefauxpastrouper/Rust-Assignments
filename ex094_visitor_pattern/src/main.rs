// Visitor Pattern: Implement visitor for complex data structures
// AST interpreter

#[derive(Debug)]
enum Expr {
    IntLit(i64),
    Add(Box<Expr>, Box<Expr>)
}

trait Visitor {
    type Result;
    fn visit_int_lit(&mut self, value: i64) -> Self::Result;
    fn visit_add(&mut self, left: &Expr, right: &Expr) -> Self::Result; 
}

impl Expr {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Result {
        match self {
            Expr::IntLit(n) => visitor.visit_int_lit(*n),
            Expr::Add(l, r) => visitor.visit_add(l, r)
        }
    }
}

struct Interpreter;
impl Visitor for Interpreter {
    type Result = i64;

    fn visit_int_lit(&mut self, value: i64) -> i64 { value }
    fn visit_add(&mut self, left: &Expr, right: &Expr) -> i64 {
        left.accept(self) + right.accept(self)
    }
}

fn main() {
    let expr = Expr::Add(
        Box::new(Expr::IntLit(1)),
        Box::new(Expr::IntLit(2))
    );

    let mut interp = Interpreter;
    println!("{}", expr.accept(&mut interp));
}
