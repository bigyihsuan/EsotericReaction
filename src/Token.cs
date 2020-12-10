namespace EsotericReaction.Tok {
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

        public static bool operator ==(Token left, Token right) {
            return left.type == right.type;
        }

        public static bool operator !=(Token left, Token right) {
            return left.type != right.type;
        }

        public override string ToString() {
            return "<" + this.line + ": " + this.type + ", '" + this.lexeme + "', " + this.literal + ">";
        }
    }
}
