using System;
using EsotericReaction.Tok;

namespace EsotericReaction.Syntax {

    abstract class Equation {
        interface Visitor<R> {
            R VisitAssignmentEquation(Assignment equation);
            R VisitExecutionEquation(Execution equation);
        }

        class Assignment : Equation {
            readonly Reagent reagent;
            readonly Token sign;
            readonly Token group;

            Assignment(Reagent reagent, Token sign, Token group) {
                this.reagent = reagent;
                this.sign = sign;
                this.group = group;
            }


        }

        class Execution : Equation {
            readonly Reagent reagent;
            readonly Token sign;
            readonly Reagent product;

            Execution(Reagent reagent, Token sign, Reagent product) {
                this.reagent = reagent;
                this.sign = sign;
                this.product = product;
            }

            override R Accept<R>(Visitor<R> visitor) {
                throw new NotImplementedException();
            }
        }

        virtual R Accept<R>(Visitor<R> visitor);
    }

    abstract class Expression { }

    class Reagent : Expression {
        readonly Term left;
        readonly Token sign;
        readonly Reagent right;

        Reagent(Term left, Token sign, Reagent right) {
            this.left = left;
            this.sign = sign;
            this.right = right;
        }
    }

    class Term : Expression {
        readonly int? coefficient;
        readonly Molecule molecule;

        Term(int? coefficient, Molecule molecule) {
            this.coefficient = coefficient;
            this.molecule = molecule;
        }
    }

    class Molecule : Expression {
        readonly Atom left;
        readonly Molecule right;

        Molecule(Atom left, Molecule right) {
            this.left = left;
            this.right = right;
        }
    }

    class Atom : Expression {
        readonly Element element;
        readonly Token subscript;

        Atom(Element element, Token subscript) {
            this.element = element;
            this.subscript = subscript;
        }
    }

    class Element : Expression {
        readonly Token element;

        Element(Token element) {
            this.element = element;
        }
    }

}
