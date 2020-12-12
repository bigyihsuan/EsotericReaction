using System.Text;

namespace EsotericReaction {
    interface Visitor<R> {
        R VisitCode(Code expr);
        R VisitEquation(Equation expr);
        R VisitReagent(Reagent expr);
        R VisitTerm(Term expr);
        R VisitMolecule(Molecule expr);
        R VisitElement(Element expr);
        R VisitSubscript(Subscript expr);
    }
    abstract class Expression {
        public abstract R Accept<R>(Visitor<R> visitor);
    }

    class AstPrinter : Visitor<string> {

        public string Print(Expression expr) {
            return expr.Accept(this);
        }

        public string VisitCode(Code expr) {
            return Parenthesize("", expr);
        }

        public string VisitEquation(Equation expr) {
            return Parenthesize(expr.op.lexeme, expr.left, expr.right);
        }

        public string VisitReagent(Reagent expr) {
            return Parenthesize("+", expr.left, expr.right);
        }

        public string VisitTerm(Term expr) {
            return Parenthesize(expr.coefficient.lexeme, expr.molecule);
        }

        public string VisitMolecule(Molecule expr) {
            return Parenthesize("", expr.left, expr.right);
        }

        public string VisitElement(Element expr) {
            return Parenthesize(expr.element.lexeme, expr.subscript);
        }

        public string VisitSubscript(Subscript expr) {
            return expr.coefficient.lexeme;
        }

        private string Parenthesize(string name, params Expression[] exprs) {
            StringBuilder builder = new StringBuilder();
            builder.Append("(").Append(name);
            foreach (Expression e in exprs) {
                builder.Append(" ");
                builder.Append(e.Accept(this));
            }
            builder.Append(")");
            return builder.ToString();
        }
    }
    class Code : Expression {
        public readonly Equation equation;
        public readonly Token sep;
        public readonly Code code;

        public Code(Equation equation, Token sep, Code code) {
            this.equation = equation;
            this.sep = sep;
            this.code = code;
        }

        public override R Accept<R>(Visitor<R> visitor) {
            return visitor.VisitCode(this);
        }
    }

    class Equation : Expression {
        public readonly Reagent left;
        public readonly Token op;
        public readonly Reagent right;

        public Equation(Reagent left, Token op, Reagent right) {
            this.left = left;
            this.op = op;
            this.right = right;
        }

        public override R Accept<R>(Visitor<R> visitor) {
            return visitor.VisitEquation(this);
        }
    }

    class Reagent : Expression {
        public readonly Term left;
        public readonly Token plus;
        public readonly Reagent right;

        public Reagent(Term left, Token plus, Reagent right) {
            this.left = left;
            this.plus = plus;
            this.right = right;
        }

        public override R Accept<R>(Visitor<R> visitor) {
            return visitor.VisitReagent(this);
        }
    }

    class Term : Expression {
        public readonly Token coefficient;
        public readonly Molecule molecule;

        public Term(Token coefficient, Molecule molecule) {
            this.coefficient = coefficient;
            this.molecule = molecule;
        }

        public override R Accept<R>(Visitor<R> visitor) {
            return visitor.VisitTerm(this);
        }
    }

    class Molecule : Expression {
        public readonly Element left;
        public readonly Molecule right;

        public Molecule(Element left, Molecule right) {
            this.left = left;
            this.right = right;
        }

        public override R Accept<R>(Visitor<R> visitor) {
            return visitor.VisitMolecule(this);
        }
    }

    class Element : Expression {
        public readonly Token element;
        public readonly Subscript subscript;

        public Element(Token element, Subscript subscript) {
            this.element = element;
            this.subscript = subscript;
        }

        public override R Accept<R>(Visitor<R> visitor) {
            return visitor.VisitElement(this);
        }
    }

    class Subscript : Expression {
        public readonly Token coefficient;

        public Subscript(Token coefficient) {
            this.coefficient = coefficient;
        }

        public override R Accept<R>(Visitor<R> visitor) {
            return visitor.VisitSubscript(this);
        }
    }
}
