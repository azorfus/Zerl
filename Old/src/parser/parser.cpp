#include "parser.h"

using namespace parser;

Parser::Parser(std::vector <lexer::Token> inp) : tokens(inp)
{
    while(current_index <= tokens.size())
    {
        current_token = tokens[current_index];
        while(current_token.type != lexer::TOK_SEMICOLON)
        {
            stack.push_back(current_token);
            consume();
        }
        if(current_token.type == lexer::TOK_SEMICOLON){ parse_statement(); consume(); }
    }
}

void Parser::consume()
{
    prev_token = current_token;
    current_index++;
    if(current_index <= tokens.size()) current_token = tokens[current_index];
}

void Parser::stack_consume()
{
    stack_tokenP = stack_tokenC;
    stack_index++;
    stack_tokenC = stack[stack_index];
}

lexer::TOKEN Parser::tok_peek(int n)
{
    return stack[stack_index+n].type;
}

ExprNode Parser::token_node()
{
    ExprNode node;
    node.type = Expr_IntLit;
    node.int_lit = std::stoi(prev_token.value);
    return node;
}

/*
    a + (b - (c * (d / e)))
    ((a / b) * c) - d + e

    if you parse `5 + 5` its
    
    + -- 5
    |
    ---5

    but if you had to parse `5 + (3 + 2)` suddenly it becomes
    
    + -- 5
    |
    --- + -- 2
        |
        --- 3

*/

ExprNode* Parser::parse_primary()
{
    ExprNode giveback;
    if(stack_tokenC.type == lexer::TOK_INT_VAL)
    {
        giveback.int_lit = std::stoi(stack_tokenC.value);
    }
    else if(stack_tokenC.type == lexer::TOK_LBRACKET)
    {
        giveback = parse_expr();
        stack_consume(); // consume '(' ?
    }
    return &expr_nodes[expr_nodes.size()];
}

ExprNode* Parser::parse_term()
{
    ExprNode* leftist = parse_primary();
    ExprNode* rightist;
    ExprNode giveback;
    while(stack_tokenC.type == lexer::TOK_MUL || stack_tokenC.type == lexer::TOK_DIV)
    {
        giveback.binary.op = stack_tokenC.type;
        stack_consume();
        rightist = parse_primary();
        giveback.binary.right = rightist;
        giveback.binary.left = leftist;
    }
    return &giveback;

}

ExprNode* Parser::parse_expr()
{
    ExprNode* leftist = parse_term();
    ExprNode giveback;
    while(stack_tokenC.type == lexer::TOK_ADD || stack_tokenC.type == lexer::TOK_SUB)
    {
        giveback.binary.op = stack_tokenC.type;
        consume();
        ExprNode* right = parse_term();
        giveback.type = Expr_Binary;
        giveback.binary.left = leftist;
        giveback.binary.right = right;
    }
    return &giveback;
}

StatementNode* Parser::parse_statement()
{
    if(stack_tokenC.type == lexer::TOK_INT || stack_tokenC.type == lexer::TOK_STRING ||
       stack_tokenC.type == lexer::TOK_BOOL || stack_tokenC.type == lexer::TOK_FLOAT)
    {
        stack_consume();
        if((stack_tokenC.type != lexer::TOK_DQUOTES || stack_tokenC.type != lexer::TOK_QUOTES) 
            && (stack_tokenC.type == lexer::TOK_STRING))
        {
            StatementNode vardecln;
            vardecln.type = Statement_VarDecln;
            vardecln.stm_vardecln.name = stack_tokenC.value;
            vardecln.stm_vardecln.value = parse_expr();
            statement_nodes.push_back(vardecln);
        }
        else { /* Raise error */ };
    }

}

Parser::~Parser() = default;