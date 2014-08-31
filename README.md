## Type-directed [TDD](http://en.wikipedia.org/wiki/Test-driven_development) in [Rust](http://www.rust-lang.org/)

[![Build Status](https://travis-ci.org/FranklinChen/type-directed-tdd-rust.png)](https://travis-ci.org/FranklinChen/type-directed-tdd-rust)

### Info

This repository contains material for my presentation ["Type-Directed TDD in Rust"](http://www.meetup.com/Pittsburgh-Code-Supply/events/183483622/) given at CMU on Monday, July 21, 2014, for the [Pittsburgh Code and Supply](http://www.codeandsupply.co/) meetup.

I ported this presentation from an [earlier presentation](https://github.com/FranklinChen/talk-on-type-directed-tdd-using-fizzbuzz) I gave at [Pittsburgh TechFest](http://pghtechfest.com/) 2014 that originally used the language [Scala](http://scala-lang.org/) for exposition (with some [Swift](http://developer.apple.com/swift/) as an addendum). I welcomed the opportunity to create a [Rust](http://www.rust-lang.org/) version of the presentation in order to show that the main ideas and work flow are *not* language-dependent, but apply to any sufficiently modern *statically [typed](http://en.wikipedia.org/wiki/Type_system)* programming language with a good set of testing libraries.

### Property-based testing

In particular, the type-directed TDD work flow leans heavily on [property-based testing](http://en.wikipedia.org/wiki/QuickCheck), a way of concisely specifying properties that result in automatically generated tests. The Rust code uses the [QuickCheck](https://github.com/BurntSushi/quickcheck) property-based testing framework.

### Code, article, and slides

#### Most recent code

I've made the `master` branch reflect the cleanest working version I have on hand of any code, independent of the presentation materials, which I am no longer modifying. So far, I have added:

- Principled `Result`-based validation

#### As actually presented on 2014-07-21

The branch [`meetup`](https://github.com/FranklinChen/type-directed-tdd-rust/tree/meetup) will always contain the version of the materials as of the *actual presentation* (with only minor typos fixed).

Documents:

- [article](https://github.com/FranklinChen/type-directed-tdd-rust/blob/meetup/presentation/article.pdf)
- [slides](https://github.com/FranklinChen/type-directed-tdd-rust/blob/meetup/presentation/presentation.pdf)

#### Important improvements after the talk!

Thanks to [Alex Crichton](https://github.com/alexcrichton), I created a new branch [`using-str`](https://github.com/FranklinChen/type-directed-tdd-rust/tree/using-str) that replaces my clumsy overuse of `String` with the type `&'a str` instead. The new code is much cleaner and does fewer memory allocations as well, and makes clear where new strings are allocated with `to_string()`. You can see the diffs in [this commit](https://github.com/FranklinChen/type-directed-tdd-rust/commit/6db41f90b522dc3d814dc10fbfa014d1a13e4e05).

Thanks to feedback on the [Reddit discussion](http://www.reddit.com/r/rust/comments/2bcwua/type_driven_tdd_in_rust/), I learned of another optimization: using `String` `append()`, which cleverly reuses an existing buffer and saves on memory allocations and gives linear concatenation.

I've made the `without-validation` branch reflect the cleanest working version I have on hand of code corresponding to the presentation materials. In particular, the code does not contain `Result`-based validation, which was outside the scope of the presentation.

Documents:

- [article](https://github.com/FranklinChen/type-directed-tdd-rust/blob/master/presentation/article.pdf)
- [slides](https://github.com/FranklinChen/type-directed-tdd-rust/blob/master/presentation/presentation.pdf)

### Caveats

Rust is still not at version 1.0 yet, and many things I wanted to do were not yet properly supported as of the date of the presentation. The type system is still being worked out. Best practices are still being worked out for how to make decisions about memory placement for all data.

The lack of the kind of closure support I wanted meant that I did not implement a *compiler* but instead an *interpreter*. I'm leaving the transformation into a compiler for later. It does bring up questions of memory ownership and sharing.

I will update the code as various language constructs and libraries become available, and as I understand Rust better. Rust feels very much like programming in C, ironically, except that memory layout and management is exposed in the type system.

### System requirements

You need to install Rust with [Cargo](http://crates.io/):

```
$ curl https://static.rust-lang.org/rustup.sh | sudo bash
```

