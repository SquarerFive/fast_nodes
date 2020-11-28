
use crate::core::*;


pub struct FCompiledGraph {
    payload: String
}



impl FCompiledGraph
{
    pub fn as_string(self) -> String {
        self.payload.clone()
    }
}

pub struct FNodeBuilder {
    pub node: FNode,
}

impl Default for FNodeBuilder {
    fn default () -> Self {
        let node_builder = FNodeBuilder {
            node: FNode::default(),
        };
        node_builder
    }
}

pub struct FGraphBuilder {
    pub graph: FGraph,
}

impl Default for FGraphBuilder {
    fn default() -> Self {
        let graph_builder = FGraphBuilder {
            graph: FGraph::default(),
        };
        graph_builder
    }
}

impl FGraphBuilder {
    pub fn new() -> FGraphBuilder {
        FGraphBuilder {
            graph: FGraph::default(),
        }
    }
    pub fn get_graph(&self) -> &FGraph
    {
        &self.graph
    }

    pub fn add_input(&mut self) -> &mut Self {
        self
    }

    pub fn create_node(&mut self, name: &str, value: f64) -> &mut Self {
        let node_id: i32 = (self.graph.nodes.len() + 1) as i32;
        let node = FNode::new(name.to_string(), node_id, None, value, None, ENodeType::CONSTANT);
        self.graph.nodes.insert(
            node_id,
            node // graph.nodes now owns node
        );
        self
    }
    pub fn add(&mut self, nodes : Vec<i32>) -> &mut Self {
        let node_id: i32 = (self.graph.nodes.len() + 1) as i32;
        let mut node_name: String = "__add".to_string();
        node_name = format!("{}{idx}__", node_name, idx=node_id);
        let node = FNode::new(
            node_name, node_id, Some(nodes.clone()), 0.0, None, ENodeType::ADD
        );
        let mut exp = "".to_string();
        for i in 0..nodes.len() 
        {
            if i > 0 {
                exp.push_str(&" + ".to_string());
            }
            exp.push_str("_i");
            exp.push_str(&i.to_string());
        }
        self.graph.nodes.insert(node_id, node);
        self
    }

    
    pub fn assign(&mut self, name:String) -> &mut Self{
        let node_id:i32 = (self.graph.nodes.len()+1) as i32;
        let node_name = name;

        let mut node = FNode::new(
            node_name.clone(), node_id, Some(vec![node_id-1]), 0.0, 
            None, ENodeType::ASSIGNMENT
        );
        // TODO: generalised language for the graph that translates to RUST/GLSL
        node.expression = format!("let {variable_name} : f64 = _i0;\n", variable_name = node_name);
        // though it would be nice to directly have gpu code in rust (maybe rust-gpu) - iirc it doesn't have full support for compute shaders yet.
        self.graph.nodes.insert(node_id, node);
        self
    }
    
    pub fn mul(&mut self, nodes : Vec<i32>) -> &mut Self {
        let node_id: i32 = (self.graph.nodes.len() + 1) as i32;
        let mut node_name: String = "__mul".to_string();
        node_name = format!("{}{idx}__", node_name, idx=node_id);
        let mut node = FNode::new(
            node_name, node_id, Some(nodes.clone()), 0.0, None, ENodeType::MULTIPLY
        );
        let mut exp = "".to_string();
        for i in 0..nodes.len() 
        {
            if i > 0 {
                exp.push_str(&" * ".to_string());
            }
            exp.push_str("_i");
            exp.push_str(&i.to_string());
        }
        node.expression = exp;
        self.graph.nodes.insert(node_id, node);
        self
    }

    pub fn div(&mut self, nodes : &Vec<i32>) -> &mut Self {
        let node_id: i32 = (self.graph.nodes.len() + 1) as i32;
        let mut node_name: String = "__div".to_string();
        node_name = format!("{}{idx}__", node_name, idx=node_id);
        let node = FNode::new(
            node_name, node_id, Some(nodes.clone()), 0.0, None, ENodeType::DIVIDE
        );
        let mut exp = "".to_string();
        for i in 0..nodes.len() 
        {
            if i > 0 {
                exp.push_str(&" / ".to_string());
            }
            exp.push_str("_i");
            exp.push_str(&i.to_string());
        }
        self.graph.nodes.insert(node_id, node);
        self
    }

    pub fn build(&self) -> String{
        build_graph(&self.graph, self.graph.nodes.get(&((self.graph.nodes.len() as i32)-0))
            .expect("Nothing in BTreeMap"))
    }

    pub fn build_(&self) -> FCompiledGraph {
        // collect all of our assignments first
        let assignments: BTreeMap<&i32, &FNode> = self.graph.nodes.iter()
            .filter(|(_node_index, node_)| node_.node_type == ENodeType::ASSIGNMENT).collect();
        
        let mut result: String = "".to_string();
        for assignment in assignments {
            result.push_str(
                build_graph_as_rs(&self.graph, assignment.1).as_str()
            );
            //result.push_str("\n");
        }        
        
        FCompiledGraph {payload: result}
    }

    

}



pub trait FGraphCompiler {
    fn compile(self);
}

impl FGraphCompiler for String {
    fn compile(self) {

    }
}