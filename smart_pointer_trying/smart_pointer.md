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

### 15.3 Drop Trait

- Drop trait is used to customize the code run when an instance of the smart pointer goes out of scope.
  - such as file, network connection, etc.
  - any type can implement Drop trait.
- Drop trait only ordered to implement drop method.
  - `fn drop(&mut self)` it ordered mutable reference to self.
- Drop trait is in the prelude, so we don't need to import it.
- we can use `std::mem::drop` to drop the value manually.
  - rust baned to call `.drop()` method manually, so we can use `std::mem::drop` to drop the value manually.

### 15.4 Rc<T>, the Reference Counted Smart Pointer

- `Rc<T>` is a reference counting smart pointer.
- sometimes, one value can be shared by more than one owner.
- `Rc<T>` can count the number of owners and find out all the owners.

#### using situations of `Rc<T>`

- when need to use data in heap and it was only read by more part of the program but cannot be sure which one is the las
to use it.
- `Rc<T>` can only used in single-threaded situations.
- `Rc<T>` need to import `std::rc::Rc` to use it.
- `Rc::clone(&a)`: to increase the number of owners.
- `Rc::strong_count(&a)`: to get the number of owners.
  - `Rc::weak_count(&a)`: to get the number of weak owners.

#### example of `Rc<T>`

- two list share another list's ownership:
  - b->b1->a1->a2->nil
  - a->a1->a2->nil
  - c->c1->a1->a2->nil

- `Rc::clone` will increase the number of owners. but it won't operate deep copy of data.
- `.clone()` is a shallow copy, it just copies the pointer to the data.

- `Rc<T>` worked by immutable borrowing the data.you can share the data to many part in program which read-only.

### 15.5 RefCell and the Interior Mutability Pattern

#### interior mutability

- `RefCell<T>` it allowed to keep immutable reference and can change the data.
  - datasturcture used unsafe code to cross the borrow rule and changeable of rust.

#### RefCell<T>

- `RefCell<T>` can only used in single-threaded situations.
- different of `Rc<T>`, `RefCell<T>` is the only one owner of the data.
  - borrow rule:
    - in any time, you can only have one mutable reference, or many of the immutable reference.
    - reference is always available.
- `RefCell<T>` & `Box<T>`
  - `Box<T>` checked when compile time or it will be error
  - `RefCell<T>` checked when runtime or it will be panic
- use `RefCell<T>` to record borrow message when it is borrowed in runtime.
  - two methods: `borrow` and `borrow_mut`
    - `borrow` will return `Ref<T>` type, it implements `Deref` trait.
    - `borrow_mut` will return `RefMut<T>` type, it implements `Deref` trait.
  - `RefCell<T>` will record how many active `Ref<T>` and `RefMut<T>`.
  - `RefCell<T>` will panic when it is borrowed by `RefMut<T>` and `Ref<T>` at the same time.
    - `.borrow()` plus one of immutable reference
    - any of `Ref<T>` leave the scope, the number of `Ref<T>` will decrease.
    - `.borrow_mut()` plus one of mutable reference
    - any of `RefMut<T>` leave the scope, the number of `RefMut<T>` will decrease.
    - immutable ref number is no limit.
    - mutable ref number is zero to one.

#### others

- `Cell<T>`: work by copy to visit data.
- `Mutex<T>`: workes in multi-threaded situations

### 15.6 Reference Cycles Can Leak Memory

- `Rc<T>` and `RefCell<T>` can create reference cycles.
- if reference cycles, the data will never be dropped.

#### how to solve reference cycles

- use `Weak<T>` to break the reference cycles.
- `Rc::clone` rises the strong_count for `Rc<T>`, `Rc<T>` will be dropped when strong_count is zero.
- `Rc<T>` create Weak Reference by `Rc::downgrade(&a)`.
- `Weak<T>` can be used to create `Rc<T>` by `Weak::upgrade(&a)`.
  - it returns `Weak<T>`
  - Rc::downgrade will plus one of weak_count.
- `Rc<T>` use weak_count to check how many weak reference to it.
- `Rc<T>` will be dropped whether weak_count is zero when `Rc<T>` need to be dropped.

#### Strong & Weak

- *strong reference* is about shared ownership of `Rc<T>`
- use *weak reference* will not increase the number of owners.
- `Weak<T>` is used to break the reference cycles.
  - when *Strong reference* is zero, the data will be dropped and the *Weak reference* will be dropped too.
- before used `Weak<T>`, make sure the data is still available.
  - call upgrade method in `Weak<T>` returns `Option<Rc<T>>`.