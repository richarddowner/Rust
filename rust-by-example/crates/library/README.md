# Library

Let's create a library, and then see how to link it to another crate.

```
$ rustc --crate-type=lib erty.rs
$ ls lib*
liberty-e6eaab2e-0.0.rlib
```

Libraries get prefixed with "lib", and contain a hash and their version in the name.
The version and name of the library can be changed using attributes.
