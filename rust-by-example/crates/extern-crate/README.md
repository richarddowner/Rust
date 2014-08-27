# Extern Crates
```
# The `-L.` arg adds the current directory to the library search path
$ rustc -L . executable.rs && ./executable
called erty's `public_function()`
called erty's `indirect_access()`, that
> called erty's `private_function()`
```
