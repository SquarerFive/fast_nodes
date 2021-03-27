
extern crate ndarray;
use ndarray::{Array3, Zip};
use noise::{Perlin};
use std::collections::HashMap;
use std::sync::Arc;

use fasteval::{Compiler};
use std::collections::BTreeMap;

pub mod graph_builder;

extern crate meval;
// use ndarray::parallel::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ENodeType {
    ADD,
    DIVIDE,
    MULTIPLY,
    SUBTRACT,
    CONSTANT,
    ASSIGNMENT,
    CUSTOM,
}

/// Represents a node within a graph.
///
/// ## Examples
/// ### Node variable
/// ```
/// use fast_nodes::core::{FNode, FGraph, FGraphBase, ENodeType, compute, FNodeBase};
/// let my_node_value = FNode::new(
///     "My Node 1".to_string(),
///     0,
///     None,
///     4.0,
///     None,
///     ENodeType::CONSTANT, // results in `let mut my_node_1:f64 = 4.0;`
/// );
/// ```
/// ## Node Operation
/// ```
/// use fast_nodes::core::{FNode, FGraph, FGraphBase, ENodeType, compute, FNodeBase};
/// let my_node_operation = FNode::new(
///     "My Node Operation".to_string(),
///     0,
///     None,
///     4.0,
///     Some(|_x:i32, _y:i32, _z:i32, val:Vec<f64>| {val[0]* val[1]}),
///     ENodeType::MULTIPLY, // This will tell the graph to automatically optimize this node into a single instruction.
/// );
/// ```

#[derive(Debug, Clone)]
pub struct FNode {
    /// Name of the node
    pub name: String,
    /// Node Index
    pub id: i32,
    /// Connected nodes
    pub connected_nodes: Option<Vec<i32>>,
    /// Node compute closure
    // pub compute : fn(Box<Vec<Box<FNode>>>) -> f64,
    /// Node value.
    pub value: f64,

    /// This will only be used for fast computation
    // pub compute_fast : fn((Rc< Option< FNode>>, Rc<Option< FNode>>, Rc<Option< FNode>>)) -> f64,
    // pub compute_grid : Option<fn(&'static dyn Any) -> f64>,
    pub compute_lw: Option<fn(i32, i32, i32, Vec<f64>) -> f64>,
    /// this will eventually replace compute
    pub node_type: ENodeType,

    pub compute_sm: Option<Arc<Perlin>>,
    pub expression: String,
}


#[derive(Debug, Clone)]
pub struct FGraph {
    pub nodes_pair: HashMap<i32, FNode>,
    pub nodes: BTreeMap<i32, FNode>,
    pub val_single_cache: Option<f64>,
}
pub trait FNodeBase {
    fn copy(self) -> Self;
    fn new(
        name: String,
        id: i32,
        connected_nodes: Option<Vec<i32>>,
        value: f64,
        compute_lw: Option<fn(i32, i32, i32, Vec<f64>) -> f64>,
        node_type: ENodeType,
    ) -> Self;
}
impl FNodeBase for FNode {
    fn new(
        name: String,
        id: i32,
        connected_nodes: Option<Vec<i32>>,
        value: f64,
        compute_lw: Option<fn(i32, i32, i32, Vec<f64>) -> f64>,
        node_type: ENodeType,
    ) -> Self {
        Self {
            name: name,
            id: id,
            connected_nodes: connected_nodes,
            value: value,
            compute_lw: compute_lw,
            node_type: node_type,
            compute_sm: None,
            expression: "value".to_string(),
        }
    }

    fn copy(self) -> Self {
        let n = FNode {
            name: self.name,
            id: self.id,
            connected_nodes: self.connected_nodes,
            value: self.value,
            node_type: self.node_type,
            compute_lw: None,
            compute_sm: self.compute_sm,
            expression: self.expression,
        };

        n
    }
}

impl Default for FNode {
    fn default() -> Self {
        Self {
            name: "my_node".to_string(),
            id: 0,
            connected_nodes: None,
            value: 0.0,
            compute_lw: None,
            compute_sm: None,
            expression: "value".to_string(),
            node_type: ENodeType::CONSTANT
        }
    }
}

pub trait FGraphBase {
    fn default() -> Self;
    fn new(nodes: BTreeMap<i32, FNode>, nodes_pair: HashMap<i32, FNode>) -> Self;

    fn set_node_value_fast(self, target: i32, value: f64) -> Self;
    fn set_node_value(self, target: i32, value: f64) -> Self;
    fn set_cached_value_ref(&mut self, value: f64) -> &mut Self;

    fn get_node(&self, target: i32) -> &FNode;
}

impl FGraphBase for FGraph {
    fn default() -> Self {
        Self {
            nodes: BTreeMap::new(),
            nodes_pair: HashMap::new(),
            val_single_cache: None,
        }
    }
    fn new(nodes: BTreeMap<i32, FNode>, nodes_pair: HashMap<i32, FNode>) -> Self {
        Self {
            nodes: nodes,
            nodes_pair: nodes_pair,
            val_single_cache: None,
        }
    }
    fn set_node_value_fast(mut self, target: i32, value: f64) -> Self {
        self.nodes_pair
            .get_mut(&target)
            .expect("Invalid Node!")
            .value = value;
        self
    }
    fn set_node_value(mut self, target: i32, value: f64) -> Self {
        self.nodes
            .get_mut(&target)
            .expect("Invalid Node Index!")
            .value = value;
        self
    }
    fn set_cached_value_ref(&mut self, value: f64) -> &mut Self {
        self.val_single_cache = Some(value);
        self
    }

