use crate::lexer::{Token, TokenType, lex};

enum var_value {
    Str(String),
    Val(f64),
}

pub enum ASTNode {

    Number(f64),

    StrLiteral(String),

    BinOpNode {
        op: String,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },

    VarDecNode {
        name: String,
        value: var_value,
    },

    AssignNode {
        name: String,
        value: var_value,
    },

    IfElseNode {
        condition: Box<ASTNode>,
        then_branch: Vec<ASTNode>,
        elif_branch: Vec<(Box<ASTNode>, Vec<ASTNode>)>,
        else_branch: Option<Vec<ASTNode>>,
    },

    LoopNode {
        condition: Box<ASTNode>,
        block: Vec<ASTNode>,
    },

    FuncCall {
        name: String,
        arguments: Vec<ASTNode>,
    },

    FuncDef {
        name: String,
        arguments: Vec<ASTNode>,
        block: Vec<ASTNode>,
    },

}

pub struct parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl parser {

    pub fn new(tokens: Vec<Token>) -> Self {
        return Self {
            tokens,
            pos: 0,
        };
    }

    fn consume(&self) {
        if self.pos + 1 <= self.tokens.len() {
            self.pos+=1;
        }
    }

    fn puke(&self) {
        if self.pos - 1 >= 0 {
            self.pos-=1;
        }
    }

    fn current(&self) -> Option<&Token> {
        return self.tokens.get(self.pos);
    }

    fn same(&self, ttype: TokenType) -> bool {
        if let Some(token) = self.current() {
            return token.ttype == ttype;
        }
        else {
            return false;
        }
    }

    fn parse_factor(&mut self) -> Option<ASTNode> {
        let token = self.current()?; // Safely unwrap Option<&Token>

        match token.ttype {
            TokenType::Num => {
                let num = token.value.parse::<f64>().ok()?;
                self.consume();
                return Some(ASTNode::Number(num));
            }

            TokenType::Opt => { 
                self.consume();
                let node = self.parse_expr()?; 
                let next = self.current()?; 
                if next.ttype != TokenType::Cpt {
                    return None;
                }
                self.consume();
                return Some(node);
            }

            _ => return None,
        }
    }

    fn parse_term(&mut self) -> Option<ASTNode> {
        let mut node = self.parse_factor()?;

        while let Some(token) = self.current() {
            match token.ttype {
                TokenType::Mul | TokenType::Div => {
                    let op = token.value.clone();
                    self.consume();
                    node = ASTNode::BinOpNode {
                        op,
                        left: Box::new(node),
                        right: Box::new(self.parse_factor()?),
                    };
                }

                _ => break,
            }
        }

        return Some(node);
    }

    fn parse_arith_expr(&mut self) -> Option<ASTNode> {
        let mut node = self.parse_term()?;

        while let Some(token) = self.current() {
            match token.ttype {
                TokenType::Add | TokenType::Sub => {
                    let op = token.value.clone();
                    self.consume();
                    node = ASTNode::BinOpNode {
                        op,
                        left: Box::new(node),
                        right: Box::new(self.parse_term()?),
                    };
                }

                _ => break,
            }
        }

        return Some(node);
    }

    fn parse_comp_expr(&mut self) -> Option<ASTNode> {
        let mut node = self.parse_arith_expr()?;

        while let Some(token) = self.current() {
            match token.ttype {
                TokenType::Geq | TokenType::Leq | TokenType::Gre | TokenType::Les | TokenType::Eqv => {
                    let op = token.value.clone();
                    self.consume();
                    node = ASTNode::BinOpNode {
                        op,
                        left: Box::new(node),
                        right: Box::new(self.parse_arith_expr()?),
                    };
                }

                _ => break,
            }
        }

        return Some(node);
    }

    fn parse_logic_expr(&mut self) -> Option<ASTNode> {
        let mut node = self.parse_comp_expr()?;

        while let Some(token) = self.current() {
            match token.ttype {
                TokenType::And | TokenType::Or => {
                    let op = token.value.clone();
                    self.consume();
                    node = ASTNode::BinOpNode {
                        op,
                        left: Box::new(node),
                        right: Box::new(self.parse_comp_expr()?),
                    };
                }

                _ => break,
            }
        }

        return Some(node);
    }

    fn parse_expr(&mut self) -> Option<ASTNode> {
        let mut node = self.parse_comp_expr()?;
        return Some(node);
    }

    fn parse_statement(&mut self) -> Option<ASTNode> {
        match self.current()?.ttype {
            TokenType::Let => self.parse_var_def(),
            TokenType::Func => self.parse_func_def(),
            TokenType::If => self.parse_ifelse(),
            TokenType::Loop => self.parse_loop(),

            TokenType::Iden => {
                let ident = self.current()?.value;
                self.consume();
                if self.current()?.ttype == TokenType::Opt {
                    return self.parse_func_call();
                }
                else if self.current()?.ttype == TokenType::Equ {
                    return self.parse_assign();
                }
                else { 
                    println!("[!]Error parsing at Token: {}", self.pos);
                    return None; 
                }
            }

            _ => self.parse_expr(),
        }
    }

    fn parse_block(&mut self) -> Option<Vec<ASTNode>> {
        self.consume(); // consume {
        
        let mut statements: Vec<ASTNode> = Vec::new();

        while let Some(token) = self.current() {

            if token.ttype == TokenType::Ccl {
                break
            }

            if let Some(node) = self.parse_statement() {
                statements.push(node);
            } else {
                return None;
            }
        } 

        if self.current()?.ttype != TokenType::Ccl {
            return None; // unterminated block
        }

        self.consume(); // Consume }

        return Some(statements);
    }

    fn parse_string(&mut self) -> Option<ASTNode> {
        self.consume(); // consume "
        let mut literal: String = String::new();
        while let Some(token) = self.current() {
            if token.ttype == TokenType::Qt {
                break;
            }
            literal.push_str(&token.value);
            self.consume();
        } 
        
        if self.current()?.ttype != TokenType::Qt {
            return None; // unterminated string
        }
        
        self.consume();
        
        return Some(ASTNode::StrLiteral(literal));
    }

    fn parse_var_def(&mut self) -> Option<ASTNode> {
        self.consume(); // consume the 'let'
        let name = self.current()?;
        self.consume();
        self.consume(); // consume the '='
        let value = self.current()?;

        let mut node = ASTNode::VarDecNode {
            name.var_value,
            value.var_value,
        };
    }

    fn parse_func_def(&mut self) -> Option<ASTNode> {
        self.consume(); // consume the 'fn'
        let name = self.current()?.value.clone();
        self.consume();

        self.consume(); // consume (
        let arguments = self.parse_arguments();
        
        if self.current()?.ttype != TokenType::Cpt {
            return None; // unterminated condition
        }
        
        self.consume(); // consume )
        if let Some(block) = self.parse_block() {      

            let node = ASTNode::FuncDef {
                name,
                arguments,
                block,
            };

            return Some(node);

        }
        else {
            return None;
        }
    }

    fn parse_arguments(&mut self) -> Opetion<ASTNode> {
        
    }

    fn parse_func_call(&mut self) -> Option<ASTNode> {

    }

    fn parse_loop(&mut self) -> Option<ASTNode> {

    }

    fn parse_ifelse(&mut self) -> Option<ASTNode> {

    }

    fn parse_assign(&mut self) -> Option<ASTNode> {

    }

}