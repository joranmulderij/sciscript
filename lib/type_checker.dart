import 'package:sciscript_dart/ast1.dart';
import 'package:sciscript_dart/ast2.dart';
import 'package:sciscript_dart/context.dart';
import 'package:sciscript_dart/types.dart';
import 'dart:math' as math;

import 'package:sciscript_dart/units.dart';

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
    NumberExpr1(:final value) => NumberExpr2(value.toDouble(), NumberType()),
    IdentifierExpr1(:final name) => () {
        final variable = context.lookup(name);
        if (variable == null) {
          throw Exception('Undefined variable: $name');
        }
        return IdentifierExpr2(name, variable.type);
      }(),
    OperatorExpr1(:final left, :final operator, :final right) => () {
        final left2 = typeCheckExpr(left, context);
        final right2 = typeCheckExpr(right, context);
        // TODO: Unit checking
        final expr2 = switch ((left2.type, operator, right2.type)) {
          (
            NumberType(constantValue: final constLeft),
            _,
            NumberType(constantValue: final constRight)
          )
              when constLeft != null && constRight != null =>
            () {
              final value = switch (operator) {
                Operator1.plus => constLeft + constRight,
                Operator1.minus => constLeft - constRight,
                Operator1.star => constLeft * constRight,
                Operator1.slash => constLeft / constRight,
                Operator1.doubleStar => math.pow(constLeft, constRight),
              };
              return NumberExpr2(
                  value.toDouble(), NumberType(constantValue: value));
            }(),
          (NumberType(), _, NumberType()) => OperatorExpr2(
              left2,
              switch (operator) {
                Operator1.plus => Operator2.plus,
                Operator1.minus => Operator2.minus,
                Operator1.star => Operator2.multiply,
                Operator1.slash => Operator2.divide,
                Operator1.doubleStar => Operator2.power,
              },
              right2,
              NumberType()),
          _ => throw Exception('Operator type mismatch'),
        };
        return expr2;
      }(),
    FunctionCallExpr1(:final function, :final argument) => () {
        final function2 = typeCheckExpr(function, context);
        final functionType = function2.type;
        final argument2 = typeCheckExpr(argument, context);
        final expr2 = switch (functionType) {
          FunctionType(:final argumentType, :final returnType) => () {
              if (!argument2.type.isAssignableTo(argumentType)) {
                throw Exception('Argument type mismatch');
              }
              return FunctionCallExpr2(
                function2,
                argument2,
                returnType,
              );
            }(),
          NumberType() => () {
              if (argument2.type is! NumberType) {
                throw Exception('Function call type mismatch');
              }
              final newExpr = OperatorExpr1(Operator1.star, function, argument);
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
      }()
  };
  return returnType;
}
