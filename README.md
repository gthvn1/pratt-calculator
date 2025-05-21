# Pratt Calculator

- To understand pratt parsing (Vaughan Pratt, 1973) we create a simple calculator.
- To start the repl: `cargo run`
- To go a little bit further we can extend our simple calculator to:
  - support let statement to bind a character to a value
  - generate bytecode because we have an AST
- For the bytecode I'd like to have a look to the [BEAM](https://blog.stenmans.org/theBeamBook/#P-ERTS) :)
- And what about running it on Xen...

## Links

- [Top down operator precedence](https://dl.acm.org/doi/10.1145/512927.512931)
- [Introduction to Pratt parsing and its terminology](https://abarker.github.io/typped/pratt_parsing_intro.html)
- [Simple Top-Down Parsing in Python](https://11l-lang.org/archive/simple-top-down-parsing/)
- [Simple but Powerful Pratt Parsing](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html)
- [YT: This Simple Algorithm Powers Real Interpreters: Pratt Parsing](https://www.youtube.com/watch?v=0c8b7YfsBKs&t=571s)

- [BEAM](https://blog.stenmans.org/theBeamBook/#P-ERTS)
- [Getting started with Erlang](https://erlang.org/documentation/doc-5.3/doc/getting_started/getting_started.html)
- [Erlang on Xen](https://erlangonxen.org/)
- [AtomVM](https://www.atomvm.net/)

## Status

- [x] Tokenizer
  - We accept `float`, `+`, `-`, `/`, `*` and parenthesis.
- [x] parse an atom
- [x] parse one expression without priorities
- [x] evaluate expression
- [x] parse a "complex" expression with priorities
- [x] accept f64
- [x] allow parenthesis

```bash
Start of the REPL... Ctrl+D to quit
>> 1 + 2 * 3
..read 10 bytes
..generate tokens
....T: Number: 1
....T: Op: +
....T: Number: 2
....T: Op: *
....T: Number: 3
..evaluate
(+ 1 (* 2 3))
..result: 7
>> (1 + 2) * 3
..read 12 bytes
..generate tokens
....T: LeftParen
....T: Number: 1
....T: Op: +
....T: Number: 2
....T: RightParen
....T: Op: *
....T: Number: 3
..evaluate
(* (+ 1 2) 3)
..result: 9
>> 1/2
..read 4 bytes
..generate tokens
....T: Number: 1
....T: Op: /
....T: Number: 2
..evaluate
(/ 1 2)
..result: 0.5
>>
```
