#include "token.h"

using namespace lexer;

Token::Token() = default;

Token::Token(int state, std::string val, unsigned int current_line) : type(find_token_type(state, val)), value(val), line_number(current_line)
{
};

TOKEN Token::find_token_type(int state, std::string &value)
{
    switch(state)
    {
        case 0: return TOK_INT_VAL;
        case 1: return TOK_STRING_VAL;
        case 2:
            if(value == ",") return TOK_COMMA;
            else if(value == ";") return TOK_SEMICOLON;
            else if(value == ":") return TOK_COLON;
            else if(value == ")") return TOK_RBRACKET;
            else if(value == "(") return TOK_LBRACKET;
            else if(value == "}") return TOK_RCURLY;
            else if(value == "{") return TOK_LCURLY;
            else if(value == "\"") return TOK_DQUOTES;
            else if(value == "\'") return TOK_QUOTES;
            else if(value == "\\") return TOK_BACKSL;
            else if(value == ".") return TOK_DOT;
            else if(value == "#") return TOK_COMMENT;
        case 3:
            if(value == "+") return TOK_ADD;
            else if(value == "-") return TOK_SUB;
            else if(value == "*") return TOK_MUL;
            else if(value == "/") return TOK_DIV;
        case 4:
            if(value == "=") return TOK_EQUALS;
            else if(value == ">") return TOK_GREATER;
            else if(value == "<") return TOK_LESSER;
        case 5:
            if(value == "\\n") return TOK_NEWLINE;
            else if(value == "\\\'") return TOK_FALSE_QUOTE;
            else if(value == "\\\"") return TOK_FALSE_DQUOTES;
        case 6:
            if(value == "if") return TOK_IF;
            else if(value == "int") return TOK_INT;
            else if(value == "string") return TOK_STRING;
            else if(value == "bool") return TOK_BOOL;
            else if(value == "float") return TOK_FLOAT;
            else if(value == "else") return TOK_ELSE;
            else if(value == "while") return TOK_WHILE;
            else if(value == "for") return TOK_FOR;
            else if(value == "in") return TOK_IN;
            else if(value == "and") return TOK_AND;
            else if(value == "or") return TOK_OR;
            else if(value == "print") return TOK_PRINT;
            else if(value == "gets") return TOK_GETS;
            else if(value == "func") return TOK_FUNC;
            else if(value == "return") return TOK_RETURN;
            else return TOK_NONE;
        case 7: return TOK_WHSP;
    };
    return TOK_NONE;
};