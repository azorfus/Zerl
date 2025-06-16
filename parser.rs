use crate::lexer::{Token, TokenType};

#[derive(Debug)]
pub enum ASTNode {

    Number(f64),

    Identifier(String),

    StrLiteral(String),

    BreakNode,

    BoolNode(bool),

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

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {

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

    pub fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn parse_factor(&mut self) -> Option<ASTNode> {
        let token = self.current()?; // Safely unwrap Option<&Token>

        match token.ttype {
            TokenType::Num => {
                let num = token.value.parse::<f64>().ok()?;
                self.consume();
                return Some(ASTNode::Number(num));
            }

            TokenType::Iden => { 
                self.consume();
                if self.current()?.ttype == TokenType::Opt {
                    return self.parse_func_call();
                }
                else { 
                    self.puke(); 
                    let iden = self.current()?.value.clone();
                    self.consume();
                    return Some(ASTNode::Identifier(iden));
                }
            }

            TokenType::Str => {
                let iden = token.value.clone();
                self.consume();
                return Some(ASTNode::StrLiteral(iden));
            }

            TokenType::True => {
                self.consume();
                return Some(ASTNode::BoolNode(true));
            }

            TokenType::False => {
                self.consume();
                return Some(ASTNode::BoolNode(false));
            }

            TokenType::Opt => { 
                self.consume();
                let node = self.parse_expr(false)?; 
                let next = self.current()?; 
                if next.ttype != TokenType::Cpt {
                    println!("[!] Error parsing at Token: {}", self.pos);
                    println!("{:?}", self.current()?);
                    return None;
                }
                self.consume();
                return Some(node);
            }

            _ => {
                    println!("[!] Error parsing at Token: {}", self.pos);
                    println!("{:?}", self.current()?);
                    return None;
                }
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

/* Unnecessary
    fn parse_str_expr(&mut self) -> Option<ASTNode> {
        
        let initial = self.current()?;
        self.consume();

        if self.current()?.ttype == TokenType::Add {
            
            let mut strings: Vec<ASTNode> = Vec::new();
            strings.push(ASTNode::StrLiteral(initial.value.clone()));

            while self.current()?.ttype == TokenType::Add {
                self.consume(); 

                match self.current()?.ttype {
                    TokenType::Str => {
                        let temp = self.current()?.value.clone();
                        strings.push(ASTNode::StrLiteral(temp));
                    },

                    TokenType::Iden => {
                        let temp = self.current()?.value.clone();
                        strings.push(ASTNode::Identifier(temp));
                    },

                    _ => return None, // shout error if other types
                }

                self.consume();
            }

            return Some(ASTNode::StrNode {
                op: String::from("+"),
                values: strings,
            });
        }
        else {
            return Some(ASTNode::StrLiteral(initial.value.clone()));
        }
    }
*/

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

    fn parse_expr(&mut self, terminate: bool) -> Option<ASTNode> {
        match self.current()?.ttype {
            
            TokenType::Iden | TokenType::Num | TokenType::Str => {
                let mut node = self.parse_logic_expr()?;
                if terminate == true && self.current()?.ttype != TokenType::Scln {
                    println!("Expected semicolon!!!");
                    return None;
                } else if terminate == true && self.current()?.ttype == TokenType::Scln {
                    self.consume();
                }

                return Some(node);
            }

            _ =>    {
                        println!("[!] Error parsing at Token (Expression) : {}", self.pos);
                        println!("{:?}", self.current()?);
                        None
                    },
        }

    }

    pub fn parse_statement(&mut self) -> Option<ASTNode> {
        match self.current()?.ttype {
            TokenType::Let => self.parse_var_def(),
            TokenType::Func => self.parse_func_def(),
            TokenType::If => self.parse_ifelse(),
            TokenType::Loop => self.parse_loop(),

            TokenType::Break => {
                                    self.consume(); // consume break
                                    if self.current()?.ttype != TokenType::Scln {
                                        println!("Expected semicolon near break!");
                                        return None;
                                    }
                                    self.consume();
                                    Some(ASTNode::BreakNode)
                                }

            TokenType::Return => {
                                    self.consume(); // consume return
                                    let node = self.parse_expr(false)?;
                                    if self.current()?.ttype != TokenType::Scln {
                                        println!("Expected semicolon near return!");
                                        return None;
                                    }
                                    self.consume();
                                    Some(ASTNode::BreakNode)
                                }

            TokenType::Iden => {

                self.consume();
                if self.current()?.ttype == TokenType::Opt {
                    self.puke();
                    let node = self.parse_func_call();

                    println!("{:?}", self.current()?);
                    if self.current()?.ttype != TokenType::Scln {
                        println!("Expected semicolon!!!");
                        return None;
                    }
                    self.consume();
                    return node;
                }
                else if self.current()?.ttype == TokenType::Equ {
                    self.puke();
                    return self.parse_assign();
                }
                else {  
                    println!("[!]Error parsing at Token (Statement) : {}", self.pos);
                    println!("{:?}", self.current()?);
                    return None; 
                }
            }

            _ => self.parse_expr(true),
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
                println!("[!] Error parsing at Token: {}", self.pos);
                println!("{:?}", self.current()?);
                return None;
            }
        } 

        if self.current()?.ttype != TokenType::Ccl {
            println!("[!] Error parsing at Token: (Block Error) {}", self.pos);
            println!("{:?}", self.current()?);
            return None; // unterminated block
        }

        self.consume(); // Consume }

        return Some(statements);
    }

    fn parse_var_def(&mut self) -> Option<ASTNode> {
        self.consume(); // consume the 'let'
        let name = self.current()?.value.clone();
        self.consume();
        self.consume(); // consume the '='
        let value = self.parse_expr(true)?;

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

        if let Some(block) = self.parse_block() {      

            let node = ASTNode::FuncDef {
                name,
                arguments,
                block,
            };

            return Some(node);

        }
        else {
            println!("[!] Error parsing at Token: {}", self.pos);
            println!("{:?}", self.current()?);
            return None;
        }
    }

    fn parse_func_call(&mut self) -> Option<ASTNode> {
        let name = self.current()?.value.clone();
        self.consume();

        let arguments = self.parse_args_call()?;

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

                _ => {
                        println!("[!] Error parsing at Token: {}", self.pos);
                        println!("{:?}", self.current()?);
                        return None;
                     }
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

            let node = self.parse_expr(false)?; 
            arguments.push(node);

            match self.current()?.ttype {
                TokenType::Com => {
                    self.consume(); // consume ,
                }
                TokenType::Cpt => {
                    self.consume(); // consume )
                    return Some(arguments);
                }
                _ => {
                        println!("[!] Error parsing at Token: (Call error) {}", self.pos);
                        println!("{:?}", self.current()?);
                        return None;
                     }
            }
        }

    }

    fn parse_loop(&mut self) -> Option<ASTNode> {
        self.consume(); // consume loop identifier

        if self.current()?.ttype != TokenType::Opt {
            println!("[!] Error parsing at Token: {}", self.pos);
            println!("{:?}", self.current()?);
            return None;
        }
        self.consume(); // consume (

        let condition = self.parse_expr(false)?;

        if self.current()?.ttype != TokenType::Cpt {
            println!("[!] Error parsing at Token: {}", self.pos);
            println!("{:?}", self.current()?);
            return None;
        }
        self.consume(); // consume )

        let block = self.parse_block()?;

        let node = ASTNode::LoopNode {
            condition: Box::new(condition),
            block,
        };

        return Some(node);

    }

    /* For reference:
        
        IfElseNode {
            condition: Box<ASTNode>,
            then_branch: Vec<ASTNode>,
            elif_branch: Vec<(Box<ASTNode>, Vec<ASTNode>)>,
            else_branch: Option<Vec<ASTNode>>,
        },
    
    */

    fn parse_ifelse(&mut self) -> Option<ASTNode> {
        self.consume(); // consume if identifier

        if self.current()?.ttype != TokenType::Opt {
            println!("[!] Error parsing at Token: {}", self.pos);
            println!("{:?}", self.current()?);
            return None;
        }
        self.consume(); // consume (

        let ifcondition = self.parse_expr(false)?;
        if self.current()?.ttype != TokenType::Cpt {
            println!("[!] Error parsing at Token: {}", self.pos);
            println!("{:?}", self.current()?);
            return None;
        }
        self.consume(); // consume )

        let then_branch = self.parse_block()?;

        let mut elif_branches: Vec<(Box<ASTNode>, Vec<ASTNode>)> = Vec::new();
        while self.current()?.ttype == TokenType::Elif {

            self.consume(); // consume elif identifier

            if self.current()?.ttype != TokenType::Opt {
                println!("[!] Error parsing at Token: {}", self.pos);
                println!("{:?}", self.current()?);
                return None;
            }
            self.consume(); // consume (

            let elifcondition = self.parse_expr(false)?;
            if self.current()?.ttype != TokenType::Cpt {
                println!("[!] Error parsing at Token: {}", self.pos);
                println!("{:?}", self.current()?);
                return None;
            }
            self.consume(); // consume )

            let ethen_branch = self.parse_block()?;

            elif_branches.push((Box::new(elifcondition), ethen_branch));
        }

        let else_branch = if self.current()?.ttype == TokenType::Else {
            
                self.consume();
                Some(self.parse_block()?)
            }
            else {
                println!("[!] Error parsing at Token: {}", self.pos);
                println!("{:?}", self.current()?);
                None
            };


        return Some(ASTNode::IfElseNode {
            condition: Box::new(ifcondition),
            then_branch,
            elif_branch: elif_branches,
            else_branch,
        });

    }

    fn parse_assign(&mut self) -> Option<ASTNode> {
        let name = self.current()?.value.clone();
        self.consume();
        self.consume(); // consume =

        let value = self.parse_expr(true)?;

        let node = ASTNode::AssignNode {
            name,
            value: Box::new(value),
        };

        return Some(node);
    }

}