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
            std::vector <StatementNode> statement_nodes;
            std::vector <ExprNode> expr_nodes;

        private:
            int current_index = 0;
            int pos = 0;

            lexer::Token current_token;
            lexer::Token prev_token;
            
            std::vector <lexer::Token> stack;
            lexer::Token stack_tokenC;
            lexer::Token stack_tokenP;
            int stack_index = 0;

            void consume();
            void stack_consume();
            
            ExprNode token_node();
            
            ExprNode* parse_expr();
            void parse_statement();
            void parse_if();
            void parse_else();
            void parse_while();
            void parse_for();

            bool fake_quote = false;
            bool is_string = false;

    };
};

#endif