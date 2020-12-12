using System.Collections.Generic;

namespace EsotericReaction {
    class Parser {
        private readonly List<Token> tokens;
        private int current = 0;

        Parser(List<Token> tokens) {
            this.tokens = tokens;
        }

        private Expression Code() {
            Expression code = Equation();
            while (Match(TokenType.EQUSEP)) {
                Token sep = Previous();
                Expression right = Equation();
                code = new Code((Equation)code, sep, (Code)right);
            }
            return code;
        }

        private Expression Equation() {
            Expression equation = Reagent();
            while (Match(TokenType.ARROW, TokenType.EQUAL)) {
                Token op = Previous();
                Expression right = Reagent();
                equation = new Equation((Reagent)equation, op, (Reagent)right);
            }
            return equation;
        }

        private Expression Reagent() {
            Expression reagent = Term();
            while (Match(TokenType.PLUS)) {
                Token plus = Previous();
                Expression right = Term();
                reagent = new Reagent((Term)reagent, plus, (Reagent)right);
            }
            return reagent;
        }

        private Expression Term() {
            if (Match(TokenType.COEFF)) {
                Token coeff = Previous();
                Expression right = Molecule();
                return new Term(coeff, (Molecule)right);
            }
            return Molecule();
        }

        private Expression Molecule() {
            Expression molecule = Element();
            if (Match(TokenType.ELEMENT)) {
                Expression right = Molecule();
                molecule = new Molecule((Element)molecule, (Molecule)right);
            }
            return molecule;
        }

        private Expression Element() {
            if (Match(TokenType.SUBSCRIPT)) {
                Expression subscript = Subscript();
                return new Element(Previous(), (Subscript)subscript);
            }
            return new Element(Peek(), null);
        }

        private Expression Subscript() {
            return new Subscript(Peek());
        }

        private bool Match(params TokenType[] types) {
            foreach (TokenType type in types) {
                if (Check(type)) {
                    Advance();
                    return true;
                }
            }
            return false;
        }

        private bool Check(TokenType type) {
            if (IsAtEnd()) {
                return false;
            }
            return Peek().type == type;
        }

        private Token Advance() {
            if (!IsAtEnd()) {
                current++;
            }
            return Previous();
        }

        private bool IsAtEnd() {
            return Peek().type == TokenType.EOF;
        }

        private Token Peek() {
            return tokens[current];
        }

        private Token Previous() {
            return tokens[current - 1];
        }
    }
}
