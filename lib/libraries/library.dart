import 'package:sciscript/context.dart';

class Library {
  final String name;
  final Map<String, Variable> scope;
  final List<String> pythonDependencies;
  final List<String> pythonImports;

  Library({
    required this.name,
    required this.scope,
    this.pythonDependencies = const [],
    this.pythonImports = const [],
  });
}
