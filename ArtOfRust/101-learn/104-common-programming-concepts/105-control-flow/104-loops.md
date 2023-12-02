# Repeating Code with `loop`

The `loop` keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.


```rust
fn main() {
    loop {
        println!("again!");
    }
}
```

```bash
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```

- Press control+c to break the loop.

- We also use `continue` for to tells the program to skip over any remaining code in this iteration of the loop and go to the next iteration.

## Returning Values from Loops

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

- Before the `loop`, we declare a variable named `counter` and initialize it to 0.
- Then we declare a variable named `result` to hold the value returned from the loop.
- On every iteration of the loop, we add `1` to the counter variable, and then check whether the `counter` is equal to `10`.
- When it is, we use the `break` keyword with the value `counter * 2`. 
- After the loop, we use a semicolon to end the statement that assigns the value to `result`. Finally, we print the value in `result`, which in this case is `20`.


## Loop Labels to Disambiguate Between Multiple Loops

- If you have loops within loops, `break` and `continue` apply to the innermost loop at that point.
- You can specify the *loop label* on a loop that you can then use with `break` or `continue` to specify that those keywords apply t othe labeled loop instead of the innermost loop.


```rust
pub fn main() {
    let mut count: i32 = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            
            remaining -= 1;
        }
        count += 1;
    }

    println!("End count = {count}");
}
```

## Conditional Loops with while

- While the condition is `true`, the loops runs.
- When the condition ceases to be `true`, the program calls `break`, stopping the loop.

- `while` to loop the program three times, counting down each time, and them, after the loop print a message and exit.

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```


## Looping Through a Collection with `for`

- You can choose to use the `while` construct to loop over the elements of collection, such as an array.

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

- here, the code counts up through the elements in the array. It starts at index `0`, and them loops untils it reaches the final index in the array (that is, when `index < 5` is not longer `true`), the code would panic.
- It's also slow, because the compiler adds runtime code to perform the conditional check of whether the index is within the bounds of the array on array on every iteration through the loop.

```bash
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/loops`
the value is: 10
the value is: 20
the value is: 30
the value is: 40
the value is: 50
```

You can use a `for` loop and execute some code for each item in a collection.


```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

- When we run this code, we'll see the same output as previous.


Here's what the countdown would loop like using a `for` loop and another method we've not yet taked about, `rev`, to reverse the range:

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```