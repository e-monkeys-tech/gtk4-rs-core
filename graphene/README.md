# Rust Graphene bindings

__Rust__ bindings and wrappers for [__Graphene__](https://github.com/ebassi/graphene), part of [gtk-rs-core](https://github.com/e-monkeys-tech/gtk4-rs-core).

Graphene __2.44__ is the lowest supported version for the underlying library.

## Minimum supported Rust version

Currently, the minimum supported Rust version is `1.54.0`.

## Documentation

 * [Rust API - Stable](https://e-monkeys-tech/gtk4-rs-core/stable/latest/docs/graphene/)
 * [Rust API - Development](https://e-monkeys-tech/gtk4-rs-core/git/docs/graphene)
 * [C API](https://developer.gnome.org/graphene/stable/)
 * [GTK Installation instructions](https://www.gtk.org/docs/installations/)

## Using

We recommend using [crates from crates.io](https://crates.io/keywords/gtk-rs),
as [demonstrated here](https://gtk-rs.org/#using).

If you want to track the bleeding edge, use the git dependency instead:

```toml
[dependencies]
graphene = { git = "https://github.com/e-monkeys-tech/gtk4-rs-core.git", package = "graphene" }
```

Avoid mixing versioned and git crates like this:

```toml
# This will not compile
[dependencies]
graphene = "0.13"
graphene = { git = "https://github.com/e-monkeys-tech/gtk4-rs-core.git", package = "graphene" }
```

### See Also

 * [glib](https://crates.io/crates/glib)

## License

__graphene__ is available under the MIT License, please refer to it.
