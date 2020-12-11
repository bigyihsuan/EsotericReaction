using System;
using System.Text;
using EsotericReaction.Tok;

namespace EsotericReaction.Syntax {
    interface Visitor<R> {
        R VisitAssignmentExpression(Assignment expr);
        R VisitExecutionExpression(Execution expr);
        R VisitReagentExpression(Reagent expr);
        R VisitTermExpression(Term expr);
        R VisitMoleculeExpression(Molecule expr);
        R VisitAtomExpression(Atom expr);
        R VisitElementExpression(Element expr);
    }
    abstract class Expression {
        public abstract R Accept<R>(Visitor<R> visitor);
    }

    class AstPrinter : Visitor<string> {

        string Print(Expression expr) {
            return expr.Accept(this);
        }

        public string VisitAssignmentExpression(Assignment expr) {
            return Parenthesize("=", expr.reagent, expr.reagent);
        }
        public string VisitExecutionExpression(Execution expr) {
            return Parenthesize("->", expr.reagent, expr.reagent);
        }

        public string VisitReagentExpression(Reagent expr) {
            return Parenthesize("+", expr.left, expr.right);
        }

        public string VisitTermExpression(Term expr) {
            return Parenthesize("Term", expr.molecule);
        }

        public string VisitMoleculeExpression(Molecule expr) {
            return Parenthesize("Molecule", expr.left, expr.right);
        }

        public string VisitAtomExpression(Atom expr) {
            return Parenthesize("", expr.element);
        }

        public string VisitElementExpression(Element expr) {
            return Parenthesize("", expr);
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

        Assignment(Reagent reagent, Reagent group) {
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

        Execution(Reagent reagent, Reagent product) {
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

        Reagent(Term left, Reagent right) {
            this.left = left;
            this.right = right;
        }

        public override R Accept<R>(Visitor<R> visitor) {
            return visitor.VisitReagentExpression(this);
        }
    }

    class Term : Expression {
        public readonly Token coefficient;
        public readonly Molecule molecule;

        Term(Token coefficient, Molecule molecule) {
            this.coefficient = coefficient;
            this.molecule = molecule;
        }

        public override R Accept<R>(Visitor<R> visitor) {
            return visitor.VisitTermExpression(this);
        }
    }

    class Molecule : Expression {
        public readonly Atom left;
        public readonly Molecule right;

        Molecule(Atom left, Molecule right) {
            this.left = left;
            this.right = right;
        }

        public override R Accept<R>(Visitor<R> visitor) {
            return visitor.VisitMoleculeExpression(this);
        }
    }

    class Atom : Expression {
        public readonly Element element;
        public readonly Token subscript;

        Atom(Element element, Token subscript) {
            this.element = element;
            this.subscript = subscript;
        }

        public override R Accept<R>(Visitor<R> visitor) {
            return visitor.VisitAtomExpression(this);
        }
    }

    class Element : Expression {
        public readonly Token element;

        Element(Token element) {
            this.element = element;
        }

        public override R Accept<R>(Visitor<R> visitor) {
            return visitor.VisitElementExpression(this);
        }
    }
}
