using System.Collections.Generic;
using System.Collections;
using System.IO;
using System;
using Interpreter;

namespace Interpreter.Lexer {
    public enum TokenType {
        EQUAL, ARROW, PLUS, ELEMENT, NAME, COEFF, SUBSCRIPT, EOF
    }

    public class Token {
        readonly TokenType type;
        readonly string lexeme;
        readonly object literal;
        readonly int line;

        public Token(TokenType type, string lexeme, object literal, int line) {
            this.type = type;
            this.lexeme = lexeme;
            this.literal = literal;
            this.line = line;
        }
        
        public static bool operator == (Token left, Token right) {
            return left.type == right.type;
        }

        public static bool operator != (Token left, Token right) {
            return left.type != right.type;
        }

        public override string ToString() {
            return "<" + this.type + ", '" + this.lexeme + "' " + this.literal + ">";
        }
    }

    public class Lexer {
        private readonly string source;
        private List<Token> tokens = new List<Token>();
        private int start = 0;
        private int current = 0;
        private int line = 1;

        private static List<string> elements;

        public Lexer(string source) {
            this.source = source;
            elements = new List<string>();
            AddElements();
        }

        public List<Token> ScanTokens() {
            while (!IsAtEnd()) {
                start = current;
                ScanToken();
            }
            AddToken(TokenType.EOF);
            return tokens;
        }

        private void ScanToken() {
            char c = Advance();
            if (EsoReaction.debug) {
                Console.Write((int)c);
                Console.WriteLine(": " + c);
            }
            switch (c) {
                case '=':
                    AddToken(TokenType.EQUAL);
                    break;
                case ';':
                    while (Peek() != '\n' && !IsAtEnd()) {
                        Advance();
                    }
                    break;
                case '-':
                    if (Match('>')) {
                        AddToken(TokenType.ARROW);
                    }
                    break;
                case '_':
                    Subscript();
                    break;
                case '+':
                    AddToken(TokenType.PLUS);
                    break;
                case ' ':
                case '\r':
                case '\t':
                    break;
                case '\n':
                    line++;
                    break;
                default:
                    if (char.IsDigit(c)) {
                        Coeff();
                    } else {
                        Element();
                    }
                    break;
                }
        }

        private void Element() {
            if (elements.Contains(source.Substring(start, 2))) {
                // 2-char elements
                Advance();
                AddToken(TokenType.ELEMENT, source.Substring(start, current - start));
            } else if (elements.Contains(source.Substring(start, 1))) {
                // 1-char element
                AddToken(TokenType.ELEMENT, source.Substring(start, current - start));
            } else if (current + 4 < source.Length) {
                if (source.Substring(current, 4) == "heat") {
                    // heat
                    for (int i = 1; i < 4; i++) {
                        Advance();
                    }
                    AddToken(TokenType.ELEMENT, source.Substring(current, current - start));
                }
                Advance();
                if (current + 5 < source.Length) {
                    if (source.Substring(current, 5) == "light") {
                        // light
                        for (int i = 1; i < 5; i++) {
                            Advance();
                        }
                        AddToken(TokenType.ELEMENT, source.Substring(current, current - start));
                    }
                }
            } else {
                // name
                while (!elements.Contains(source.Substring(current, current - start))
                    && source.Substring(current, current - start) != "light"
                    && source.Substring(current, current - start) != "heat") {
                    Advance();
                }
                AddToken(TokenType.NAME, source.Substring(current, current - start));
            }
        }

        private void Subscript() {
            Advance();
            while (char.IsDigit(Peek())) {
                Advance();
            }
            AddToken(TokenType.SUBSCRIPT, int.Parse(source.Substring(start+1, current - start)));
        }

        private void Coeff() {
            while (char.IsDigit(Peek())) {
                Advance();
            }
            AddToken(TokenType.COEFF, int.Parse(source.Substring(start, current - start)));
        }

        private char Peek() {
            if (IsAtEnd()) {
                return '\0';
            }
            return source[current];
        }

        private bool Match(char expected) {
            if (IsAtEnd()) {
                return false;
            }
            if (source[current] != expected) {
                return false;
            }
            current++;
            return true;
        }

        private char Advance() {
            current++;
            return source[current - 1];
        }

        private bool IsAtEnd() {
            return this.current >= this.source.Length;
        }

        private void AddToken(TokenType type) {
            AddToken(type, null);
        }

        private void AddToken(TokenType type, object literal) {
            string lexeme = source.Substring(start, current - start);
            Token token = new Token(type, lexeme, literal, this.line);
            if (EsoReaction.debug) {
                Console.WriteLine("AddToken: " + token);
            }
            tokens.Add(token);
        }

        private void AddElements() {
            string[] elementList = File.ReadAllText("elementlist.csv").Split('\n');
            foreach (string line in elementList) {
                string[] props = line.Split(',');
                if (props.Length > 1) {
                    elements.Add(props[1]);
                }
            }
        }
    }
    
}