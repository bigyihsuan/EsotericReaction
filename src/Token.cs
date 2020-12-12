namespace EsotericReaction {
    public enum TokenType {
        EQUSEP, EQUAL, ARROW, PLUS, ELEMENT, NAME, COEFF, SUBSCRIPT, EOF
    }

    public class Token {
        public readonly TokenType type;
        public readonly string lexeme;
        public readonly object literal;
        public readonly int line;
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

        public override bool Equals(object obj) {
            return base.Equals(obj);
        }

        public override int GetHashCode() {
            return base.GetHashCode();
        }
    }
}
