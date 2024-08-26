# pattern matching

## 18.1 Pattern

- pattern is a special grammar in Rust which is used to match group and simple type structures 
- Combining patterns with matching expressions and other constructs can better control the control flow of the program
- Pattern in includes follows:
  - Character value. 
  - Deconstructed arrays, enum, struct，tuple 
  - variable 
  - wildcard 
  - placeholder 

### match and Arm

- ```` rust
    match value {
        pattern => expression,
        pattern => expression,
        pattern => expression,
    }
    ````
- `match` rules:
  - try best to match the all possibilities
- `_`: placeholder, match all the values except the specified value
  - and it will not bind the value to the variable
  - it is always the last arm of match expression

### if let

- `if let` is a syntax sugar of `match` expression
- `if let` can used with `else`, include:
  - `else if`
  - `else if let`
- but `if let` didn't check all the possibilities, it only check the specified pattern
- ***example1***

### while let

- if the Pattern fits the rule, it will run.
- ***example2***

### let

- ***example3***

### function

- ***example3***

## 18.2

## 18.3 

- ***example4***