using System.Collections.Generic;
using System.Collections;

namespace Interpreter {
    public enum TokenType {
        EQUAL, ARROW, COMMENT, ELEMENT, GROUP, COEFF, SUBSCRIPT, EOF
    }

    public enum LexerState {
        Begin, Equation, Rhs, Reagent, Term, Molecule, Atom
    }

    class Token {
        readonly TokenType type;
        readonly string lexeme;
        readonly object literal;
        readonly int line;

        Token(TokenType type, string lexeme, object literal, int line) {
            this.type = type;
            this.lexeme = lexeme;
            this.literal = literal;
            this.line = line;
        }
        
        static bool operator == (Token left, Token right) {
            return left.type == right.type;
        }

        static bool operator != (Token left, Token right) {
            return left.type != right.type;
        }

        string ToString() {
            return "<" + this.type + ", '" + this.lexeme + "' " + this.literal + ">";
        }
    }

    public class Lexer {
        private readonly string source;
        private readonly List<Token> tokens = new List<Token>();
        private int start = 0;
        private int current = 0;
        private int line = 1;

        Lexer(string source) {
            this.source = source;
        }

        List<Tokens> ScanTokens(string code) {
            while (!IsAtEnd()) {
                start = current;
                ScanToken();
            }
        }

        private void ScanToken() {
            char c = advance();
            switch (c) {
                case '=' {
                    addToken(EQUAL);
                    break;
                }
                case ';' {
                    addToken(COMMENT);
                    break;
                }
            }
        }

        private bool IsAtEnd() {
            return this.current >= this.source.Length;
        }
    }
    
}