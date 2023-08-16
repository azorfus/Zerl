#ifndef PARSER_H
#define PARSER_H

#include <vector>
#include <string>
#include "../lexer/lexer.h"
#include "ast.h"

namespace parser
{
    class Parser
    {
        public:

            Parser::Parser();
            Parser::~Parser();
            
            std::vector <lexer::Token> tokens;
            std::vector <union ASTNode> ast_nodes;

        private:
            int current_token = 0;
            void consume();
            void parse_expr(int, int);
            void parse_statement(int, int);
            void parse_if(int, int);
            void parse_else(int, int);
            void parse_while(int, int);
            void parse_for(int, int);

            bool fake_quote = false;
            bool is_string = false;

    };
};

#endif