import 'package:sciscript_dart/c_generator.dart';
import 'package:sciscript_dart/context.dart';
import 'package:sciscript_dart/types.dart';

final stdLib = <String, Variable>{
  'print': Variable(
    CustomFunctionType(VoidType(), AnyType(), (arg, helper, injectScope) {
      final type = arg.type as NumberType;
      print(type.units);
      final unitString = StringBuffer();
      for (final entry in type.units.units.entries) {
        final unitName = entry.key.name;
        final exponent = entry.value;
        if (exponent == 1) {
          unitString.write(' $unitName');
        } else {
          unitString.write(' $unitName^$exponent');
        }
      }
      return 'printf("%f$unitString\\n", ${generateCFromExpr(arg, helper, injectScope)})';
    }),
    VariableMutability.constant,
  ),
};
