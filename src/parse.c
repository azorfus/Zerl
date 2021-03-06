#include "parse.h"
#include "lex.h"
#include "toktype.h"

int vars[30][2] = {};

void next_tok(char** src, token_lex* Tok, int* IT)
{
	*Tok = tokenize(*src, *IT);
	*IT = Tok->index;
}

void parse(char* src)
{
	token_lex Tok;
	int IT = 0;

	for(int i=0;i<strlen(src);i++)
	{
		next_tok(&src, &Tok, &IT);
		if(Tok.type==8)
		{
			next_tok(&src, &Tok, &IT);
			if(Tok.type != 10)
			{
				printf("Parse error: [%s] not an Identifier! Expected Identifier after var\n", Tok.token);
				exit(0);
			}
			else
			{
				next_tok(&src, &Tok, &IT);
				if(Tok.type == 15)
				{
					next_tok(&src, &Tok, &IT);
					if(Tok.type == 1)
					{

					}
					else
					{
						printf("Parse error: [%s] not an integer!\n", Tok.token);
					}
				}
				else
				{

				}
			}
		}
	}
}