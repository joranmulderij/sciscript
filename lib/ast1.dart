sealed class Line1 {
  // TODO: should hold references to the original source code.
}

class ExprLine1 extends Line1 {
  final Expr1 expr;

  ExprLine1(this.expr);

  @override
  String toString() => 'ExprLine1($expr)';
}

class AssignmentLine1 extends Line1 {
  final String identifier;
  final Expr1 expr;
  final AssignmentType1 type;

  AssignmentLine1(this.identifier, this.expr, this.type);

  @override
  String toString() => 'AssignmentLine1($identifier, $expr)';
}

enum AssignmentType1 {
  let,
  var_,
  reassign,
}

class UnitDefLine1 extends Line1 {
  final String name;

  UnitDefLine1(this.name);

  @override
  String toString() => 'UnitDefLine1($name)';
}

class FunDefLine1 extends Line1 {
  final String name;
  final Expr1 body;

  FunDefLine1(this.name, this.body);

  @override
  String toString() => 'FunDefLine1($name, $body)';
}

sealed class Expr1 {
  // TODO: should hold references to the original source code.
}

class NumberExpr1 extends Expr1 {
  // num is used here to allow for both integers and doubles.
  // This is only for type checking of units.
  final num value;

  NumberExpr1(this.value);

  @override
  String toString() => 'NumberExpr1($value)';
}

class IdentifierExpr1 extends Expr1 {
  final String name;

  IdentifierExpr1(this.name);

  @override
  String toString() => 'IdentifierExpr1($name)';
}

class FunctionCallExpr1 extends Expr1 {
  final Expr1 function;
  final List<Expr1> arguments;

  FunctionCallExpr1(this.function, this.arguments);

  @override
  String toString() => 'FunctionCallExpr1($function, $arguments)';
}

class BlockExpr1 extends Expr1 {
  final List<Line1> lines;

  BlockExpr1(this.lines);

  @override
  String toString() => 'BlockExpr1($lines)';
}

class ArrayExpr1 extends Expr1 {
  final List<Expr1> elements;

  ArrayExpr1(this.elements);

  @override
  String toString() => 'ArrayExpr1($elements)';
}

class OperatorExpr1 extends Expr1 {
  final Operator1 operator;
  final Expr1 left;
  final Expr1 right;

  OperatorExpr1(this.operator, this.left, this.right);

  @override
  String toString() => 'OperatorExpr1($operator, $left, $right)';
}

enum Operator1 {
  plus,
  minus,
  star,
  slash,
  doubleStar,
  circumflex, // ^
}
