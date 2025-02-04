import 'package:sciscript/context.dart';
import 'package:sciscript/libraries/library.dart';
import 'package:sciscript/types.dart';

final plotLibrary = Library(
  name: 'plot',
  scope: {
    'plot': Variable(
      FunctionType(VoidType(), AnyType()),
      VariableMutability.constant,
      pythonName: 'plt.plot',
    ),
  },
  pythonDependencies: ['matplotlib'],
  pythonImports: ['import matplotlib.pyplot as plt'],
);
