# Rust GIO bindings

__Rust__ bindings and wrappers for [GIO](https://developer.gnome.org/gio/), part of [gtk-rs-core](https://github.com/e-monkeys-tech/gtk4-rs-core).

GIO __2.48__ is the lowest supported version for the underlying library.

## Minimum supported Rust version

Currently, the minimum supported Rust version is `1.54.0`.

## Documentation

 * [Rust API - Stable](https://e-monkeys-tech/gtk4-rs-core/stable/latest/docs/gio/)
 * [Rust API - Development](https://e-monkeys-tech/gtk4-rs-core/git/docs/gio)
 * [C API](https://developer.gnome.org/gio/stable/)
 * [GTK Installation instructions](https://www.gtk.org/docs/installations/)

## Using

We recommend using [crates from crates.io](https://crates.io/keywords/gtk-rs),
as [demonstrated here](https://gtk-rs.org/#using).

If you want to track the bleeding edge, use the git dependency instead:

```toml
[dependencies]
gio = { git = "https://github.com/e-monkeys-tech/gtk4-rs-core.git", package = "gio" }
```

Avoid mixing versioned and git crates like this:

```toml
# This will not compile
[dependencies]
gio = "0.13"
gio = { git = "https://github.com/e-monkeys-tech/gtk4-rs-core.git", package = "gio" }
```

### See Also

 * [glib](https://crates.io/crates/glib)

## License

__gio__ is available under the MIT License, please refer to it.
