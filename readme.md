# Rust

![](assets/20240823_062954_image.png)

![](assets/20240823_062231_image.png)

* Rust Tooling


![](assets/20240823_062353_image.png)

## Comments

- use `//` for one line comment.
- use `/* */` for multiline comment.

## Mutability - Immutability

* **Immutable** : that can't be changed once initialized and `mutable` is opposite of it.
* Every variable in rust is immutable by default until we make it mutable.

  ```rust
  let x=10; // immutable
  let mut x=10; // mutable
  ```

## Const vs Let

* **let**: help us to define `mutable/immutable` variables like above.
* **const**:
  * `mut` is not allowed.And must be annotated with type(eg. i32 or u32) while declaring it.
  * Always immutable and must be initialized while declaring it to literals or expression which result constant value in compile time.
  * unlike `let` it can be declared in global scope.And they are printed in code(compiler will assign the value of const to all places where it is used so no memory allocated for it at runtime).

    ```rust
    let x; x=10; // can be initialized later still immutable.
    const x=10; // must be initialized while declaring it.
    ```

## Shadowing

- Rust allows the same variable names in nest scopes which shadows the outer scope name.

  ```rust
  let x =5;
  {
    let x=10;
     println!(x); // output: 10
  }
  println!(x); // output: 5
  ```

## Data types

- Since rust is `statically` typed language it must know it's data type at compile time. Most of the time compiler can infer the type based on the value we assigned to them.
- **Scalar type:** scalar type repsent single value. Integers,floating-point numbers, Booleans, characters.

  - **Integers**

    ![1724379847695](image/readme/1724379847695.png)

    - Here usize and isize types depends on the architecture of the computer our programm is running.
  - **Floating =>** let x: f32/64;
  - **Boolean =>** let x:bool = true/false;
  - **Character =>** let x:char = 'z' ; // In rust character is four bytes in size represent unicode scalar value.
- **Note:** rust don't do explicit type conversion for the literals so for that we have to do it.
  `let x = 5 as u64;`
- **Comound Types**

  ```rust
  // Tuples
  let tup: (i32,f64,u8) = (500,6.4,1);
  let (x,y,z) = tup; // destrucutre syntex
  // to access value : 1. either destructure them like above. 2. Or we can access tup.0, tup.1, tupe.2

  // Array
  let a = [1,2,3,4];
  let b:[i32; 5] = [1,2,3,4,5]; // [type; size]
  let c = [0; 5]; // [initial value; array_size]
  // to access values : a[0], [1]. but unlike c/cpp when we access the index which is not part of array our program will panic at runtime and exit.
  ```


## Statement and Expression

- Anything ended with `;` is called `statement` and `expression` is part of statement. `Expression` evaluates to a resultant value. While `statement` performs some action but do not returns the value.
- Eg of `statements` are : 1. anything ended with semicolon, 2. Function definition.
- Eg. of `expression` are : 1. calling a function is a expression, 2. calling a macro is epxression. 3. A new scope block created with curly braceses is an expression.
  **Note :**  `Rust` is expression based language. Where if we don't put `;` in the end then it will be treated as return value we'll see more on that later.

  ```rust
  let y = {
  	// this is epxression.
  	let x =3;
  	x+1
  	}  // after this value of y = x+1;
  ```


## Function

```rust

/*
* fn name(/* parameters */) -> return_type { value_to_return }
*/

fn five() -> i32 {5}

fn plus_one(x: i32) -> i32 {x+1}

```


## Control Flow

- Rust supports : `if, else if and else `  & `match : as switch case in other language`
- Loops
  - Infinite loop using : `loop { // statement }  `
  - It supports : `while loop` , `for loop : but iterates over ranges like(python) or collection, and works with iterators`.
- Loop labels : allows breaking out of nested loops.

```rust
fn main() {

    'outer: for i in 0..=5 /* this means 5 is also included */ { 
        for j in 0..5 {
            if j == 2 {
                break 'outer; // Breaks out of the outer loop
            }
            println!("i: {}, j: {}", i, j);
        }
    }
}

```
