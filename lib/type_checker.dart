import 'package:sciscript/ast1.dart';
import 'package:sciscript/ast2.dart';
import 'package:sciscript/context.dart';
import 'package:sciscript/types.dart';
import 'dart:math' as math;

import 'package:sciscript/units.dart';

List<Line2> typeCheckLines(List<Line1> lines, Context context) {
  return lines
      .map((line) => typeCheckLine(line, context))
      .whereType<Line2>()
      .toList();
}

Line2? typeCheckLine(Line1 line, Context context) {
  return switch (line) {
    AssignmentLine1(:final identifier, :final expr) => () {
        final valueType = typeCheckExpr(expr, context);
        final variable = Variable(valueType.type, VariableMutability.immutable);
        context.add(identifier, variable);
        return AssignmentLine2(
          identifier,
          valueType,
        );
      }(),
    ExprLine1(:final expr) => ExprLine2(typeCheckExpr(expr, context)),
    UnitDefLine1(:final name) => () {
        final numberType = NumberType(
          constantValue: 1,
          units: UnitSet({Unit(name): 1}),
        );
        final variable = Variable(numberType, VariableMutability.constant);
        context.add(name, variable);
        return AssignmentLine2(name, NumberExpr2(1, numberType));
      }(),
    FunDefLine1() => throw UnimplementedError(),
  };
}

Expr2 typeCheckExpr(Expr1 expr, Context context) {
  final returnType = switch (expr) {
    NumberExpr1(:final value) =>
      NumberExpr2(value.toDouble(), NumberType(constantValue: value)),
    IdentifierExpr1(:final name) => () {
        final variable = context.lookup(name);
        if (variable == null) {
          throw Exception('Undefined variable: $name');
        }
        final usedName = variable.pythonName ?? name;
        return IdentifierExpr2(usedName, variable.type);
      }(),
    OperatorExpr1(:final left, :final operator, :final right) => () {
        final left2 = typeCheckExpr(left, context);
        final right2 = typeCheckExpr(right, context);
        // TODO: Unit checking
        final expr2 = switch ((left2.type, operator, right2.type)) {
          (
            NumberType(constantValue: final constLeft, units: final unitsLeft),
            _,
            NumberType(
              constantValue: final constRight,
              units: final unitsRight
            ),
          )
              when constLeft != null && constRight != null =>
            () {
              final value = switch (operator) {
                Operator1.plus => constLeft + constRight,
                Operator1.minus => constLeft - constRight,
                Operator1.star => constLeft * constRight,
                Operator1.slash => constLeft / constRight,
                Operator1.doubleStar ||
                Operator1.circumflex =>
                  math.pow(constLeft, constRight),
              };
              final units = switch (operator) {
                Operator1.plus => unitsLeft == unitsRight
                    ? unitsLeft
                    : throw Exception('Unit mismatch'),
                Operator1.minus => unitsLeft == unitsRight
                    ? unitsLeft
                    : throw Exception('Unit mismatch'),
                Operator1.star => unitsLeft + unitsRight,
                Operator1.slash => unitsLeft - unitsRight,
                Operator1.doubleStar ||
                Operator1.circumflex =>
                  constRight is int
                      ? unitsLeft * constRight
                      : throw Exception('Exponent must be a constant integer'),
              };
              return NumberExpr2(
                value.toDouble(),
                NumberType(constantValue: value, units: units),
              );
            }(),
          (
            NumberType(units: final unitsLeft),
            _,
            NumberType(
              units: final unitsRight,
              constantValue: final constRight
            ),
          ) =>
            () {
              final operator2 = switch (operator) {
                Operator1.plus => Operator2.plus,
                Operator1.minus => Operator2.minus,
                Operator1.star => Operator2.multiply,
                Operator1.slash => Operator2.divide,
                Operator1.doubleStar || Operator1.circumflex => Operator2.power,
              };
              final units = switch (operator) {
                Operator1.plus => unitsLeft == unitsRight
                    ? unitsLeft
                    : throw Exception('Unit mismatch'),
                Operator1.minus => unitsLeft == unitsRight
                    ? unitsLeft
                    : throw Exception('Unit mismatch'),
                Operator1.star => unitsLeft + unitsRight,
                Operator1.slash => unitsLeft - unitsRight,
                Operator1.doubleStar || Operator1.circumflex => () {
                    if (!unitsRight.isEmpty()) {
                      throw Exception('Exponent cannot have units');
                    }
                    if (constRight is int) {
                      return unitsLeft * constRight;
                    }
                    throw Exception('Exponent must be a constant integer');
                  }(),
              };
              return OperatorExpr2(
                  left2, operator2, right2, NumberType(units: units));
            }(),
          _ => throw Exception('Operator type mismatch'),
        };
        return expr2;
      }(),
    FunctionCallExpr1(:final function, :final arguments) => () {
        final function2 = typeCheckExpr(function, context);
        final functionType = function2.type;
        final argumentExprs2 = arguments
            .map((argument) => typeCheckExpr(argument, context))
            .toList();
        final expr2 = switch (functionType) {
          FunctionType(:final argumentTypes, :final returnType) => () {
              for (var i = 0; i < argumentExprs2.length; i++) {
                final argumentExpr2 = argumentExprs2[i];
                final argumentType = argumentTypes[i];
                if (!argumentExpr2.type.isAssignableTo(argumentType)) {
                  throw Exception('Argument type mismatch');
                }
              }
              return FunctionCallExpr2(
                function2,
                argumentExprs2,
                returnType,
              );
            }(),
          NumberType() => () {
              if (argumentExprs2.length != 1) {
                throw Exception('Function call type mismatch');
              }
              if (argumentExprs2[0].type is! NumberType) {
                throw Exception('Function call type mismatch');
              }
              final newExpr =
                  OperatorExpr1(Operator1.star, function, arguments[0]);
              return typeCheckExpr(newExpr, context);
            }(),
          _ => throw Exception('Function call type mismatch: $functionType'),
        };
        return expr2;
      }(),
    BlockExpr1(:final lines) => () {
        context.pushScope();
        final lines2 = lines
            .map((line) => typeCheckLine(line, context))
            .whereType<Line2>()
            .toList();
        context.popScope();
        final returnType = switch (lines2.lastOrNull) {
          ExprLine2(:final expr) => expr.type,
          _ => VoidType(),
        };
        return BlockExpr2(lines2, returnType);
      }(),
    ArrayExpr1(:final elements) => () {
        final elements2 =
            elements.map((element) => typeCheckExpr(element, context)).toList();
        var elementType = elements2.firstOrNull?.type ?? AnyType();
        if (elements2.any((element) =>
            !element.type.isAssignableTo(elementType, ignoreConstant: true))) {
          elementType = AnyType();
        }
        return ArrayExpr2(elements2, ArrayType(elementType, elements2.length));
      }(),
  };
  return returnType;
}
