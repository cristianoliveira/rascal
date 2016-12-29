# Rascal [![Build Status](https://travis-ci.org/cristianoliveira/rascal.svg?branch=master)](https://travis-ci.org/cristianoliveira/rascal)
A (almost)functional interpreted language made by Rust

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
  * Mutables explicit: `var x = 1;`
  * Assign values: `x = 0;`
  * Blocks: `{ .. }`
  * Operator: `+`, `-`, `*`, `/` and `%`
  * Comparison: `==`,`!=`, `>`, `<`, `and` and `or`
  * If else: `if 1==1 { .. else .. }`
  * Loop: `while 1==1 { .. }`
  * Function: `let foo = fn [x] { x + 1 }`
  * Print: `print (1+1)`

### Example
  First project euler challenge:
```rust
var sum = 0;
var number = 0;

let is_multiple = fn [x, y] { (x % y) == 0 };

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
var y = 0;

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
var y = 0;

while y < 4 {
  y = y + 1
};

y == 4
```
Result: true

### Scope per block
```rust
var y = 0;

{
  var x = y + 1
};

x == 4
```
Error: "Variable x doesn't exist in this context"

### Functions
```rust
let foo = fn [x] { x + 1 };

foo(10)
```
Result: 11

### High Order Functions
```rust
let composed = fn [f] { f(10) };
let foo = fn [x] { x + 1 };

composed(foo)
```
Result: 11

### Closures
```rust
var state = 0;

let mutstate = fn [x] { state = state + x };
mutstate(10);
mutstate(5);

state
```
Result: 15

## Future implementations
  * Strings: support for strings
  * String comparison: support for compare strings
  * Return: return in the middle of a block
  * Stable REPL: run code without exiting for sintax errors
  * Comments: ignore after `#`

## The Architecture
  It is a simple interpreded language that walks on an AST executing the program.
  I would like to implement some bytecode, just for science sake, but for now this
  is fine. Example of an AST generated by code:

```rust
var a = 10;
var b = 1;

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
