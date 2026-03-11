pub mod codegen;
pub mod lexer;
pub mod parser;
pub mod sema;

fn main() -> anyhow::Result<()> {
    let mut lexer = lexer::Lexer::new(std::fs::read_to_string("tests/basic_syntax.lpp")?);
    lexer.tokenize()?;
    for (token, span) in &lexer.tokens {
        println!("{token:?} at {span}");
    }
    Ok(())
}
