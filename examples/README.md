# Examples

## Circle

The simplest example and a good starting point for understanding how to use
this library.

```
cargo run --example circle
```

Demonstrates:

- Creating a `Grid`
- Selecting and mutating a set of `Coord`s from a pattern

## Ring

Builds upon the Circle example, adding a bit of complexity.

```
cargo run --example ring
```

Demonstrates:

- Combining pattern iterators
- Using flood fill to select an internal region

## Life

Uses the techniques shown previously to implement Conway's Game of Life.

```
cargo run --example life
```

## Dungeon

A more advanced example showing simple dungeon room generation.

```
cargo run --example dungeon
```

Demonstrates:

- Advanced `Rect` usage with binary space partitioning
- Using `Cluster` patterns