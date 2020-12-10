using System.Collections.Generic;
using System.IO;
using System;
using EsotericReaction.Tok;

namespace EsotericReaction.Lex {
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
                case '(':
                    Name();
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
                    } else if (char.IsLetter(c)) {
                        Element();
                    }
                    break;
                }
        }

        private void Name() {
            while (!Match(')')) {
                Advance();
            }
            AddToken(TokenType.NAME, source.Substring(start+1, current - start - 1));
        }

        private void Element() {
            if (Peek() == 'h' || Peek() == 'l') {
                while (char.IsLetter(Peek())) {
                    Advance();
                }
                AddToken(TokenType.ELEMENT, source.Substring(start, current - start));
            } else {
                if (elements.Contains(source.Substring(start, 2))) {
                    // 2-char elements
                    Advance();
                    AddToken(TokenType.ELEMENT, source.Substring(start, current - start));
                } else if (elements.Contains(source.Substring(start, 1))) {
                    // 1-char element
                    AddToken(TokenType.ELEMENT, source.Substring(start, current - start));
                }
            }
        }

        private void Subscript() {
            //Advance();
            while (char.IsDigit(Peek())) {
                Advance();
            }
            AddToken(TokenType.SUBSCRIPT, int.Parse(source.Substring(start+1, current - start - 1)));
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