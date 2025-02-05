import 'package:sciscript/context.dart';
import 'package:sciscript/libraries/library.dart';
import 'package:sciscript/types.dart';

final plotLibrary = Library(
  name: 'plot',
  scope: {
    'plot': Variable(
      FunctionType(
          VoidType(), [ArrayType(NumberType()), ArrayType(NumberType())]),
      VariableMutability.constant,
      pythonName: 'plt.plot',
    ),
    'show': Variable(
      FunctionType(VoidType(), []),
      VariableMutability.constant,
      pythonName: 'plt.show',
    ),
  },
  pythonDependencies: ['matplotlib'],
  pythonImports: ['import matplotlib.pyplot as plt'],
);
