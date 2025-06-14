use crate::lexer::{Token, TokenType, lex};

pub enum ASTNode {
    Number(f64),
    Op {
        op: String,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
}

fn parse_factor(tokens: &[Token], pos: &mut usize) -> Option<ASTNode> {
    while *pos < tokens.len() {
        match tokens[*pos].ttype {
            TokenType::Num => {
                let num = tokens[*pos].value.parse::<f64>().ok()?;
                *pos += 1;
                return Some(ASTNode::Number(num));
            }

            TokenType::Opt => {
                *pos += 1;
                let node = parse_expr(tokens, pos)?;
                if *pos >= tokens.len() || tokens[*pos].ttype != TokenType::Cpt {
                    return None;
                }
                *pos += 1;
                return Some(node);
            }

            _ => return None,
        }
    }
    None
}

fn parse_term(tokens: &[Token], pos: &mut usize) -> Option<ASTNode> {
    let mut node = parse_factor(tokens, pos)?;

    while *pos < tokens.len() {
        match tokens[*pos].ttype {
            TokenType::Mul | TokenType::Div => {
                let op = tokens[*pos].value.clone();
                *pos += 1;
                node = ASTNode::Op {
                    op,
                    left: Box::new(node),
                    right: Box::new(parse_factor(tokens, pos)?),
                };
            }

            _ => break,
        }
    }

    Some(node)
}

fn parse_expr(tokens: &[Token], pos: &mut usize) -> Option<ASTNode> {
    let mut node = parse_term(tokens, pos)?;

    while *pos < tokens.len() {
        match tokens[*pos].ttype {
            TokenType::Add | TokenType::Sub => {
                let op = tokens[*pos].value.clone();
                *pos += 1;
                node = ASTNode::Op {
                    op,
                    left: Box::new(node),
                    right: Box::new(parse_term(tokens, pos)?),
                };
            }

            _ => break,
        }
    }

    Some(node)
}

pub fn parse(file_buffer: &str, pos: &mut usize) -> Option<ASTNode> {
    let mut ast: Option<ASTNode> = None;

    let mut expr_tokens = Vec::new();

    // Lex tokens until we hit Scln or EOF
    loop {
        let token = lex(file_buffer, pos)?;
        if token.ttype == TokenType::Scln {
            break;
        }
        expr_tokens.push(token);
    }

    let mut local_pos = 0;
    ast = parse_expr(&expr_tokens, &mut local_pos);
    if ast.is_none() {
        return None;
    }

    ast
}
