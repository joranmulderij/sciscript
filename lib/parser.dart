import 'package:petitparser/petitparser.dart';
import 'package:sciscript_dart/ast1.dart';

List<Line1> parse(String input) {
  final parser = ExpressionDefinition().build();
  final result = parser.parse(input);
  if (result is Success) {
    return result.value;
  } else {
    throw Exception('${result.message} at ${result.position}');
  }
}

class ExpressionDefinition extends GrammarDefinition {
  @override
  Parser<List<Line1>> start() => ref0(lines).end();

  Parser<List<Line1>> lines() => (newline().starString() &
          ref0(line).plusSeparated(ref0(lineSeparator)) &
          newline().starString())
      .map((lines) => lines[1].elements);

  // Lines
  Parser<Line1> line() => [
        ref0(unitDefLine),
        ref0(assignmentLine),
        ref0(exprLine),
      ].toChoiceParser();
  Parser<UnitDefLine1> unitDefLine() =>
      (ref0(unit) & ref0(identifier)).map((values) {
        return UnitDefLine1(values[1]);
      });
  Parser<AssignmentLine1> assignmentLine() =>
      ([ref0(let), ref0(var_)].toChoiceParser().optional() &
              ref0(identifier) &
              char('=').myTrim() &
              ref0(expr))
          .map((values) {
        final keyword = values[0] as String?;
        final type = switch (keyword) {
          'let' => AssignmentType1.let,
          'var' => AssignmentType1.var_,
          null => AssignmentType1.reassign,
          _ => throw UnimplementedError(),
        };
        return AssignmentLine1(values[1], values[3], type);
      });
  Parser<ExprLine1> exprLine() => ref0(expr).map((expr) => ExprLine1(expr));

  // Expressions
  Parser<Expr1> expr() {
    final builder = ExpressionBuilder<Expr1>();
    builder.primitive(ref0(identifierExpr));
    builder.primitive(ref0(numberExpr));
    builder.primitive(ref0(blockExpr));
    builder.group().wrapper(char('(').myTrim(), char(')').myTrim(),
        (left, value, right) {
      return value;
    });

    builder.group().left([ref0(doubleStar), ref0(circumflex)].toChoiceParser(),
        (left, op, right) {
      return OperatorExpr1(op, left, right);
    });

    builder.group().left([ref0(star), ref0(slash)].toChoiceParser(),
        (left, op, right) {
      return OperatorExpr1(op, left, right);
    });

    builder.group().left([ref0(plus), ref0(minus)].toChoiceParser(),
        (left, op, right) {
      return OperatorExpr1(op, left, right);
    });

    // char('@').not() is a workaround. Its a parser that matches nothing.
    builder.group().left(char('@').not(), (left, op, right) {
      return FunctionCallExpr1(left, right);
    });

    return builder.build();
  }

  Parser<NumberExpr1> numberExpr() => digit()
      .plusString()
      .map((value) => NumberExpr1(num.parse(value)))
      .myTrim();
  Parser<IdentifierExpr1> identifierExpr() =>
      ref0(identifier).map((value) => IdentifierExpr1(value));
  Parser<BlockExpr1> blockExpr() =>
      (char('{').myTrim() & ref0(lines) & char('}').myTrim())
          .map((values) => BlockExpr1(values[1]));

  // Tokens
  Parser<String> identifier() =>
      (letter() & word().starString()).flatten().myTrim();
  Parser<Operator1> plus() => char('+').map((_) => Operator1.plus).myTrim();
  Parser<Operator1> minus() => char('-').map((_) => Operator1.minus).myTrim();
  Parser<Operator1> star() => char('*').map((_) => Operator1.star).myTrim();
  Parser<Operator1> doubleStar() =>
      string('**').map((_) => Operator1.star).myTrim();
  // ^
  Parser<Operator1> circumflex() =>
      char('^').map((_) => Operator1.circumflex).myTrim();
  Parser<Operator1> slash() => char('/').map((_) => Operator1.slash).myTrim();
  Parser<String> lineSeparator() =>
      [newline(), char(';')].toChoiceParser().plusString();
  Parser<String> unit() => string('unit').myTrim();
  Parser<String> let() => string('let').myTrim();
  Parser<String> var_() => string('var').myTrim();
}

extension _CustomTrim<R> on Parser<R> {
  Parser<R> myTrim() => TrimmingParser<R>(this, myWhitespace(), myWhitespace());
  Parser<String> myWhitespace() => [char(' '), char('\t')].toChoiceParser();
}
