# Rascal
A Pascal/Ruby like interpreter in Rust

# Structure
This language is based on Pascal/Ruby and follow its structure.

  * Blocks: `begin .. end`
  * Assign variables: `=`
  * Boolean: `true`, `false`
  * Comparison: `==`,`!=`, `>`, `<`, `and`, `or`
  * Statement end: `;`
  * Return expression: `return`
  * If else: `if 1==1 begin .. else .. end`
  * Loop: `while 1==1 begin .. end`

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

If else blocks
```ruby
begin
  x = 2;
  y = 0;

  if x != 2 begin
    y = 13
  else
    y = 42
  end

  y == 42
end
```
Result: true

Loops
```ruby
begin
  y = 0;

  while y < 4 begin
    y = y + 1
  end;

  return y == 4;
end
```
Result: true

## Future implementations
  * Imutable default: All vars are imutable by default `let`
  * Mutable explicit: explicity say when var is mutable `mut`
  * Strings: support for strings
  * String comparison: support for compare strings
  * Print: prints on stdout the given expression
  * Return: return in the middle of a block
  * Stable REPL: run code without exiting for sintax errors
  * Comments: ignore after `#`

## Licence
MIT
