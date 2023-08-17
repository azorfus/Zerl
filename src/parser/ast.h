#ifndef AST_H
#define AST_H

#include <iostream>
#include <vector>
#include <string>
#include <stdbool.h>
#include <../lexer/token.h>

namespace parser
{

    enum ExprType
    {
        Expr_IntLit, Expr_FloatLit, Expr_BoolLit, 
        Expr_StringLit, Expr_FuncName, Expr_Unary, 
        Expr_Binary, Expr_Assignment, Expr_Variable, 
        Expr_FuncCall, Expr_Dot
    };
    enum ValueType
    {
        Value_IntLit, Value_FloatLit, Value_BoolLit,
        Value_StringLit
    };
    enum NodeType { Expr, Statement };

    typedef struct ExprNode ExprNode;
    struct ExprNode
    {
        ExprType type;
        ValueType ret_type;
        bool can_assign;
        bool is_const; 
        union 
        {
            int int_lit;
            float float_lit;
            std::string string_lit;
            bool bool_lit;

            struct { ExprNode* left; ExprNode* right; } assignment;
            struct { lexer::TOKEN op; ExprNode* opd; } unary;
            struct { lexer::TOKEN op; ExprNode* left; ExprNode* right; } binary;
            struct { std::string name; std::vector <ExprNode*> params; } callfunc;
            struct { ExprNode* left; ExprNode* right; } dot;
        };
    };

    enum StatementType
    {
        Statement_Block, Statement_Expr, Statement_Return,
        Statement_If, Statement_Else, Statement_While,
        Statement_VarDecln, Statement_FuncDecln, Statement_Print,
        Statement_Gets
    };

    typedef struct StatementNode StatementNode;
    struct StatementNode
    {
        StatementType type;
        union 
        {
            ExprNode* expression;
            StatementNode* block;

            struct { ExprNode* ret_value; } stm_return;
            struct { ExprNode* condition; StatementNode* next; } stm_if;
            struct { StatementNode* next; } stm_else;
            struct { ExprNode* condition; StatementNode* next; } stm_while;
            struct { lexer::TOKEN var_type; std::string name; ExprNode* value; } stm_vardecln;
            struct { std::string name; std::vector <lexer::TOKEN> param_types; std::vector <std::string> param_names; StatementNode* block; } stm_func_decln;
            struct { ExprNode* print_value; } stm_print;
            struct { lexer::TOKEN get_type; } stm_gets;

        } statement;
    };

}

#endif