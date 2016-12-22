# Rascal [![Build Status](https://travis-ci.org/cristianoliveira/rascal.svg?branch=master)](https://travis-ci.org/cristianoliveira/rascal)
A functional interpreted language made by Rust

## Features of functional languages
According with [Haskel wiki](https://wiki.haskell.org/Functional_programming) the features of a functional language are:

- [x] Functions
- [x] Higher-order functions
- [ ] Purity
- [x] Immutable data
- [x] Referential transparency
- [ ] Lazy evaluation
- [x] Recursion

## Use
```bash
rascal ./example.rl
```

## Repl
```bash
rascal
>>
```

# Install and run
```bash
git clone https://github.com/cristianoliveira/rascal.git
cd rascal
cargo build && cargo install
rascal ./example.rl
```

# Motivation?
> “If you don’t know how compilers work, then you don’t know how computers work.
> If you’re not 100% sure whether you know how compilers work,
  > then you don’t know how they work.” — Steve Yegge

# Structure
  * Integers: `0-9`
  * Boolean: `true`, `false`
  * Imutables by default: `let x = 1;`
  * Mutables explicit: `let mut x = 1;`
  * Assign values: `x = 0;`
  * Blocks: `{ .. }`
  * Operator: `+`, `-`, `*`, `/` and `%`
  * Comparison: `==`,`!=`, `>`, `<`, `and` and `or`
  * If else: `if 1==1 { .. else .. }`
  * Loop: `while 1==1 { .. }`
  * Function: `fn foo = [x] { x + 1 }`
  * Print: `print (1+1)`

### Example
  First project euler challenge:
```rust
let mut sum = 0;
let mut number = 0;

fn is_multiple = [x, y] { (x % y) == 0 };

while number < 10 {
  if is_multiple(number, 5) or is_multiple(number, 3) { sum = sum + number };
  number = number + 1
};

sum
```

Each statement requires a `;` unless the last statement. Example of runnable code:

### Integers expressions
```rust
let x = 20;
let y = 15;
let z = x + y;
z - 5
```
Result: 30

### Bolean expressions
```rust
let x = 2;
let y = 1;
let z = x != y;
z == true
```
Result: true

### If Else blocks
```rust
let x = 2;
let mut y = 0;

if x != 2 {
  y = 13
    else
      y = 42
};

y == 42
```
Result: true

### Loops
```rust
let mut y = 0;

while y < 4 {
  y = y + 1
};

y == 4
```
Result: true

### Scope per block
```rust
let mut y = 0;

{
  let mut x = y + 1
};

x == 4
```
Error: "Variable x doesn't exists in this context"

### Functions
```rust
fn foo = [x] { x + 1 };

foo(10)
```
Result: 11

### High Order Functions
```rust
fn composed = [f] { f(10) };
fn foo = [x] { x + 1 };

composed(foo)
```
Result: 11

## Future implementations
  * Strings: support for strings
  * String comparison: support for compare strings
  * Print: prints on stdout the given expression
  * Return: return in the middle of a block
  * Stable REPL: run code without exiting for sintax errors
  * Comments: ignore after `#`
  * Lambda: support for short `x = |y| y + 1;`

## The Architecture
  It is a simple interpreded language that walks on an AST executing the program.
  I would like to implement some bytecode, just for science sake, but for now this
  is fine. Example of an AST generated by code:

```rust
let mut a = 10;
let mut b = 1;

while b != 0 {
  if a > b {
    a = a - b;
  else
    b = b - a;
  }
};

return a
```
![sintax](http://i.stack.imgur.com/JDAbW.png)

## Licence
MIT
