## Type-directed [TDD](http://en.wikipedia.org/wiki/Test-driven_development) in [Rust]((http://www.rust-lang.org/)

[![Build Status](https://travis-ci.org/FranklinChen/type-directed-tdd-rust.png)](https://travis-ci.org/FranklinChen/type-directed-tdd-rust)

This repository contains material for my presentation ["Type-Directed TDD in Rust"](http://www.meetup.com/Pittsburgh-Code-Supply/events/183483622/) given at CMU on Monday, July 21, 2014, for the [Pittsburgh Code and Supply](http://www.codeandsupply.co/) meetup.

I ported this presentation from an [earlier presentation](https://github.com/FranklinChen/talk-on-type-directed-tdd-using-fizzbuzz) I gave at [Pittsburgh TechFest](http://pghtechfest.com/) 2014 that originally used the language [Scala](http://scala-lang.org/) for exposition (with some [Swift](http://developer.apple.com/swift/) as an addendum). I welcomed the opportunity to create a [Rust](http://www.rust-lang.org/) version of the presentation in order to show that the main ideas and work flow are *not* language-dependent, but apply to any sufficiently modern *statically [typed](http://en.wikipedia.org/wiki/Type_system)* programming language with a good set of testing libraries.

### Property-based testing

In particular, the type-directed TDD work flow leans heavily on [property-based testing](http://en.wikipedia.org/wiki/QuickCheck), a way of concisely specifying properties that result in automatically generated tests. The Rust code uses the [QuickCheck](https://github.com/BurntSushi/quickcheck) property-based testing framework.

### Caveats

Rust is still not at version 1.0 yet, and many things I wanted to do were not yet properly supported as of the date of the presentation. The type system is still being worked out. Best practices are still being worked out for how to make decisions about memory ownership, cloning, etc.

I will update the code as various language constructs and libraries become available, and as I understand Rust better. Rust feels very much like programming in C, ironically, except that memory layout and management is exposed in the type system.

### System requirements

You need to install Rust with [Cargo](http://crates.io/):

```
$ curl www.rust-lang.org/rustup.sh | sudo bash
```

