#include "parser.h"

using namespace parser;

Parser::Parser()
{
    while(current_token <= tokens.size())
    {
        if(tokens[current_token].type == lexer::TOK_PRINT) parse_statement(current_token, lexer::TOK_PRINT);

    }
}

void Parser::consume()
{
    current_token++;
}

void Parser::parse_expr(int current_tok, int tok_type)
{

}

void Parser::parse_statement(int current_tok, int tok_type)
{

}

Parser::~Parser() = default;