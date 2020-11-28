use criterion::{black_box, criterion_group, criterion_main, Criterion};

use fast_nodes::vcnode::*;
use std::collections::HashMap;
use noise::{NoiseFn, Perlin};
use std::collections::BTreeMap;
use fasteval::Evaler;    // use this trait so we can call eval().
use fasteval::Compiler;  // use this trait so we can call compile().



fn node_addition(c: &mut Criterion) -> Result<(), fasteval::Error> {

    let n1 = FNode::new (
        "Constant 0".to_string(), 
        0, 
        None, 
        4.0,
        None,
        ENodeType::CONSTANT,
    );

    let mut n2 = FNode ::new( 
        "Constant 1".to_string(), 
        1,
        None, 
        2.0,
        None,
        ENodeType::CONSTANT,
    );
    n2.expression = "value".to_string();
    let p = Perlin::new();
    let mut n3 = FNode ::new(
        "Multiplier ".to_string(), 
        2, 
        Some(vec![n1.id, n2.id]), 
        2.0,
        Some(|_x:i32, _y:i32, _z: i32, inputs: Vec<f64>| {  ((_x as f64 % 3.0)*(_y as f64 % 2.0)/(_z as f64 *4.0 % 24.8))*inputs[0]*inputs[1]  }),
        ENodeType::MULTIPLY
    );
    
    //n3.expression = "((x % 3.0)*(y % 2.0)/(z * 4.0 % 24.8)*_i0*_i1)".to_string();
    n3.expression = "(_i0 * _i1)".to_string();

    let mut graph = FGraph::new(vec![n1, n2, n3], HashMap::new()); //FGraph {nodes: vec![n1, n2, n3], nodes_pair: HashMap::new()};

    graph = graph.set_node_value(0, 8.0);

    let eval_node = compile_graph(&graph, &graph.nodes[2]);

    //let g = &mut graph;
    let mut x:f64 = 0.0;
    let mut use_caching: bool = true;
    let mut value : f64 = 0.0;
    let mut slab = fasteval::Slab::new();
    let mut map: BTreeMap<String, f64> = BTreeMap::new();
    let parser = fasteval::Parser::new();
    let graph_instruction = build_graph(&graph, &graph.nodes[2]);
    let compiled = parser.parse(graph_instruction.as_str(), &mut slab.ps).unwrap().from(&slab.ps).compile(&slab.ps, &mut slab.cs);
    //map.insert("x".to_string(), x as f64);
    //#[cfg(feature="unsafe-vars")]
    //unsafe {slab.ps.add_unsafe_var("x".to_string(), &x); }
    let val = 
    c.bench_function("compute_graph", |b| b.iter( || 
        { 
            /* 
            if graph.val_single_cache.is_none() {
                graph.set_cached_value_ref (eval_node(0.0, 0.0, 0.0)); 
            }
            else {

            }
            */
            
            /*
            if (!use_caching) {
                let x = eval_node(x as f64, x as f64, x as f64);
                if eval_node(x as f64, x as f64, x as f64) != value  {
                    value = eval_node(x as f64, x as f64, x as f64);

                }
                else {
                    use_caching = true;

                }
            }
            */
            
            if let fasteval::IConst(_c) = compiled {
                value = _c
            }
            else {
                //#[cfg(feature="unsafe-vars")]
                //if let fasteval::IUnsafeVar{ptr, ..} = compiled {
                  //  unsafe { value = *ptr; }
                //} else {
                value =  compiled.eval(&slab, &mut map).unwrap();
                //}
            }
            
            
             //x+=1.0;
        }));
    
    //assert_eq!(val, 16.0);
    println!("Finished bench!");
    Ok(())
}
/*
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

*/
criterion_group!(benches, node_addition);
criterion_main!(benches);