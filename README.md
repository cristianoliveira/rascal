# Rascal
A Pascal/Ruby like interpreter in Rust

# Structure
This language is based on Pascal/Ruby and follow its structure.

  * Blocks: `begin .. end`
  * Assign variables: `=`
  * Statement end: `;`
  * Return expression: `return`

Each statement requires a `;` unless the last statement. Example of runnable code:
```ruby
begin
  x = 20;
  y = 15;
  z = x + y;

  return z - 5;
end
```
Result: 30

## Future implementations
  * Boolean: evaluate booleans
  * Print: prints on stdout the given expression
  * Conditional: Evaluate conditional `if` `else` blocks
  * Loops: `while` condition repeat statement
  * Return: return in the middle of a block
  * Stable REPL: run code without exiting for sintax errors
  * Comments: ignore after `#`

## Licence
MIT
