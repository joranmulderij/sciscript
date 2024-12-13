import 'package:sciscript_dart/c_generator.dart';
import 'package:sciscript_dart/context.dart';
import 'package:sciscript_dart/types.dart';

final stdLib = <String, Variable>{
  'print': Variable(
    CustomFunctionType(VoidType(), NumberType(), (arg) {
      return 'printf("%f\\n", ${generateCFromExpr(arg)})';
    }),
    VariableMutability.constant,
  ),
};
