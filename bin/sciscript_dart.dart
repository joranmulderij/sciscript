import 'dart:io';

import 'package:sciscript_dart/c_generator.dart';
import 'package:sciscript_dart/context.dart';
import 'package:sciscript_dart/parser.dart';
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
  final cCode = generateCFromLines(ast2);
  final cFile = File('./c_files/output.c');
  cFile.writeAsStringSync(cCode);
  final result = await Process.run(
      'clang', ['./c_files/output.c', '-o', './c_files/output']);
  stdout.write(result.stdout);
  stdout.write(result.stderr);
  final output = await Process.run('./c_files/output', []);
  stdout.write(output.stdout);
  stdout.write(output.stderr);
}
