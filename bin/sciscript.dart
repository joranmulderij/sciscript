import 'dart:io';

import 'package:sciscript/context.dart';
import 'package:sciscript/libraries/plot.dart';
import 'package:sciscript/parser.dart';
import 'package:sciscript/python_generator.dart';
import 'package:sciscript/libraries/stdlib.dart';
import 'package:sciscript/type_checker.dart';

void main(List<String> arguments) async {
  final file = File('input.sci');
  final input = file.readAsStringSync();
  final ast1 = parse(input);
  print(ast1);
  final context = Context();
  final List<String> pythonImports = [];
  context.loadLibrary(stdLib);
  context.loadLibrary(plotLibrary);
  pythonImports.addAll(stdLib.pythonImports);
  pythonImports.addAll(plotLibrary.pythonImports);
  final ast2 = typeCheckLines(ast1, context);
  final pythonCode = StringBuffer();
  for (final pythonImport in pythonImports) {
    pythonCode.writeln(pythonImport);
  }
  final pythonLinesCode = generatePythonFromLines(ast2);
  pythonCode.write(pythonLinesCode);

  final pythonFile = File('./generated/output.py');
  pythonFile.writeAsStringSync(pythonCode.toString());
  final result = await Process.run('python', ['./generated/output.py']);
  stdout.write(result.stdout);
  stdout.write(result.stderr);
}
