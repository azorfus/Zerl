#include <stdio.h>
#include <iostream>
#include <fstream>
#include <sstream>
#include "lexer/lexer.h"

using namespace std;

int main(int argc, char* argv[])
{
    ifstream file;
    string buffer;
    file.open(argv[1], ios::in);

    if(file)
    {
        ostringstream ss;
        ss << file.rdbuf();
        buffer = ss.str();
    }
    file.close();
    
    lexer::Lexer lexer(buffer);
    for(int i = 0; i<=lexer.tokens.size(); i++)
    {
        cout << '(' << lexer.tokens[i].value << " : " << lexer.tokens[i].type << ", " << lexer.tokens[i].line_number << ") ";
        if(i%5==0 && i!=0) cout << endl;
    };
    cout << endl;

    // feed tokens to parser ...
    return 0;
};