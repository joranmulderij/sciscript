sealed class Line1 {
  // TODO: should hold references to the original source code.
}

class ExprLine1 extends Line1 {
  final Expr1 expr;

  ExprLine1(this.expr);
}

class AssignmentLine1 extends Line1 {
  final String identifier;
  final Expr1 expr;

  AssignmentLine1(this.identifier, this.expr);
}

class UnitDefLine1 extends Line1 {
  final String name;

  UnitDefLine1(this.name);
}

class FunDefLine1 extends Line1 {
  final String name;
  final Expr1 body;

  FunDefLine1(this.name, this.body);
}

sealed class Expr1 {
  // TODO: should hold references to the original source code.
}

class NumberExpr1 extends Expr1 {
  // num is used here to allow for both integers and doubles.
  // This is only for type checking of units.
  final num value;

  NumberExpr1(this.value);
}

class IdentifierExpr1 extends Expr1 {
  final String name;

  IdentifierExpr1(this.name);
}

class FunctionCallExpr1 extends Expr1 {
  final Expr1 function;
  final Expr1 argument;

  FunctionCallExpr1(this.function, this.argument);
}

class BlockExpr1 extends Expr1 {
  final List<Line1> lines;

  BlockExpr1(this.lines);
}

class OperatorExpr1 extends Expr1 {
  final Operator1 operator;
  final Expr1 left;
  final Expr1 right;

  OperatorExpr1(this.operator, this.left, this.right);
}

enum Operator1 {
  plus,
  minus,
  star,
  slash,
  doubleStar,
}
