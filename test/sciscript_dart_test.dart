import 'package:petitparser/reflection.dart';
import 'package:sciscript/parser.dart';
import 'package:test/test.dart';

void main() {
  test('detect common problems', () {
    final definition = ExpressionDefinition();
    final parser = definition.build();
    expect(linter(parser), isEmpty);
  });
}
