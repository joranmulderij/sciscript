import 'package:sciscript/types.dart';

sealed class Line2 {
  const Line2();
}

class ExprLine2 extends Line2 {
  final Expr2 expr;

  const ExprLine2(this.expr);
}

class AssignmentLine2 extends Line2 {
  final String identifier;
  final Expr2 expr;

  const AssignmentLine2(this.identifier, this.expr);
}

// class FunDefLine2

sealed class Expr2 {
  final MyType type;
  const Expr2(this.type);
}

class FunctionCallExpr2 extends Expr2 {
  final Expr2 function;
  final List<Expr2> arguments;

  const FunctionCallExpr2(this.function, this.arguments, super.type);
}

class NumberExpr2 extends Expr2 {
  final num value;

  NumberExpr2(this.value, super.type);
}

class IdentifierExpr2 extends Expr2 {
  final String name;

  IdentifierExpr2(this.name, super.type);
}

class BlockExpr2 extends Expr2 {
  final List<Line2> lines;

  BlockExpr2(this.lines, super.type);
}

class ArrayExpr2 extends Expr2 {
  final List<Expr2> elements;

  ArrayExpr2(this.elements, super.type);
}

class OperatorExpr2 extends Expr2 {
  final Operator2 operator;
  final Expr2 left;
  final Expr2 right;

  OperatorExpr2(this.left, this.operator, this.right, super.type);
}

enum Operator2 {
  plus,
  minus,
  multiply,
  divide,
  power,
}
