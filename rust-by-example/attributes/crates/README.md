When the `crate_type` attribute is used, we no longer need to pass the
`--crate-type` flat to `rustc`

```
$ rustc lib.rs
$ ls lib*
liberty-a1e7dc98-0.1.rlib
```
