# smart pointer

## 15.0

### something about

- pointer: a variable that stores the memory address of another variable
- The most common pointer in Rust is' reference'
- reference
  - immutable reference: `&T`
  - mutable reference: `&mut T`
  - reference is a pointer that points to a value
  - without other costs.

### smart pointer

- action like pointer
- additional metadata and capabilities
- like a data structure
- they always own the data they point. Just like they own the data, and then they share the data to other owners,when
there are no other owners, data will be dropped.

#### ***reference counting*** smart pointer type

- `Rc<T>`: reference counting smart pointer
- it can count the number of owners,makes this data keep by more than one owner.
- when it has no owner, it will be dropped automatically.

#### example of a smart pointer

- `String` and `Vec<T>`
- they all have memory area, allowed user to do operations.
- they have metadata
- Provide additional features or guarantees (String ensures that its data is legally encoded in UTF-8)

#### how to use smart pointer

- it always use 'struct' to define a smart pointer.
  - `Deref` and `Drop` traits are implemented for smart pointers.
- `Deref trait`: allows an instance of the smart pointer struct to be treated as a reference to the data.
- `Drop trait`: allows you to customize the code that is run when an instance of the smart pointer goes out of scope.

#### what will be introduced
- common smart pointer types
  - `Box<T>`: for allocating values on the heap
  - `Rc<T>`: a reference counting type that enables multiple ownership
  - `Ref<T>` and `RefMut<T>`: accessed through RefCell\<T>, a type that enforces the borrowing rules at runtime instead of compile time
- otherwise
  - `interior mutability pattern`: a design pattern in Rust that allows you to mutate data even when there are immutable references to that data
  - `reference cycles`: a scenario in which two `Rc<T>` values refer to each other, creating memory leaks

## 15.1 Box<T> Points to Data on the Heap

### Box<T> type

- `Box<T>` allows saving data on the heap rather than the stack.
- what in stack is pointer that points to the heap data.
- no more costs than other pointers.
- without other extra function
- implemented `Deref` trait and `Drop` trait

### when use Box<T>

- the size of a type is unknown at compile time and you want to use a value of that type in a context that requires an exact size
- when you have huge data and you want to transfer ownership but ensure the data won't be copied when you do so.
- when used some value, you just care about implemented needed trait, but don't care about the type of the value.

### example of Box<T>

- use `Box<T>`  to save data on the heap
- use `Box<T>` to access the recursion
  - when rust compiler, they need to know the size of the type at compile time.
  - but the size of the recursion is unknown at compile time.
- `Box<T>` can solve this problem.,it like Cons List in Functional language.

#### about Cons List

- in rust, we always use `Vec<T>` instead of Cons List.
- how can rust knows enum's size?
  - it will go through all the variants of the enum, and find the largest one.

### 15.2 Deref Trait(1)

- if Deref trait is implemented, we can use `*` to dereference the value.
- by implementing Deref trait, we can use the value as a reference.
- `*` is a syntactic sugar for `*(value.deref())`

`Box<T>` be let as a tuple struct with elements of type `T`.

#### Deref Coercion

- Deref Coercion is a feature that allows you to treat any type that implements Deref as a reference to the inner type.
- if `T` implement Deref Trait:
  - Deref Coercion will convert `&T` to `&U` if `T` implements Deref to `U`
- when some type reference send to func or method but its type and definition are different to needed:
  - Deref Coercion will automatically work.
  - IDE will do aseries of call to deref to change it to the type we needed.

#### deref and mutable

- `DerefMut` trait is used to dereference mutable references.
- when type and trait in these three situations, rust will operate deref coercion:
  - From `&T` to `&U` when `T: Deref<Target=U>`
  - From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
  - From `&mut T` to `&U` when `T: Deref<Target=U>`