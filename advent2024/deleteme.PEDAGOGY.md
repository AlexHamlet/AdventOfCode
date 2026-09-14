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

# leaked Rust source code

```rust
enum Option<T> {
    None,
    Some(T),
}

#[unused_must_use]
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

# Not leaked Rust source code

```rust
enum InputEvent {
    KeyboardEvent {
        char_code: Option<char>,
        scancode: Option<i32>,
        is_pressed: bool,
    },
    MouseButtonEvent { mouse_button: i32, x: i32, y: i32, is_pressed: bool },
    MouseMoveEvent { x: i32, y: i32 },
}
```

```cs
enum InputEventType {
    KEYBOARD_EVENT, MOUSE_BUTTON_EVENT, MOUSE_MOVE_EVENT
};

class InputEvent {
    InputEventType type;
    // ONLY VALID FOR KEYBOARD EVENTS
    Char char_code;
    Integer scancode;
    // ONLY VALID FOR MOUSE EVENTS
    int x, y;
    // VALID SPECIFICALLY FOR MOUSE BUTTON EVENTS
    int mouse_button;
    // valid for everything except mouse move events
    bool is_pressed;
};
```

```c
enum InputEventType {
    KEYBOARD_EVENT, MOUSE_BUTTON_EVENT, MOUSE_MOVE_EVENT
};

union InputEvent {
    InputEventType type;
    KeyboardInputEvent keyboard;
    MouseButtonEvent mouse_btn;
    MouseMoveEvent mouse_move;
}

struct KeyboardInputEvent {
    InputEventType type;
    char char_code; // 0 = no character
    int scancode; // 0 = no key (???)
    bool is_pressed;
}

struct MouseButtonEvent {
    InputEventType type;
    int x, y;
    int button;
    bool is_pressed;
}

struct MouseMoveEvent {
    InputEventType type;
    int x, y;
}
```

# how to make a game installer that deletes the hard disk

```sh
#!/bin/sh

set -e # THIS IS IMPORTANT! this makes it so the script dies if anything fails

echo "Now installing Embers of the Shattered System v2.0."

echo "Uninstalling previous version..."
cd /usr/games/shattered_system
rm -rf .

echo "Installing new version..."
tar xzf ~/Downloads/shattered_system_2.0.tar.gz

echo "All finished :)"
```

# The Carousel of Traits

Traits you should consider `#[derive(...)]`ing on every new struct/enum you make, and when you would/wouldn't do it (in the canonical order):

- `Debug`: to print a programmer-friendly version of me for debugging purposes
- `Clone`: if it is *possible* to make a copy
- `Copy`: if it is *always cheap* and *never incorrect* to make a copy (e.g. a 42 is a 42, but different instances of `File` or `Vec<T>` are different)
- `PartialEq`: if you can ask the question "are these two values of your type equal?" (`==` operator)
- `Eq`: if there is no value that is equal to nothing ("no NaNs club")
- `PartialOrd`: if you can ask the question "which of these two values of your type is smaller?" (`<` and `>`)
- `Ord`: if there is no value that is not ordered ("no NaNs club" #2)
- `Hash`: if it would ever make sense to use this type as the key in a `HashMap`

# conversions

```rust
let foo = 10i16;
let x = foo as i32; // primitive conversion, everything's fine
let y = i32::from(foo); // infallible conversion (`From`), everything's fine
let z = i32::try_from(foo).unwrap(); // fallible conversion (`TryFrom`) which will not fail

let foo = 100_000i32;
let x = foo as i16; // primitive conversion, WILL CHOP OFF BITS
let y = i16::from(foo); // infallible conversion NOT AVAILABLE
let z = i16::try_from(foo).unwrap(); // fallible conversion which fails because the value is out of range

```

# "pointiness"

translation: move it

```
x' = x + t_x
y' = y + t_y
z' = z + t_z
```

scaling it: resize it

```
x' = x * s_x
y' = y * s_y
z' = z * s_z
```

rotating it: rotating it (let's pretend only rotation around the Z axis exists)

```
c = cos(theta)
s = sin(theta)
x' = x * c + y * -s
y' = x * s + y * c
z' = z
```

if only there were one notation that would let us describe all three!

```
x' = x * a + y * b + z * c + d
y' = x * e + y * f + z * g + h
z' = x * i + y * j + z * k + l
```

As far as computer graphics are concerned, that's what matrices are for.

```
┌         ┐
│ a b c d │
│ e f g h │
│ i j k l │
└         ┘
```

is just another way of writing the above.

(matrix rant)

Real matrices don't just have "and then you add a thing" at the end. In order for the real matrix math to work:


```
x' = x * a + y * b + z * c + w * d
y' = x * e + y * f + z * g + w * h
z' = x * i + y * j + z * k + w * l
w' = x * m + y * n + z * o + w * p
┌         ┐
│ a b c d │
│ e f g h │
│ i j k l │
│ m n o p │ (typically 0 0 0 1)
└         ┘
```

This is called an augmented vector, and Solra calls the `w` coordinate "pointiness".

# how to swap two values in C

```c
// some algebraic rules regarding XOR:
// a ^ b ^ b = a
// a ^ b ^ a = b (same rule)
void swap_ints(int* a, int* b) {
    // a := a, b := b
    *a ^= *b;
    // a := mix(a,b), b := b
    *b ^= *a;
    // a := mix(a,b), b := a
    *a ^= *b;
    // a := b, b := a
}
```

# how to make Solra sad in C

```c
// is this callable at compile time? NO
int my_constant_function();
// is this callable at compile time? GUESS IS YES, answer is NO
const int my_constant_function();
// is this callable at compile time? GUESS IS YES, answer is YES
constexpr int my_constant_function();
// is this callable at compile time? GUESS IS NO, answer is SOLRA DOESN'T KNOW
extern constexpr int my_constant_function();
// is this callable at compile time? GUESS IS NO, answer is NO
int my_constant_function() __attribute__((pure));
// bonus brainfreeze: the first two are the same
// bonus question: which of those declare a function that must be defined in
// this file, and which ones declare a function that may be defined in another
// file?
// your answer: all of them can be defined in another file
// CORRECT!
// this one would have to be defined in this file:
static int my_constant_function();
// (one of many meanings of static)
```

```c
// the cursed inline keyword that doesn't mean inline
static inline int sum(int a, int b) { return a+b; }
```
