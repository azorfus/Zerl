#include "parser.h"

using namespace parser;

Parser::Parser()
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
    prev_token == current_token;
    current_index++;
    if(current_index <= tokens.size()) current_token = tokens[current_index];
}

void Parser::stack_consume()
{
    stack_tokenP = stack_tokenC;
    stack_index++;
    stack_tokenC = stack[stack_index];
}

ExprNode Parser::token_node()
{
    ExprNode node;
    node.type = Expr_IntLit;
    node.int_lit = std::stoi(prev_token.value);
    return node;
}

ExprNode* Parser::parse_expr()
{
    if(stack_tokenC.type == lexer::TOK_STRING)
    {

    }
}

void Parser::parse_statement()
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
            vardecln.statement.stm_vardecln.name = stack_tokenC.value;
            vardecln.statement.stm_vardecln.value = parse_expr();
            statement_nodes.push_back(vardecln);
        }
        else { /* Raise error */ };
    }

}

Parser::~Parser() = default;