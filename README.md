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

## add AST

There are several changes as follows:

* add ```src/ast.rs```

* modify ```src/cocunut.y``` so that the parser won't execute the operations there. Instead it returns AST nodes as a vector of type ```Vector<ast::Node>```. So there is no parse-time evaluation now.

* add ```mod ast;``` in ```src/main.rs```

```
$ cargo -q run '2+2'
[Add { lhs: Number { value: 2 }, rhs: Number { value: 2 } }]
$ cargo -q run '2+2*2'
[Add { lhs: Number { value: 2 }, rhs: Mul { lhs: Number { value: 2 }, rhs: Number { value: 2 } } }]
```

* add ast-time evaluation: add two functions ```eval()``` and ```eval_exp()``` in ```src/main.rs```

```
$ cargo -q run '2+2'
4
$ cargo -q run '2+2*3' 
8
$ cargo -q run '(2+3)*2'
10
```

* restructure the ```src/main.rs``` to have a new function ```from_str()``` and add simple test

```
$ cargo -q run '2+2'
4
$ cargo test
running 1 test
test eval_expressions ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## reference

* [writing interpreter in Rust by Pavel Durov](https://p3ld3v.medium.com/writing-interpreter-in-rust-using-grmtools-7a6a0458b99f)

* [grmtools quickstart](https://softdevteam.github.io/grmtools/master/book/quickstart.html)