    fn get_node(&self, target: i32) -> &FNode {
        self.nodes.get(&target).expect("Node does not exist in graph tree!")
    }
}

pub fn build_graph(graph: &FGraph, node: &FNode) -> String {
    if node.connected_nodes.is_none() {
        return node.expression.replace("value", &node.value.to_string());
    } else {
        let children: BTreeMap<&i32, &FNode> = graph
            .nodes
            .iter()
            .filter(|(node_id, _n_ref)| {
                node.connected_nodes
                    .as_ref()
                    .expect("Invalid Node")
                    .contains(node_id)
            })
            .collect();
        if children.len() > 0 {
            //let mut values: Vec<f64> = vec![0.0; children.len()];
            //values.reserve(children.len());
            //values.par_iter_mut().enumerate().for_each(|(i, v) |{*v = compute_lw(x,y,z, graph, children[i]); });
            let mut result: String = node.expression.clone();
            for (idx, child) in children.iter().enumerate() {
                //values[idx] = compute_lw(x,y,z,graph, child);
                let mut val_id = "_i".to_string();
                val_id.push_str(&idx.to_string());
                let t = result.replace(&val_id, &build_graph(graph, child.1));
                result = t;
            }

            return result.clone();
        } else {
            return node.expression.replace("value", &node.value.to_string());
        }
    }

    //return node.expression.replace("value", &node.value.to_string());
}

pub fn build_graph_as_rs(graph: &FGraph, node: &FNode) -> String {
    let mut result: String;
    if node.connected_nodes.is_none() {
        result = node.expression.replace("value", &node.value.to_string());
    } else {
        // For now we will only support one input
        let children: BTreeMap<&i32, &FNode> = graph
            .nodes
            .iter()
            .filter(|(node_index, _n_obj)| {
                node.connected_nodes
                    .as_ref()
                    .expect("Invalid Node")
                    .contains(node_index)
            })
            .collect();
        // let mut values: Vec<f64> = vec![0.0; children.len()];
        result = "let __var_name__: f64 = __value__;\n"
            .to_string()
            .replace("__var_name__", node.name.as_str());
        let children_values : Vec<&&FNode> = children.values().collect();
        result = result.replace("__value__", build_graph(graph, children_values[0]).as_str());
    }

    result
}

pub fn compile_graph(graph: &FGraph, node: &FNode) -> impl Fn(f64, f64, f64) -> f64 {
    let instruction = build_graph(graph, node);
    println!("Instruction {}", instruction);
    let expr: meval::Expr = instruction.parse().unwrap();
    let _func = expr.bind3("x", "y", "z").unwrap();
    _func
}

pub fn compute_lw(x: i32, y: i32, z: i32, graph: &FGraph, node: &FNode) -> f64 {
    if node.connected_nodes.is_none() {
        node.value
    } else {
        let children: BTreeMap<&i32, &FNode> = graph
            .nodes
            .iter()
            .filter(|(node_index, _node_obj)| {
                node.connected_nodes
                    .as_ref()
                    .expect("Invalid Node")
                    .contains(node_index)
            })
            .collect();
        if children.len() > 0 {
            let mut values: Vec<f64> = vec![0.0; children.len()];
            //values.reserve(children.len());
            //values.par_iter_mut().enumerate().for_each(|(i, v) |{*v = compute_lw(x,y,z, graph, children[i]); });

            for (idx, (_nidx, child)) in children.iter().enumerate() {
                values[idx] = compute_lw(x, y, z, graph, child);
            }

            (node.compute_lw.unwrap())(x, y, z, values)
        } else {
            node.value
        }
    }
}

pub fn compute(graph: &FGraph, node: &i32) -> f64 {
    let node = graph
        .nodes
        .get(node)
        .expect("Invalid node supplied!");
    compute_lw(0, 0, 0, graph, node)
}


pub fn compile_graph_(
    graph: &FGraph,
    node: &FNode,
    slab: &mut fasteval::Slab,
    _map: &mut BTreeMap<String, f64>,
) -> fasteval::Instruction //Result<(), fasteval::Error>
{
    let parser = fasteval::Parser::new();
    let graph_instruction = build_graph(graph, node);

    let compiled = parser
        .parse(graph_instruction.as_str(), &mut slab.ps)
        .unwrap()
        .from(&slab.ps)
        .compile(&slab.ps, &mut slab.cs);
    compiled
}
pub fn grid3(graph: &FGraph, node_id: &i32, size: &i32) -> Array3<f64> {

    let mut n_array = Array3::<f64>::zeros((*size as usize, *size as usize, *size as usize));
    let mut i_array = Array3::<i32>::zeros((*size as usize, *size as usize, *size as usize));

    i_array
        .iter_mut()
        .enumerate()
        .for_each(|(i, v)| *v = i as i32);

    let node = graph.nodes.get(node_id ).unwrap();
    let mut slab = fasteval::Slab::new();
    let mut map: BTreeMap<String, f64> = BTreeMap::new();

    let _myf = Arc::new(compile_graph_(graph, node, &mut slab, &mut map));
    let instruction = build_graph(graph, node);

    let parser = fasteval::Parser::new();
    let _expr = Arc::new(
        parser
            .parse(instruction.as_str(), &mut slab.ps)
            .unwrap()
            .from(&slab.ps),
    );

    Zip::from(&mut n_array).and(&i_array).par_apply(|n, &i| {
        let x = i / (*size * (*size));
        let y = (i / *size) % *size;
        let z = i % *size;
        let _ns = fasteval::EmptyNamespace;
        *n = compute_lw(x, y, z, graph, node); //ref_gf(x as f64, y as f64, z as f64);
    });


    n_array
}


