# Rascal
A Pascal/Ruby like interpreter in Rust

# Structure
This language is based on Pascal/Ruby and follow its structure.

  * Blocks: `begin .. end`
  * Assign variables: `=`
  * Boolean: `true`, `false`
  * Comparison: `==`,`!=`, `and`, `or`
  * Statement end: `;`
  * Return expression: `return`

Each statement requires a `;` unless the last statement. Example of runnable code:
Integers
```ruby
begin
  x = 20;
  y = 15;
  z = x + y;

  return z - 5;
end
```
Result: 30

Bolean
```ruby
begin
  x = 2;
  y = 1;
  z = x != y;

  return z == true;
end
```
Result: true

Loops
```ruby
begin
  y = 0;

  while y != 4 begin
    y = y + 1
  end;

  return y == 0;
end
```
Result: false

## Future implementations
  * Boolean: evaluate booleans - DONE
  * Print: prints on stdout the given expression
  * Conditional: Evaluate conditional `if` `else` blocks
  * Loops: `while` condition repeat statement
  * Return: return in the middle of a block
  * Stable REPL: run code without exiting for sintax errors
  * Comments: ignore after `#`

## Licence
MIT
