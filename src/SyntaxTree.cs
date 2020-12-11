using System.Text;
using EsotericReaction.Tok;

/* TODO:
    * Add Code class that has a List<Equation>
    * Reagent has List<Term>
    * Term has Coefficient and List<Atom>
 */

namespace EsotericReaction.Ast {
    interface Visitor<R> {
        R VisitAssignmentExpression(Assignment expr);
        R VisitExecutionExpression(Execution expr);
        R VisitReagentExpression(Reagent expr);
        R VisitTermExpression(Term expr);
        R VisitMoleculeExpression(Molecule expr);
        R VisitElementExpression(Element expr);
        R VisitCoefficientExpression(Coefficient expr);
    }
    abstract class Expression {
        public abstract R Accept<R>(Visitor<R> visitor);
    }

    class AstPrinter : Visitor<string> {

        public string Print(Expression expr) {
            return expr.Accept(this);
        }

        public string VisitAssignmentExpression(Assignment expr) {
            return Parenthesize("=", expr.reagent, expr.group);
        }
        public string VisitExecutionExpression(Execution expr) {
            return Parenthesize("->", expr.reagent, expr.product);
        }

        public string VisitReagentExpression(Reagent expr) {
            return Parenthesize("+", expr.left, expr.right);
        }

        public string VisitTermExpression(Term expr) {
            return Parenthesize("Term", expr.coefficient, expr.molecule);
        }

        public string VisitMoleculeExpression(Molecule expr) {
            return Parenthesize("Molecule", expr.left, expr.right);
        }

        public string VisitElementExpression(Element expr) {
            return Parenthesize(expr.element.lexeme, expr.subscript);
        }

        public string VisitCoefficientExpression(Coefficient expr) {
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
    class Assignment : Expression {
        public readonly Reagent reagent;
        public readonly Reagent group;

        public Assignment(Reagent reagent, Reagent group) {
            this.reagent = reagent;
            this.group = group;
        }

        public override R Accept<R>(Visitor<R> visitor) {
            return visitor.VisitAssignmentExpression(this);
        }
    }

    class Execution : Expression {
        public readonly Reagent reagent;
        public readonly Reagent product;

        public Execution(Reagent reagent, Reagent product) {
            this.reagent = reagent;
            this.product = product;
        }

        public override R Accept<R>(Visitor<R> visitor) {
            return visitor.VisitExecutionExpression(this);
        }
    }

    class Reagent : Expression {
        public readonly Term left;
        public readonly Reagent right;

        public Reagent(Term left, Reagent right) {
            this.left = left;
            this.right = right;
        }

        public override R Accept<R>(Visitor<R> visitor) {
            return visitor.VisitReagentExpression(this);
        }
    }

    class Term : Expression {
        public readonly Coefficient coefficient;
        public readonly Molecule molecule;

        public Term(Coefficient coefficient, Molecule molecule) {
            this.coefficient = coefficient;
            this.molecule = molecule;
        }

        public override R Accept<R>(Visitor<R> visitor) {
            return visitor.VisitTermExpression(this);
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
            return visitor.VisitMoleculeExpression(this);
        }
    }

    class Element : Expression {
        public readonly Token element;
        public readonly Coefficient subscript;

        public Element(Token element, Coefficient subscript) {
            this.element = element;
            this.subscript = subscript;
        }

        public override R Accept<R>(Visitor<R> visitor) {
            return visitor.VisitElementExpression(this);
        }
    }

    class Coefficient : Expression {
        public readonly Token coefficient;

        public Coefficient(Token coefficient) {
            this.coefficient = coefficient;
        }

        public override R Accept<R>(Visitor<R> visitor) {
            return visitor.VisitCoefficientExpression(this);
        }
    }
}
