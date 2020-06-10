<div style="text-align:center">
    <img src="banner.png" width="400" style="border-radius:1rem;"/>
</div>

----

Zypo is a new language focused on portability and developer needs ✨

## Objectives

- Dynamic typing
- Binary portability
- Static compilation
- Markdown-friendly (all compiler logs should be in markdown)
- Memory safe compiler (programs written are not memory safe!)

## The Syntax

2 simple dummy functions:

```kotlin
fun hello(other_int) {
    if(other_int == 5) {
        var x = 24;

        while(x / other_int != 2) {
            --snip--
        }
    }

    var result = "hello";
}

fun mul_x(first, second) {
    return first * second == 6;
}
```

[Fibonacci](https://en.wikipedia.org/wiki/Fibonacci_number) sequence:

```kotlin
fun fibonacci(stop_iteration) {
    if (stop_iteration == (0 or 1)) {
        return 0; -- return just 0 as user input is incorrect.
    }

    return fibonacci(stop_iteration - 1) + fibonacci(stop_iteration - 2);
}
```

You can find more code examples in the `examples/` directory in the same path as this README!

*Note: we use kotlin for markdown highlighting as Zypo highlighting is not supported just yet.. 🤞*

## Project structure

In the sublevels are descriptions on each made part of this compiler.

### **`/`** - Main repository

- Contains general docs on the rest of zypo including setup and running
- Contains the 2 core Zypo modules, `zypo-lib` (the main compiler library) and `zypo` (the cli)
- Named "Zypo"/"The compiler" or if referencing in docs "`zypo-rs`" (always lowercase in codeblock)

#### **`/zypo-lib/`** - Compiler library

- Named "The compiler library" or "`zypo-lib`" (always lowercase in codeblock)
- Contains the main guts of the compiler and a simple API to connect outside code to multiple stages of the library

#### **`/zypo-cli/`** - Main CLI

- Named "The CLI" or "`zypo-cli`" (always lowercase in codeblock)
- The CLI the majority of people use to interact with Zypo
- On releasing the binary, the name is shortened to just `./zypo` for linux or `./zypo.exe` for windows

#### **`/examples/`** - Zypo examples

- Contains some examples for `.zy`/Zypo code that are used in this README
