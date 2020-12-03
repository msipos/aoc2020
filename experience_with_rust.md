# Some notes on Rust while playing with AOC2020

## Program 01 

### Error handling is painful

I suppose I really got spoiled with Python/Java/C++, but I find exceptions to be quite handy.  There is so many times I need to write `.unwrap()` which makes me feel uncomfortable or `?` which I suppose only works when there is an error being returned. I tried to make a function that returns a `Result` but it would appear if I cannot be bothered to define an error type, then I mus treturn something funky like `Box<dyn Error>`. That seems a little ugly and could have been shortened somehow.

### References are also painful

Again, a measure of how spoiled I am, but I have forgotten the need to think about whether the variable is a reference or not. I suppose in a language like Rust, it is important to know in order to avoid potentially expensive moves.

### What are types

Maybe I need a better IDE, but it is hard for me to say what type is a variable. I am broadly uncertain a lot of times whether I am dealing with a reference or a value someplace and have to keep checking docs.

## Program 02

### Playing with Cargo

It's quite cool, and the regex module was easy to get started with.

`lazy_static` was a little ugly. For some reason it seems like if you are loading a macro, you have to do:

```
#[macro_use] extern crate lazy_static;
```

Would have been nice to have unified syntax.

## Program 03

Tried to play with `ndarray` package. Managed to create a 2D array. Wasn't too impressed with the indexing and the slicing syntax.

Slightly confused, when I index into the array I do `[1, 2, 3]` but shapes in `.zeros()` call are done via `(1, 2, 3)`. Something to figure out.