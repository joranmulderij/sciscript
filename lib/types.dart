import 'package:sciscript/units.dart';

sealed class MyType {
  const MyType();

  bool isAssignableTo(MyType other, {bool ignoreConstant = false}) =>
      other is AnyType || _isAssignableTo(other, ignoreConstant);

  bool _isAssignableTo(MyType other, bool ignoreConstant);
}

class AnyType extends MyType {
  const AnyType();

  @override
  bool _isAssignableTo(MyType other, bool ignoreConstant) => other is AnyType;
}

class NumberType extends MyType {
  final num? constantValue;
  final UnitSet units;

  NumberType({this.constantValue, this.units = const UnitSet.empty()});

  @override
  bool _isAssignableTo(MyType other, ignoreConstant) =>
      other is NumberType &&
      units == other.units &&
      (other.constantValue == null || ignoreConstant);

  @override
  String toString() {
    return 'NumberType($constantValue, $units)';
  }
}

class ArrayType extends MyType {
  final MyType elementType;
  final int? length;

  ArrayType(this.elementType, [this.length]);

  @override
  bool _isAssignableTo(MyType other, bool ignoreConstant) {
    if (other is! ArrayType) return false;
    if (length != null && other.length != null && length != other.length) {
      return false;
    }
    if (length == null && other.length != null) return false;
    if (!elementType._isAssignableTo(other.elementType, false)) return false;
    return true;
  }
}

class FunctionType extends MyType {
  final MyType returnType;
  final List<MyType> argumentTypes;
  // final bool acceptsManyArguments;

  FunctionType(this.returnType, this.argumentTypes);

  @override
  bool _isAssignableTo(MyType other, bool ignoreConstant) {
    if (other is! FunctionType) return false;
    if (!returnType._isAssignableTo(other.returnType, false)) return false;
    if (argumentTypes.length != other.argumentTypes.length) return false;
    for (var i = 0; i < argumentTypes.length; i++) {
      if (!argumentTypes[i]._isAssignableTo(other.argumentTypes[i], false)) {
        return false;
      }
    }
    // if (acceptsManyArguments && !other.acceptsManyArguments) return false;
    return true;
  }
}

class VoidType extends MyType {
  const VoidType();

  @override
  bool _isAssignableTo(MyType other, bool ignoreConstant) => other is VoidType;
}
