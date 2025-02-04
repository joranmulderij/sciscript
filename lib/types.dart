import 'package:sciscript/ast2.dart';
import 'package:sciscript/c_generator.dart';
import 'package:sciscript/units.dart';

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

class ArrayType extends MyType {
  final MyType elementType;
  final int? length;

  ArrayType(this.elementType, this.length);

  @override
  bool _isAssignableTo(MyType other) {
    if (other is! ArrayType) return false;
    if (length != null && other.length != null && length != other.length) {
      return false;
    }
    if (length == null && other.length != null) return false;
    if (!elementType._isAssignableTo(other.elementType)) return false;
    return true;
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

class VoidType extends MyType {
  const VoidType();

  @override
  bool _isAssignableTo(MyType other) => other is VoidType;
}
