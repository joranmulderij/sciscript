import 'package:sciscript_dart/types.dart';

class Context {
  List<Scope> scopes = [Scope()];

  Variable? lookup(String name) {
    for (var scope in scopes.reversed) {
      if (scope.variables.containsKey(name)) {
        return scope.variables[name];
      }
    }
    return null;
  }

  void add(String name, Variable type) {
    scopes.last.variables[name] = type;
  }

  void addAll(Map<String, Variable> variables) {
    scopes.last.variables.addAll(variables);
  }

  void pushScope() {
    scopes.add(Scope());
  }

  void popScope() {
    scopes.removeLast();
  }
}

class Scope {
  Map<String, Variable> variables = {};
}

class Variable {
  final String id;
  final MyType type;
  final VariableMutability mutability;

  static int _idCounter = 0;

  Variable(this.type, this.mutability) : id = 'var${++_idCounter}';
}

enum VariableMutability {
  constant,
  mutable,
  immutable,
}
