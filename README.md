# Rascal
A Pascal interpreter in Rust

# Structure
This language is based on Pascal and follow its structure. But not the reserved
keywords.

  * Blocks: `BEGIN .. END`
  * Assign variables: `:=`
  * Statement end: `;`

Each statement requires a `;` unless the last statement. Example of runnable code:
```Pascal
BEGIN
  x := 20;
  y := 15;
  z := x + y
END
```

## Future implementations
  * Print: prints on stdout the given expression
  * Conditional: Evaluate conditional `IF` `ELSE` blocks
  * Return: return in the middle of a block
  * Stable REPL: run code without exiting for sintax errors

## Licence
MIT
