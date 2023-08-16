#ifndef LEXER_H
#define LEXER_H

#include <string>
#include <vector>
#include "token.h"

namespace lexer
{
    enum TokType
    {
        INTEGER,
        STRING,
        CONSTSRUCTOR,
        ARITHMETIC_OP,
        RELATIONAL_OP,
        KEYWORD,
        WHITESPACE
    };

    class Lexer
    {
        public:
            Lexer(std::string&);

            Token next_token(std::string&, int);
            std::vector <Token> tokens = {};

            ~Lexer();

        private:
            bool is_string = false;
            bool is_digits = false;
            bool is_float = false;
            unsigned int current_line = 1;

            char current_token = 0;
            unsigned int current_index = 0;
    };

};

#endif 