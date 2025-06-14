#[derive(Debug, PartialEq)]

pub enum TokenType {
    Num, Add, Sub, Div, Mul, Dot,
    Opt, Cpt, Ocl, Ccl, Scln, Equ,
    Eqv, Gre, Les, Geq, Leq, Out,
    In, Loop, If, Elif, Else, Func,
    Iden, Qt, And, Or, Let
}

#[derive(Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub value: String,
}

pub fn lex(file_buffer: &str, pos: &mut usize) -> Option<Token> {
    let chars: Vec<char> = file_buffer.chars().collect();

    while *pos < chars.len() {
        if chars[*pos].is_whitespace() {
            *pos += 1;
            continue;
        }

        if chars[*pos].is_ascii_digit() {
            let mut val = String::new();
            let mut float = false;
            while *pos < chars.len() &&
                  (chars[*pos].is_ascii_digit() || chars[*pos] == '.') {
                if chars[*pos] == '.' {
                    if float {
                        println!("Error: Invalid number (multiple dots)");
                        return None;
                    }
                    float = true;
                }
                val.push(chars[*pos]);
                *pos += 1;
            }
            return Some(Token { ttype: TokenType::Num, value: val });
        } 
        else if chars[*pos].is_ascii_alphabetic() {
        	let mut val = String::new();
            let mut float = false;
            while *pos < chars.len() && chars[*pos].is_ascii_alphabetic() {
                val.push(chars[*pos]);
                *pos += 1;
            }

            // Identifiers
            let token_type = match val.as_str() {
			    "out" => TokenType::Out,
			    "in" => TokenType::In,
			    "loop" => TokenType::Loop,
			    "if" => TokenType::If,
			    "elif" => TokenType::Elif,
			    "else" => TokenType::Else,
			    "fn" => TokenType::Func,
			    "and" => TokenType::And,
			    "or" => TokenType::Or,
			    "let" => TokenType::Let,
			    _ => TokenType::Iden,
			};

			return Some(Token { ttype: token_type, value: val })
        }

        // Match single-character tokens
        let tok = match chars[*pos] {
            '+' => Some(Token { ttype: TokenType::Add, value: "+".to_string() }),
            '-' => Some(Token { ttype: TokenType::Sub, value: "-".to_string() }),
            '*' => Some(Token { ttype: TokenType::Mul, value: "*".to_string() }),
            '/' => Some(Token { ttype: TokenType::Div, value: "/".to_string() }),
            '(' => Some(Token { ttype: TokenType::Opt, value: "(".to_string() }),
            ')' => Some(Token { ttype: TokenType::Cpt, value: ")".to_string() }),
            '{' => Some(Token { ttype: TokenType::Ocl, value: "{".to_string() }),
            '}' => Some(Token { ttype: TokenType::Ccl, value: "}".to_string() }),
            ';' => Some(Token { ttype: TokenType::Scln, value: ";".to_string() }),
            '\"' => Some(Token { ttype: TokenType::Qt, value: "\"".to_string() }),

            '=' => {
			    *pos += 1;
			    if chars[*pos] == '=' {
			        *pos += 1;
			        Some(Token { ttype: TokenType::Eqv, value: "==".to_string() })
			    } else {
			        Some(Token { ttype: TokenType::Equ, value: "=".to_string() })
			    }
			},

			'<' => {
			    *pos += 1;
			    if chars[*pos] == '=' {
			        *pos += 1;
			        Some(Token { ttype: TokenType::Leq, value: "<=".to_string() })
			    } else {
			        Some(Token { ttype: TokenType::Les, value: "<".to_string() })
			    }
			},

			'>' => {
			    *pos += 1;
			    if chars[*pos] == '=' {
			        *pos += 1;
			        Some(Token { ttype: TokenType::Geq, value: ">=".to_string() })
			    } else {
			        Some(Token { ttype: TokenType::Gre, value: ">".to_string() })
			    }
			},

            _ => None,
        };

        *pos += 1;
        return tok;
    }

    None
}
