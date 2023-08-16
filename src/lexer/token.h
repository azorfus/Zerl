#ifndef TOKEN_H
#define TOKEN_H

#include <iostream>
#include <string>
#include <vector>
#include <ctype.h>

namespace lexer
{
    enum TOKEN
    {
        TOK_NONE,

        TOK_INT, TOK_STRING,
        TOK_BOOL, TOK_INT_VAL,
        TOK_STRING_VAL, TOK_DOT,
        TOK_BOOL_VAL, TOK_WHSP,
        TOK_SET, TOK_FUNC,
        TOK_RETURN, TOK_IF,
        TOK_ELSE, TOK_WHILE,
        TOK_FOR, TOK_IN,
        TOK_AND, TOK_OR,
        TOK_PRINT, TOK_GETS, 
        TOK_BACKSL, TOK_COMMENT,

        TOK_NEWLINE, TOK_FALSE_QUOTE,
        TOK_FALSE_DQUOTES,
        
        TOK_ADD, TOK_SUB,
        TOK_MUL, TOK_DIV,
        TOK_COMMA, TOK_SEMICOLON,
        TOK_COLON, TOK_EQUALS,
        TOK_GREATER, TOK_LESSER,
        TOK_RBRACKET, TOK_LBRACKET,
        TOK_RCURLY, TOK_LCURLY,
        TOK_DQUOTES, TOK_QUOTES,

        TOK_EOF, TOK_ERROR
    };

    class Token
    {
        public:
            Token();
            Token(int, std::string, unsigned int);
            TOKEN type;
            std::string value;
            unsigned int line_number;
        
        private:
            TOKEN find_token_type(int, std::string&);
    };
};

#endif 