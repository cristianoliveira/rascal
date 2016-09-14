# Rascal
A functional interpreted language based on Ruby/Haskell made by Rust

# Use
```bash
rascal ./example.rl
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
We can say the same about interpreted languages and interpreters.

# Structure
This language is based on Pascal/Ruby and follow its structure.

  * Integers: `0-9`
  * Boolean: `true`, `false`
  * Imutables variables: `imut x;`
  * Assign values: `x = 0;`
  * Blocks: `begin .. end`
  * Comparison: `==`,`!=`, `>`, `<`, `and`, `or`
  * If else: `if 1==1 begin .. else .. end`
  * Loop: `while 1==1 begin .. end`

Each statement requires a `;` unless the last statement. Example of runnable code:
### Integers expressions
```ruby
begin
  imut x = 20;
  imut y = 15;
  imut z = x + y;
  z - 5
end
```
Result: 30

### Bolean expressions
```ruby
begin
  imut x = 2;
  imut y = 1;
  imut z = x != y;
  z == true
end
```
Result: true

### If Else blocks
```ruby
begin
  imut x = 2;
  mut y = 0;

  if x != 2 begin
    y = 13
  else
    y = 42
  end

  y == 42
end
```
Result: true

### Loops
```ruby
begin
  mut y = 0;

  while y < 4 begin
    y = y + 1
  end;

  y == 4
end
```
Result: true

## Future implementations
  * Strings: support for strings
  * String comparison: support for compare strings
  * Print: prints on stdout the given expression
  * Return: return in the middle of a block
  * Stable REPL: run code without exiting for sintax errors
  * Comments: ignore after `#`

## Licence
MIT
