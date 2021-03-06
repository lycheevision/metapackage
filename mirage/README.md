# mirage

A crate for rapid prototyping and analysis of computer vision algorithms.

## Concept

This project borrows the idea of ["high level"][high-level-post] libraries and applies it to mīrō.
My short take on the topic is that high-level libraries make iterative prototyping using statically 
typed programming languages, feasible. On the other hand, high-level libraries are generally not 
meant for production use.

The idea of [interactive programming][interactive-wiki] is at the very heart of __mirage__ and is 
something I am very excited about. Interactive programming is a programming paradigm that uses
hot-swapping, so the user can make changes in a program while it is running. In essence, the 
programming activity becomes part of the program flow itself.

## Installing Rust

I recommend you install [rustup][rustup] and then run the following command to use 
the Rust nightly version:

```sh
$ rustup install nightly # or rustup update nightly
```

Although you can run a command from the nightly toolchain without switching to nightly:

```sh
$ rustup run nightly [command]
```

I recommend that you switch to nightly, globally:

```sh
$ rustup default nightly
```

## Platforms

* Currently only supports macOS (please refer to issue#26)

[high-level-post]: http://blog.piston.rs/2014/12/27/the-road-to-high-level-libraries/
[interactive-wiki]: https://en.wikipedia.org/wiki/Interactive_programming
[rustup]: https://www.rustup.rs