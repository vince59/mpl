#[derive(Debug)]
pub struct Program {
    pub imports: Vec<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug)]
pub enum Stmt {
    Print(String),
    Println(String),
}
