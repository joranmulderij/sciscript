import 'package:sciscript/context.dart';
import 'package:sciscript/libraries/library.dart';
import 'package:sciscript/types.dart';

final stdLib = Library(
  name: 'stdlib',
  scope: {
    'print': Variable(
      FunctionType(VoidType(), [AnyType()]),
      VariableMutability.constant,
      pythonName: 'myprint',
    ),
  },
  pythonDependencies: ['numpy'],
  pythonImports: [
    'from sciscript_python_lib import *',
    'from collections.abc import Callable',
    'import typing as tp',
    'import numpy as np',
    'import numpy.typing as npt',
  ],
);
