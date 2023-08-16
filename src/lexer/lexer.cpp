#include "lexer.h"

#define print(s) std::cout << s;
using namespace lexer;

Lexer::Lexer(std::string &buffer)
{
    int buffer_length = buffer.length();
    Token tok;
    while(current_index <= buffer_length)
    {
        tok = next_token(buffer, current_index);
        gotta_go_back:
        if(tok.type == TOK_BACKSL)
        {
            std::string temp = "";
            current_index++;
            if(current_index <= buffer_length) tok = next_token(buffer, current_index);
            if(tok.type == TOK_STRING_VAL && tok.value == "n") 
            { 
                temp.append("\\n"); 
                tokens.push_back(Token(SPECIAL, temp, current_line)); 
                current_index++;
                if(current_index <= buffer_length) tok = next_token(buffer, current_index);
            }
            else if(tok.type == TOK_QUOTES && tok.value == "\'") 
            { 
                temp.append("\\\'"); 
                tokens.push_back(Token(SPECIAL, temp, current_line));
                current_index++;
                if(current_index <= buffer_length) tok = next_token(buffer, current_index);
            }
            else if(tok.type == TOK_DQUOTES && tok.value == "\"") 
            { 
                temp.append("\\\""); 
                tokens.push_back(Token(SPECIAL, temp, current_line));
                current_index++;
                if(current_index <= buffer_length) tok = next_token(buffer, current_index);
            }
            else { current_index--; tok = next_token(buffer, current_index); }
        }
        if(tok.type == TOK_STRING_VAL)
        {
            std::string temp = "";
            is_string = true;
            while(is_string)
            {
                if(tok.type != TOK_STRING_VAL)
                {
                    if(Token(KEYWORD, temp, current_line).type == 1) tokens.push_back(Token(STRING, temp, current_line));
                    else tokens.push_back(Token(KEYWORD, temp, current_line));
                    is_string = false;
                    if(tok.type == TOK_BACKSL) goto gotta_go_back;
                }
                else
                {
                    temp.append(tok.value);
                    current_index += 1;
                    if(current_index <= buffer_length) tok = next_token(buffer, current_index);
                }
            }
        }
        else if(tok.type == TOK_INT_VAL)
        {
            std::string temp = "";
            is_digits = true;
            while(is_digits)
            {
                if(tok.type != TOK_INT_VAL)
                {
                    tokens.push_back(Token(INTEGER, temp, current_line));
                    is_digits = false;
                }
                else
                {
                    temp.append(tok.value);
                    current_index += 1;
                    if(current_index <= buffer_length) tok = next_token(buffer, current_index);
                }
            }
        }
        if(tok.type != TOK_WHSP) tokens.push_back(tok);
        current_index += 1;
    }
};

Token Lexer::next_token(std::string& buffer, int index)
{
    int state;
    std::string val;
    current_token = buffer[index];

    if(current_token == '\n' || current_token == '\t')
    {
        state = WHITESPACE;
        val = "";
    }
    if(current_token == '\n')
    {
        current_line += 1;
    }
    else if(current_token == ' ')
    {
        state = WHITESPACE;
        val = " ";
    }
    else if(current_token == ',' || current_token == ';' || current_token == ':' || current_token == '{' || 
            current_token == '}' || current_token == '(' || current_token == ')' || current_token == '\\' || 
            current_token == '\'' || current_token == '\"' || current_token == '.' || current_token == '#')
    {
        state = CONSTSRUCTOR;
        val = current_token;
    }
    else if(isalpha(current_token) || current_token == '!' || current_token == '@' || current_token == '$')
    {
        state = STRING;
        val = current_token;
    }
    else if(isdigit(current_token))
    {
        state = INTEGER;
        val = current_token;
    }
    else if(current_token == '+' || current_token == '-' || current_token == '*' || current_token == '/')
    {
        state = ARITHMETIC_OP;
        val = current_token;
    }
    else if(current_token == '=' || current_token == '<' || current_token == '>') 
    {
        state = RELATIONAL_OP;
        val = current_token;
    }
    return Token(state, val, current_line);
};

Lexer::~Lexer() = default;