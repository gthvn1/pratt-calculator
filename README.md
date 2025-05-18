# Pratt Calculator

- To understand pratt parsing (Vaughan Pratt, 1973) we create a simple calculator.
- To start the repl: `cargo run`

## Links
- [Top down operator precedence](https://dl.acm.org/doi/10.1145/512927.512931)
- [Introduction to Pratt parsing and its terminology](https://abarker.github.io/typped/pratt_parsing_intro.html)
- [Simple Top-Down Parsing in Python](https://11l-lang.org/archive/simple-top-down-parsing/)
- [Simple but Powerful Pratt Parsing](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html)
- [YT: This Simple Algorithm Powers Real Interpreters: Pratt Parsing](https://www.youtube.com/watch?v=0c8b7YfsBKs&t=571s)

## Status

- [x] Tokenizer
  - We accept `integer`, `+`, `-`, `/` and `*`.
- [x] parse an atom
- [x] parse one expression without priorities
- [x] evaluate expression
- [x] parse a "complex" expression with priorities
- [ ] accept f64
- [ ] allow parenthesis

```bash
Start of the REPL... Ctrl+D to quit
>> 2 + 3 * 4
..read 10 bytes
..generate tokens
....S: Integer: 2
....S: Op: +
....S: Integer: 3
....S: Op: *
....S: Integer: 4
..evaluate
(+ 2 (* 3 4))
..result: 14
>> 2 * 3 + 4
..read 10 bytes
..generate tokens
....S: Integer: 2
....S: Op: *
....S: Integer: 3
....S: Op: +
....S: Integer: 4
..evaluate
(+ (* 2 3) 4)
..result: 10
>>
```
