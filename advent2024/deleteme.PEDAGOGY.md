# Ownership

```rust
let x = 24;
let mut y = 42;
let mut foo = &x;
println!("{}", *foo); // prints 24
foo = &y;
println!("{}", *foo); // 42
*foo = 44; // error: (& is not &mut, we can only look)
y = 44; // error: you can't modify something while there are & references to it
drop(foo); // make foo go away
y = 44; // NOW you can do this
```

In that code, `x` owns a value (24) and `y` owns a value (42, later 44). `foo` only ever owns references. It doesn't get any ownership of the values beyond them. (That's what references are for; to allow access without conferring ownership.)

```rust
let mut x = 24;
let foo = &mut x;
// foo is NOT mutable
foo = &mut something_else; // error
// but it contains a mutable reference
*foo = 42; // perfectly fine
x = 44; // error: you can't modify something while someone else has &mut on it
// (borrow checker strikes!)
drop(foo)
x = 44; // now you can
```

```js
const myList = [1, 2, 3]; // like Java's final
myList[2] = 5; // Monty Python mode
myList = [4, 5, 6]; // ERROR: can't change a const variable!
myList[2] = 3; // but this doesn't count as changing
```

```rust
let mut vec = Vec::new(); // Vec owns some locations, the number of which can change at runtime
vec.push(1);
vec.push(2);
vec.push(5);
let foo: &i32 = &vec[2]; // variable "vec" owns an object of type "Vec" which owns the location we're referencing here (which has a value of 5 in it)
let ref_slice: &[i32] = &vec[1..3]; // variable "vec" owns an object of type "Vec" which owns the two locations we're referencing here (current values 2, 5)

let array: [i32; 3] = [1, 2, 5]; // an object of type "array of 3 i32s"
// how big is it? it is 3 i32s big
// how many i32s are in it? 3
// An array owns a fixed number of locations

let ref_slice: &[i32] = &array[0..2]; // a slice does NOT own the locations
let slice: [i32] = array[0..2]; // THIS IS AN ERROR
// Read [i32] as "slice of i32s", and the number of i32s in the slice is only
// known at runtime. The compiler needs to know how much space to allocate for
// the variable at compile time. So you can't put "a slice" in a variable
// directly.

let mut byte_vec: Vec<u8>; // some number of u8s that can grow and shrink at runtime (owned)
let mut byte_slice: &[u8]; // a reference to some number of consecutive u8s inside somebody else, they own it, we don't
let mut byte_array: [u8; 42]; // 42 u8s. they can mutate or whatever but there will always be 42 of them.

let mut string: String; // some amount of text that can grow and shrink at runtime (owned)
let mut str_slice: &str; // a reference to some consecutive text owned by somebody else
let mut str_array: !; // (there's no corresponding thing here)
```

## In which Solra is slightly mean/weird to Java

```java
// (assuming I got the array constructor syntax right—I am so old that the
// array constructor syntax is new to me :|)
string[] myArray = new string[]{"Hello", "World"};
myArray = new string[]{"Goodbye", "Cruel", "World"};
myArray = new string[]{"oops"};
myArray = null; // Solra sniping at nullability here for no reason
```

# The Three Kinds of Self

- `self`: consumed, not mutable
- `&self`: look but don't touch
- `&mut self`: look and touch
- `mut self`: consumed, mutable (exactly the same as `self`, except that the variable named `self` also gets to be mutable)

# a brief aside about iterators

Normal iterator lifecycle:

- Create (e.g. `.iter()`, `.into_iter()`, `.lines()`)
- (zero or more) Operate (e.g. `.map()`, `.filter()`)
- Consume (e.g. `.sum()`, `.collect()`)

# The Rust naming convention

- Non-primitive types, traits, enum variants (which are types for reasons we won't get into): `CapitalCamelCase`
- Constants, statics: `SCREAMING_SNAKE_CASE`
- **All other names*: `snake_case`

# Resetting the "days since Solra complained about C" counter to 0

```c
int x = 5;
printf("%i\n", --x--); // 4
printf("%i\n", x); // 3
printf("%i\n", --++x++--); //3 if it's valid which it might not be
x = 5;
printf("%i\n", x-- -- --); // 5
// the point is it's confusing
// (Solra confused himself but GTB was fine)
x = 5;
printf("%i %i %i %i %i\n", x, x--, x, x++, x); // 5 5 5 5 5
// possible outputs depending on ABI:
// 5 5 4 4 5 (args pushed left-to-right)
// 5 6 6 5 5 (args pushed right-to-left)
```

# example of strange case in day 2 part 2

1 9 3 4 5

# some rules of life

- If you are going to write `&Vec<T>`, **always** write `&[T]` instead.
- If you are going to write `&String`, **always** write `&str` instead.
