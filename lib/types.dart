import 'package:sciscript_dart/ast2.dart';
import 'package:sciscript_dart/c_generator.dart';
import 'package:sciscript_dart/units.dart';

sealed class MyType {
  const MyType();

  bool isAssignableTo(MyType other);
}

class NumberType implements MyType {
  final num? constantValue;
  final UnitSet units;

  NumberType({this.constantValue, this.units = const UnitSet.empty()});

  @override
  bool isAssignableTo(MyType other) =>
      other is NumberType &&
      units == other.units &&
      other.constantValue == null;
}

class FunctionType implements MyType {
  final MyType returnType;
  final MyType argumentType;

  FunctionType(this.returnType, this.argumentType);

  @override
  bool isAssignableTo(MyType other) {
    if (other is! FunctionType) return false;
    if (!returnType.isAssignableTo(other.returnType)) return false;
    if (!argumentType.isAssignableTo(other.argumentType)) return false;
    return true;
  }
}

class CustomFunctionType extends FunctionType {
  final String Function(
          Expr2 arg, GeneratorHelper helper, void Function(String) injectScope)
      customToCFunction;

  CustomFunctionType(
      super.returnType, super.argumentType, this.customToCFunction);
}

class VoidType implements MyType {
  const VoidType();

  @override
  bool isAssignableTo(MyType other) => other is VoidType;
}
