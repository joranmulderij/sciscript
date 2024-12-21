import 'package:sciscript_dart/ast2.dart';
import 'package:sciscript_dart/types.dart';

class GeneratorHelper {
  int _counter = 0;
  final StringBuffer _globalBuffer = StringBuffer();

  String generateName() {
    return 'temp${_counter++}';
  }

  void injectGlobal(String code) {
    _globalBuffer.writeln(code);
  }
}

String generateCFromLines(List<Line2> lines) {
  final helper = GeneratorHelper();
  final buffer = StringBuffer();
  buffer.writeln('#include <stdio.h>');
  buffer.writeln('int main() {');
  for (final line in lines) {
    buffer.write(_generateCFromLine(line, helper));
  }
  buffer.writeln('return 0;');
  buffer.writeln('}');
  buffer.writeln(helper._globalBuffer);
  return buffer.toString();
}

String _generateCFromLine(Line2 line, GeneratorHelper helper) {
  final buffer = StringBuffer();
  final cCode = switch (line) {
    AssignmentLine2(:final identifier, :final expr) => () {
        final exprCode = generateCFromExpr(expr, helper, buffer.writeln);
        if (expr.type is VoidType) throw UnsupportedError('Void assignment');
        return '${_generateCFromType(expr.type, identifier)} = $exprCode;\n';
      }(),
    // Cast to void to suppress unused value warning
    ExprLine2(:final expr) => () {
        final exprCode = generateCFromExpr(expr, helper, buffer.writeln);
        return '(void) $exprCode;\n';
      }(),
  };
  buffer.writeln(cCode);
  return buffer.toString();
}

String generateCFromExpr(
    Expr2 expr, GeneratorHelper helper, void Function(String) injectScope) {
  final cCode = switch (expr) {
    NumberExpr2(:final value) => value.toString(),
    IdentifierExpr2(:final name) => name,
    OperatorExpr2(:final operator, :final left, :final right) => () {
        final leftValue = generateCFromExpr(left, helper, injectScope);
        final rightValue = generateCFromExpr(right, helper, injectScope);
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
          return functionType.customToCFunction(argument, helper, injectScope);
        }
        final functionName = generateCFromExpr(function, helper, injectScope);
        final argumentValue = generateCFromExpr(argument, helper, injectScope);
        return '$functionName($argumentValue)';
      }(),
    BlockExpr2(:final lines) => () {
        for (var i = 0; i < lines.length; i++) {
          final line = lines[i];
          if (i == lines.length - 1 &&
              line is ExprLine2 &&
              line.expr.type is! VoidType) {
            final variableName = helper.generateName();
            injectScope(
                '${_generateCFromType(line.expr.type, variableName)} = ${generateCFromExpr(line.expr, helper, injectScope)};');
            return variableName;
          } else {
            injectScope(_generateCFromLine(line, helper));
          }
        }
        return 'NULL';
      }(),
  };
  return cCode;
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
    AnyType() => throw UnsupportedError('AnyType not supported'),
  };
  return cCode;
}
