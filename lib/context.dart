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

  void add(String name, Variable variable) {
    scopes.last.variables[name] = variable;
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

  @override
  String toString() {
    return scopes.toString();
  }
}

class Scope {
  Map<String, Variable> variables = {};

  @override
  String toString() {
    return variables.toString();
  }
}

class Variable {
  final String id;
  final MyType type;
  final VariableMutability mutability;

  static int _idCounter = 0;

  Variable(this.type, this.mutability) : id = 'var${++_idCounter}';

  @override
  String toString() {
    return 'Variable($id, $type, $mutability)';
  }
}

enum VariableMutability {
  constant,
  mutable,
  immutable,
}
