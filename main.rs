mod lexer;
mod parser;

use std::fs;
use std::env;

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename)?;

    let file_buffer = fs::read_to_string(filename)?;

    let mut pos = 0;
    let mut token = lexer::lex(&file_buffer, &mut pos);
    println!("{:?}", token);
    pos = pos + 1;

    // Lex tokens until we hit Scln or EOF
    loop {
        token = lexer::lex(&file_buffer, &mut pos);
        println!("{:?}", token);
        if pos >= file_buffer.len() { break; }
    }

    /*
    let mut expr_index = 1;

    while pos < file_buffer.len() {
        match parser::parse(file_buffer, &mut pos) {
            Some(ast) => {
                println!("\nExpression {} AST:", expr_index);
                pretty_print(&ast, "", true);
                expr_index += 1;
            }
            None => {
                println!("Failed to parse expression at position {}", pos);
                break;
            }
        }
    }
    */

    return Ok(());
}


fn pretty_print(node: &parser::ASTNode, prefix: &str, is_last: bool) {
    let connector = if is_last { "└── " } else { "├── " };
    print!("{}", prefix);
    print!("{}", connector);

    match node {
        parser::ASTNode::Number(n) => {
            println!("Number({})", n);
        }
        parser::ASTNode::Op { op, left, right } => {
            println!("Operator('{}')", op);

            let child_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            pretty_print(left, &child_prefix, false);
            pretty_print(right, &child_prefix, true);
        }
    }
}
