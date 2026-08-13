# rust_style_main

This is a tiny freestanding demo-ing using a rust style `main` in a `#[no_std]` such a free standing binary.

Normally, when you are doing a `no_std` bin you end up defining your entry point such as `_start`.
Then you just call whatever code you want from there. Which is fine, but then
you are not really using rust's normal `main` machinery anymore. This example
shows how to get back to that. You still provide your own `_start`, do the tiny
amount of setup needed, and then call the rustc generated `main` wrapper.

So from the user side you still get to write something like:

```rust
fn main() {
    stdout(b"Hello: from main() -> ()\n");
}
```

There are also some commented out `main`s in [src/main.rs](src/main.rs) you can
switch to. The point is that these work more like a normal rust `main`.
The commented out examples in can be swapped in to try other familiar return types:

```rust
fn main() -> i32
fn main() -> Result<(), ()>
```

## What this is showing

- A `#![no_std]` binary crate with its own `_start`.
- A naked x86-64 linux entry point.
- A `__premain` function that calls the rustc generated `main` symbol.
- A small [lang_start/lib.rs](lang_start/lib.rs) crate that defines the
  `start` and `termination` lang items.
- A custom `Termination` trait so `main -> ()`, `main -> i32`, and
  `main -> Result<(), ()>` can all report an exit code.
- Some basic linux syscalls so this does not need libc.

The flow is basically:

```text
_start
  -> __premain
    -> rustc-generated main
      -> #[lang = "start"] my_start
        -> user fn main()
        -> Termination::report()
    -> exit(status)
```

## Requirements / assumptions

This is intentionally experimental and low-level.

It is using internal rust things, inline assembly, linux syscall numbers,
and a hand a written `_start` that I would call incomplete. Just enough
to get things working.

- To enable the unstable lang items features used by `lang_start`
- `RUSTC_BOOTSTRAP=1`, as configured in [build.ninja](build.ninja)

Should you use this in real code, probably not. The point here is more to show the
pieces involved and how they fit together.

## Building

First build the `lang_start` crate:

```sh
ninja
```

Then build and run the actual bin crate:

```sh
cargo run
```

Cargo uses [.cargo/config.toml](.cargo/config.toml), which passes:

- `-C link-arg=-nostartfiles`
- `--extern lang_start=./build/liblang_start.rlib`

The first one keeps the C runtime start files out of it. The second one points
rustc at the helper crate that ninja built.

## Notes

The write syscall helper currently writes to file descriptor `1`, so when you run
this you should see debug strings. This was mainly just for poking at the call
flow.

The different `main` examples are there so you can see the custom
`Termination` impls in [lang_start/lib.rs](lang_start/lib.rs) get used:

- `()` exits with `0`
- `i32` exits with whatever integer you returned
- `Result<_, _>` exits with `0` for `Ok(_)` and `101` for `Err(_)`

Panics are set to abort, and the panic handler just exits with `101`.
