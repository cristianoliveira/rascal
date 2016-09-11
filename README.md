# Rascal
A Pascal interpreter in Rust

# Structure
This language is based on Pascal and follow its structure. But not the reserved
keywords.

  * Blocks: `begin .. end`
  * Assign variables: `:=`
  * Statement end: `;`

Each statement requires a `;` unless the last statement. Example of runnable code:
```ruby
begin
  x := 20;
  y := 15;
  z := x + y;

  return z - 5;
end
```
Result: 30

## Future implementations
  * Print: prints on stdout the given expression
  * Conditional: Evaluate conditional `if` `else` blocks
  * Loops: `while` condition repeat statement
  * Return: return in the middle of a block
  * Stable REPL: run code without exiting for sintax errors
  * Comments: ignore after `#`

## Licence
MIT
