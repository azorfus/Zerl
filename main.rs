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
    pos = pos + 1;

    let mut pos = 0;
    let mut tokens = Vec::new();
    while pos < file_buffer.len() {
        match lexer::lex(&file_buffer, &mut pos) {
            Some(tok) => {
                // println!("{:?}", tok); // Optional: debug print
                tokens.push(tok);
            }
            None => {
                eprintln!("Lexing error near position {}", pos);
                break;
            }
        }
    }

    let mut parser = parser::Parser::new(tokens);

    println!("AST:");

    while !parser.is_at_end() {
        match parser.parse_statement() {
            Some(ast) => {
                pretty_print(&ast, "", true);
            }
            None => {
                eprintln!("Parsing failed.");
                break;
            }
        }
    }

    return Ok(());
}

use parser::ASTNode;

fn pretty_print(node: &ASTNode, prefix: &str, is_last: bool) {
    let connector = if is_last { "└── " } else { "├── " };
    print!("{}", prefix);
    print!("{}", connector);

    match node {
        ASTNode::Number(n) => {
            println!("Number({})", n);
        }

        ASTNode::Identifier(iden) => {
            println!("Identifier({})", iden);
        }

        ASTNode::BoolNode(b) => {
            println!("Bool({})", b);
        }

        ASTNode::BreakNode => {
            println!("Break");
        }

        ASTNode::StrLiteral(s) => {
            println!("StrLiteral(\"{}\")", s);
        }

        ASTNode::BinOpNode { op, left, right } => {
            println!("BinOp('{}')", op);
            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            pretty_print(left, &new_prefix, false);
            pretty_print(right, &new_prefix, true);
        }

        ASTNode::VarDecNode { name, value } => {
            println!("VarDec({})", name);
            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            pretty_print(value, &new_prefix, true);
        }

        ASTNode::AssignNode { name, value } => {
            println!("Assign({})", name);
            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            pretty_print(value, &new_prefix, true);
        }

        ASTNode::IfElseNode {
            condition,
            then_branch,
            elif_branch,
            else_branch,
        } => {
            println!("If");

            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            pretty_print(condition, &new_prefix, false);

            println!("{}{}Then", new_prefix, "├── ");
            for (i, stmt) in then_branch.iter().enumerate() {
                pretty_print(stmt, &format!("{}│   ", new_prefix), i == then_branch.len() - 1 && elif_branch.is_empty() && else_branch.is_none());
            }

            for (i, (elif_cond, elif_block)) in elif_branch.iter().enumerate() {
                println!("{}{}Elif", new_prefix, "├── ");
                let elif_prefix = format!("{}│   ", new_prefix);
                pretty_print(elif_cond, &elif_prefix, false);
                for (j, stmt) in elif_block.iter().enumerate() {
                    pretty_print(stmt, &format!("{}│   ", elif_prefix), j == elif_block.len() - 1);
                }
            }

            if let Some(else_block) = else_branch {
                println!("{}{}Else", new_prefix, "└── ");
                for (i, stmt) in else_block.iter().enumerate() {
                    pretty_print(stmt, &format!("{}    ", new_prefix), i == else_block.len() - 1);
                }
            }
        }

        ASTNode::LoopNode { condition, block } => {
            println!("Loop");
            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            pretty_print(condition, &new_prefix, false);
            for (i, stmt) in block.iter().enumerate() {
                pretty_print(stmt, &new_prefix, i == block.len() - 1);
            }
        }

        ASTNode::FuncCall { name, arguments } => {
            println!("FuncCall({})", name);
            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            for (i, arg) in arguments.iter().enumerate() {
                pretty_print(arg, &new_prefix, i == arguments.len() - 1);
            }
        }

        ASTNode::FuncDef { name, arguments, block } => {
            println!("FuncDef({})", name);
            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });

            println!("{}├── Args: {:?}", new_prefix, arguments);

            for (i, stmt) in block.iter().enumerate() {
                pretty_print(stmt, &new_prefix, i == block.len() - 1);
            }
        }
    }
}

