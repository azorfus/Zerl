use crate::lexer::{Token, TokenType, lex};

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
        value: Box<ASTNode>,
    },

    AssignNode {
        name: String,
        value: Box<ASTNode>,
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
        arguments: Vec<String>,
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

    fn consume(&mut self) {
        if self.pos + 1 < self.tokens.len() {
            self.pos+=1;
        }
    }

    fn puke(&mut self) {
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
        
        return Some(ASTNode::StrLiteral(literal));
    }

    fn parse_var_def(&mut self) -> Option<ASTNode> {
        self.consume(); // consume the 'let'
        let name = self.current()?.value.clone();
        self.consume();
        self.consume(); // consume the '='
        let value = self.parse_expr()?;

        if self.current()?.ttype != TokenType::Scln {
            return None;
        }

        self.consume(); // consume ;

        let mut node = ASTNode::VarDecNode {
            name: name,
            value: Box::new(value),
        };

        return Some(node);
    }

    fn parse_func_def(&mut self) -> Option<ASTNode> {   
        self.consume(); // consume the 'fn'
        let name = self.current()?.value.clone();
        self.consume();

        let arguments = self.parse_args_def()?;
        
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

    fn parse_func_call(&mut self) -> Option<ASTNode> {
        self.puke();
        let name = self.current()?.value.clone();
        self.consume();

        let arguments = self.parse_args_call()?;
        
        if self.current()?.ttype != TokenType::Cpt {
            return None; // unterminated condition
        }
        
        self.consume(); // consume )

        if self.current()?.ttype != TokenType::Scln {
            return None; // unterminated condition
        }

        self.consume(); // consume ;

        let node = ASTNode::FuncCall {
            name,
            arguments,
        };

        return Some(node); 
    }

    fn parse_args_def(&mut self) -> Option<Vec<String>> {
        self.consume(); // consume (

        let mut arguments = Vec::new();

        while let Some(token) = self.current() {

            if token.ttype == TokenType::Cpt {
                self.consume(); // consume )
                return Some(arguments);
            }

            arguments.push(token.value.clone());
            self.consume(); // consume identifier

            match self.current()?.ttype {

                TokenType::Com => {
                    self.consume(); // consume ,
                }
                TokenType::Cpt => {
                    self.consume(); // consume )
                    return Some(arguments);
                }

                _ => return None,

            }
        }

        None
    }

    fn parse_args_call(&mut self) -> Option<Vec<ASTNode>> {
        self.consume(); // consume (

        let mut arguments = Vec::new();

        loop {
            if self.current()?.ttype == TokenType::Cpt {
                self.consume(); // consume )
                return Some(arguments);
            }

            let node = self.parse_expr()?; 
            arguments.push(node);

            match self.current()?.ttype {
                TokenType::Com => {
                    self.consume(); // consume ,
                }
                TokenType::Cpt => {
                    self.consume(); // consume )
                    return Some(arguments);
                }
                _ => return None,
            }
        }

        None
    }

    fn parse_loop(&mut self) -> Option<ASTNode> {
        let mut node = self.parse_comp_expr()?;
        return Some(node);

    }

    fn parse_ifelse(&mut self) -> Option<ASTNode> {
        let mut node = self.parse_comp_expr()?;
        return Some(node);

    }

    fn parse_assign(&mut self) -> Option<ASTNode> {
        self.puke();
        let name = self.current()?.value.clone();
        self.consume();

        self.consume(); // consume =

        let value = self.parse_expr()?;

        let node = ASTNode::AssignNode {
            name,
            value: Box::new(value),
        };

        if self.current()?.ttype != TokenType::Scln {
            return None; // unterminated condition
        }

        self.consume(); // consume ;

        return Some(node);
    }

}