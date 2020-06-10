<div style="text-align:center">
    <h1>Zypo</h1>
    <img src="logo.png" width="100" style="border-radius:1rem;"/>
</div>

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

*Note: we use kotlin for markdown highlighting as Zypo highlighting is not supported just yet.. 🤞*

## Project structure

Below is the file and module structure for this repository and Zypo as a whole:

- **`/`** -- Main repository, named "Zypo" or specifically "`zypo-rs`" (always lowercase in codeblock)
  - **`zypo-lib/`** -- Compiler library, named "`zypo-lib`" (always lowercase in codeblock)
  - **`zypo/`** -- Main compiler CLI, named "`zypo`" (always lowercase in codeblock)
