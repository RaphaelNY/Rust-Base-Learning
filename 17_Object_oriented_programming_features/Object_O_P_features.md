# 17.0 Object oriented programming features

## 17.1 Introduction

> Object orientation typically includes the following features: named objects, encapsulation, inheritance

## Objects contain data and behavior

- Object-oriented programs are composed of objects.
- Objects encapsulate data and operate on these data processes, which are often referred to as methods or operations

### encapsulation

- Encapsulation is the process of combining data and functions into a single unit called a class.
- `pub`
- ***example1***

### inheritance

- Inheritance: allows an object to inherit the data and behavior of another object without the need to repeatedly define related code
- in rust, haven't inheritance.
- why use inheritance:
  - code reusability
    - Rust: use `trait` to share code
  - multiple inheritance
    - Rust: use `trait` and `T` (bounded parametric)

##  17.2 use trait to save value of different types

### demand:
- create a GUI tool:
  - it will go through some list of items, call its draw func to draw it on the screen
    - such as *Button, Text, Image, Textfield, etc.*
- in oriented programming:
  - Define a *Component* superclass that defines the draw method
  - Define Button, TextField, etc.Inheritance and Component Class

### Define a trait for public action

- Rust avoids referring to struct or enum as objects because they are separate from the impl block.
- The trait object is somewhat similar to objects in other languages
  - They combined data and behavior to some extent
- trait different from traditional Objects:
  -we cannot add value to trait
- Trait is specifically used to abstract certain shared behaviors, and it is not as universal as objects in other languages
- ***example2***

### The trait executes dynamic dispatch

- When constraining trait scope generics, the Rust compiler performs homomorphism
  - The compiler will generate non generic implementations of corresponding functions and methods for each type we use to replace generic type parameters

### Trait object must promise that the trait is object-safe

- only when the trait is object-safe, can we use it as a trait object
- rust use a series of rules to check whether it safe or not,
  - method's return type isn't `self`
  - method didn't include any `T`
- ***example3***

## 17.3 Implementing object-oriented design patterns

### state pattern

- state pattern is a Implementing object-oriented design patterns:
  -  The internal state possessed by a value is expressed by several state objects, and the behavior of the value changes with the change of the internal state
- using state pattern means:
  - When business requirements change, there is no need to modify the code that holds the status value, or use the code that holds this value
  - Just need to update the code inside the state object to change its rules or add some new state objects
- ***example4***

### make status and action as type

- rust type checker can help us to check the correctness of the code to stop user use un-useful status.
- ***example5***

