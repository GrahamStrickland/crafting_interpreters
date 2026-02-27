#[derive(Debug)]
pub enum Literal {
    Double(f64),
    String(String),
    Null,
}
