# coconut

## setup

this code uses the crate ```grmtools```, that is a collection of Rust libraries for text parsing at compile and run times.

```
$ cargo new coconut
$ cargo -q run '2+2'
4
```

Before running the second command to build the code, there are several things to complete.

* set up ```Cargo.toml```: add the build dependencies.

* create ```src/coconut.l``` file to define the vocabulary used in the interpreter.

* create ```src/coconut.y``` file to define the parser grammar: general settings, YACC grammar rules, and Rust code

* create ```build.rs``` to compile the grammar into Rust code

* fill in the ```src/main.rs``` to pack everything together.

## reference

* [writing interpreter in Rust by Pavel Durov](https://p3ld3v.medium.com/writing-interpreter-in-rust-using-grmtools-7a6a0458b99f)

