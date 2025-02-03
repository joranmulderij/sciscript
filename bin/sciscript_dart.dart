import 'dart:io';

import 'package:sciscript_dart/context.dart';
import 'package:sciscript_dart/parser.dart';
import 'package:sciscript_dart/python_generator.dart';
import 'package:sciscript_dart/std_lib.dart';
import 'package:sciscript_dart/type_checker.dart';

void main(List<String> arguments) async {
  final file = File('input.sci');
  final input = file.readAsStringSync();
  final ast1 = parse(input);
  print(ast1);
  final context = Context();
  context.addAll(stdLib);
  final ast2 = typeCheckLines(ast1, context);
  final pythonCode = generatePythonFromLines(ast2);
  final pythonFile = File('./generated/output.py');
  pythonFile.writeAsStringSync(pythonCode);
  final result = await Process.run('python', ['./generated/output.py']);
  stdout.write(result.stdout);
  stdout.write(result.stderr);
}
