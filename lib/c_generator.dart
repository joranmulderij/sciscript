import 'package:sciscript_dart/ast2.dart';
import 'package:sciscript_dart/types.dart';

String generateCFromLines(List<Line2> lines) {
  final buffer = StringBuffer();
  buffer.writeln('#include <stdio.h>');
  buffer.writeln('int main() {');
  for (final line in lines) {
    buffer.writeln(_generateCFromLine(line));
  }
  buffer.writeln('return 0;');
  buffer.writeln('}');
  return buffer.toString();
}

String _generateCFromLine(Line2 line) {
  return switch (line) {
    AssignmentLine2(:final identifier, :final expr) =>
      '${_generateCFromType(expr.type, identifier)} = ${generateCFromExpr(expr)};',
    // Cast to void to suppress unused value warning
    ExprLine2(:final expr) => '(void) ${generateCFromExpr(expr)};',
  };
}

String generateCFromExpr(Expr2 expr) {
  return switch (expr) {
    NumberExpr2(:final value) => value.toString(),
    IdentifierExpr2(:final name) => name,
    OperatorExpr2(:final operator, :final left, :final right) => () {
        final leftValue = generateCFromExpr(left);
        final rightValue = generateCFromExpr(right);
        return switch (operator) {
          Operator2.plus => '($leftValue + $rightValue)',
          Operator2.minus => '($leftValue - $rightValue)',
          Operator2.multiply => '($leftValue * $rightValue)',
          Operator2.divide => '($leftValue / $rightValue)',
          Operator2.power => 'pow($leftValue, $rightValue)',
        };
      }(),
    FunctionCallExpr2(
      :final function,
      :final argument,
    ) =>
      () {
        final functionType = function.type;
        if (functionType is CustomFunctionType) {
          return functionType.customToCFunction(argument);
        }
        final functionName = generateCFromExpr(function);
        final argumentValue = generateCFromExpr(argument);
        return '$functionName($argumentValue)';
      }(),
  };
}

String _generateCFromType(MyType type, String variableName) {
  final cCode = switch (type) {
    NumberType() => 'double $variableName',
    VoidType() => 'void $variableName',
    FunctionType(:final returnType, :final argumentType) => () {
        final returnC = _generateCFromType(returnType, '');
        final argumentC = _generateCFromType(argumentType, '');
        return '$returnC (*$variableName)($argumentC)';
      }(),
  };
  return cCode;
}
