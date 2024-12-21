import 'package:sciscript_dart/ast2.dart';
import 'package:sciscript_dart/c_generator.dart';
import 'package:sciscript_dart/units.dart';

sealed class MyType {
  const MyType();

  bool isAssignableTo(MyType other) =>
      other is AnyType || _isAssignableTo(other);

  bool _isAssignableTo(MyType other);
}

class AnyType extends MyType {
  const AnyType();

  @override
  bool _isAssignableTo(MyType other) => other is AnyType;
}

class NumberType extends MyType {
  final num? constantValue;
  final UnitSet units;

  NumberType({this.constantValue, this.units = const UnitSet.empty()});

  @override
  bool _isAssignableTo(MyType other) =>
      other is NumberType &&
      units == other.units &&
      other.constantValue == null;

  @override
  String toString() {
    return 'NumberType($constantValue, $units)';
  }
}

class FunctionType extends MyType {
  final MyType returnType;
  final MyType argumentType;

  FunctionType(this.returnType, this.argumentType);

  @override
  bool _isAssignableTo(MyType other) {
    if (other is! FunctionType) return false;
    if (!returnType._isAssignableTo(other.returnType)) return false;
    if (!argumentType._isAssignableTo(other.argumentType)) return false;
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

class VoidType extends MyType {
  const VoidType();

  @override
  bool _isAssignableTo(MyType other) => other is VoidType;
}
