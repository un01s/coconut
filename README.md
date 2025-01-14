# coconut

## 1. setup

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

## 2. add AST

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

## 3. bytecode and stack-based VM

AST (abstract syntax tree) is a structural representation of the program, while bytecode is just a set of instructions executed by the VM. Here are some benefits from bytecode:

* portability: bytecode can be generated once and executed on multiple platforms

* performance and optimizations: bytecode can serve as an intermediate phase for JIT compilation

* separation of concerns: AST separates the parsing and evaluation phases. Bytecode separates the code structural mode (how the program is organized) from the execution model (how the program is run).

There are some changes in the code. Add ```src/instruction.rs```, then some code change in ```src/main.rs```. Look for the function ```ast_to_bytecode()```.

```
$ cargo test
   Compiling coconut v0.1.0 (/Users/wb/io/sw/coconut)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.08s
     Running unittests src/main.rs (target/debug/deps/coconut-72f2ff74d49428fe)

running 1 test
test eval_expressions ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

$ cargo -q run '2+2'
4
```

## reference

* [writing interpreter in Rust by Pavel Durov](https://p3ld3v.medium.com/writing-interpreter-in-rust-using-grmtools-7a6a0458b99f)

* [bytecode and stack-based VM: part 3](https://betterprogramming.pub/writing-an-interpreter-in-rust-bytecode-and-stack-based-vm-part-3-943af4acf9e0)

* [grmtools quickstart](https://softdevteam.github.io/grmtools/master/book/quickstart.html)

