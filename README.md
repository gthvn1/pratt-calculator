# Pratt Calculator

To understand pratt parsing (Vaughan Pratt, 1973) we create a simple calculator.
- [Top down operator precedence](https://dl.acm.org/doi/10.1145/512927.512931)
- [Simple Top-Down Parsing in Python](https://11l-lang.org/archive/simple-top-down-parsing/)
- [Simple but Powerful Pratt Parsing](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html)

---

### 🛠 High-level Steps to implement parser and evaluator using Pratt parsing

#### Define an AST (Abstract Syntax Tree)

* We need a data structure to represent parsed expressions.
* Typically:
  * A node that is either:
    * An integer literal (`i64`).
    * A binary operation combining two expressions and an operator.

Later we will evaluate the tree recursively.

#### Create a Parser struct over the token stream

* Create a `Parser` that wraps our list of tokens.
* Keep a **peekable iterator** to inspect the current and next tokens.

#### Understand Pratt parsing core idea

* Pratt parsing works by associating precedence and associativity with operators.
* It works via a recursive method (`parse_expression`) that takes a *current precedence level*
and parses everything at that precedence or higher.
  * When the precedence drops, we return the parsed sub-expression.
* It naturally handles operator precedence and associativity in a single loop.

#### Setup operator precedences

* Define a **function or table** that maps each operator to a numeric precedence level.
  * `*` and `/` → Higher precedence (e.g. 10).
  * `+` and `-` → Lower precedence (e.g. 5).

#### Write the `parse_expression(min_precedence)` method

* Core of Pratt parsing:

  1. Start by parsing the **left-hand side (lhs)**:
     * For our case, it can only be a number.
  2. While the current token is an operator with precedence ≥ `min_precedence`:
     * Get the operator.
     * Get its precedence.
     * Advance to next token.
     * Recursively parse the **right-hand side (rhs)** with **higher precedence (e.g. precedence + 1)**.
     * Combine `lhs`, operator, and `rhs` into a new node.
  3. Return the `lhs`.

#### Finish when no more tokens or when lower precedence detected

* When the token stream ends or the next operator has lower precedence than our current `min_precedence`,
stop parsing and return the current expression node.

#### Evaluate the parsed AST recursively

* Once we have the AST, implement an evaluator:
  * For an integer literal → return it.
  * For a binary operation → evaluate left and right recursively and apply the operator.

### 🔄 Flow example

1. `parse_expression(0)`:

   * Parses `123`.
   * Sees `+`, precedence 5 ≥ 0.

     * Parses right side with `min_precedence = 6`.
     * Sees `32`.
     * Returns `32`.
   * Returns `Add(123, 32)`.

2. For `123 + 2 * 3`:

   * Parses `123`.
   * Sees `+`, precedence 5.

     * Parses right side with `min_precedence = 6`.
     * Parses `2`.
     * Sees `*`, precedence 10 ≥ 6.

       * Parses right side with `min_precedence = 11`.
       * Parses `3`.
       * Returns `Mult(2, 3)`.
     * Returns `Mult(2, 3)`.
   * Returns `Add(123, Mult(2, 3))`.

---

### 📌 Summary checklist

| Step | Task                                                                 |
| ---- | -------------------------------------------------------------------- |
| 1    | Define `Expr` enum (`Integer`, `BinaryOp(Box<Expr>, Op, Box<Expr>)`) |
| 2    | Create `Parser` that wraps tokens                                    |
| 3    | Define operator precedence table                                     |
| 4    | Write `parse_expression(min_prec)` method                            |
| 5    | Implement the recursive loop for Pratt parsing                       |
| 6    | Evaluate the AST recursively                                         |

