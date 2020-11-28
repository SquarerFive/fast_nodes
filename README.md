# Fast Nodes
fast_nodes is a library purposed for use in my game engine, it is currently being built off my existing node graph which was done in Python and optimized so it can be run in parallel to generate voxel worlds.

This is in no way, shape or form - production ready, and is subject to significant changes.

## An example
```rust
let graph_builder_ = FGraphBuilder::new()
            .create_node("my_var", 3.5)
            .create_node("my_other_var", 2.0)
            .mul(vec![1, 2])
            .assign("my_assignment".to_string())
            .create_node("my_new_var", 7.0)
            .assign("my_other_assignment".to_string())
            .build_()
            .as_string();

        println!("built graph: {}", graph_builder_);
```

## General concept
The graph itself builds a string of instructions which is then converted into Rust code, I plan on having it compile into a shared library so it can be accessed across the whole project. 

# Rust Template
Standard template which will be used for my projects. I aim to move my development ecosystem over to RUST from C++ as it offers both guarunteed memory safety and a system to organise and manage packages/dependencies, while performing at a similar level (or better) than C++. 

## Getting Started
```python
# run tests
cargo test
# build ( add '--release' for a production build)
cargo build
# run
cargo run
```
