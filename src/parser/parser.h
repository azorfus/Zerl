#ifndef PARSER_H
#define PARSER_H

#include <vector>
#include <string>
#include "../lexer/lexer.h"

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
            int current = 0;
    };
};

#endif