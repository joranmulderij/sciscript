import 'package:sciscript/ast2.dart';
import 'package:sciscript/types.dart';

class GeneratorHelper {
  int _counter = 0;

  String generateName() {
    return 'temp${_counter++}';
  }
}

String generatePythonFromLines(List<Line2> lines) {
  final helper = GeneratorHelper();
  final buffer = StringBuffer();
  for (final line in lines) {
    buffer.write(_generatePythonFromLine(line, helper));
  }
  return buffer.toString();
}

String _generatePythonFromLine(Line2 line, GeneratorHelper helper) {
  final buffer = StringBuffer();
  final cCode = switch (line) {
    AssignmentLine2(:final identifier, :final expr) => () {
        final exprCode = generateCFromExpr(expr, helper, buffer.write);
        if (expr.type is VoidType) throw UnsupportedError('Void assignment');
        return '$identifier: ${_generatePythonFromType(expr.type)} = $exprCode\n';
      }(),
    // Cast to void to suppress unused value warning
    ExprLine2(:final expr) => () {
        final exprCode = generateCFromExpr(expr, helper, buffer.write);
        if (exprCode.isNotEmpty) return '$exprCode\n';
        return '';
      }(),
  };
  buffer.write(cCode);
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
      :final arguments,
    ) =>
      () {
        final functionName = generateCFromExpr(function, helper, injectScope);
        final argumentValues = arguments
            .map((argument) => generateCFromExpr(argument, helper, injectScope))
            .join(', ');
        return '$functionName($argumentValues)';
      }(),
    BlockExpr2(:final lines) => () {
        for (var i = 0; i < lines.length; i++) {
          final line = lines[i];
          if (i == lines.length - 1 &&
              line is ExprLine2 &&
              line.expr.type is! VoidType) {
            final variableName = helper.generateName();
            injectScope(
                '$variableName: ${_generatePythonFromType(line.expr.type)} = ${generateCFromExpr(line.expr, helper, injectScope)}\n');
            return variableName;
          } else {
            injectScope(_generatePythonFromLine(line, helper));
          }
        }
        return '';
      }(),
    ArrayExpr2(:final elements) => () {
        final elementValues = elements
            .map((element) => generateCFromExpr(element, helper, injectScope))
            .join(', ');
        return 'np.array([$elementValues])';
      }(),
  };
  return cCode;
}

String _generatePythonFromType(MyType type) {
  final cCode = switch (type) {
    NumberType() => 'num',
    VoidType() => 'NoneType',
    FunctionType(:final returnType, :final argumentTypes) => () {
        final returnTypeCode = _generatePythonFromType(returnType);
        final argumentTypeCodes = argumentTypes
            .map((argumentType) => _generatePythonFromType(argumentType))
            .join(', ');
        return 'Callable[[$argumentTypeCodes], $returnTypeCode]';
      }(),
    ArrayType() => 'np.ndarray',
    AnyType() => throw UnsupportedError('AnyType not supported'),
  };
  return cCode;
}
