use criterion::{black_box, criterion_group, criterion_main, Criterion};

use fast_nodes::core::*;
use fast_nodes::core::graph_builder::*;
use std::collections::HashMap;
use noise::{NoiseFn, Perlin};
use std::collections::BTreeMap;
use fasteval::Evaler;    // use this trait so we can call eval().
use fasteval::Compiler;  // use this trait so we can call compile().


// TODO: Add graph compiler
fn node_addition(c: &mut Criterion) -> Result<(), fasteval::Error> {
    let graph_builder = FGraphBuilder::new()
        .create_node("a", 3.5)
        .create_node("b", 2.0)
        .add(vec![1, 2])
        .assign("value")
        .build()
        .as_string();
    println!("Finished bench!");
    Ok(())
}


criterion_group!(benches, node_addition);
criterion_main!(benches);