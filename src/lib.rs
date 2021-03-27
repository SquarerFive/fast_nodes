#![crate_name = "fast_nodes"]

// low level node implementation

/// The VCNode Module, contains the absolute bare mininumum to create a graph.
pub mod core;


#[cfg(test)]
mod tests {
    
    use super::core::*;
    use std::{collections::BTreeMap, time::{SystemTime}};
    use std::collections::HashMap;
    use graph_builder::FGraphBuilder;
    use tempus_fugit::*;
    


    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn node_grid_test()
    {
        /*
        let constant1 = FNode::new(
            "My Variable 1".to_string(),
            0, 
            None, 
            4.0, 
            None,
            ENodeType::CONSTANT,
        );
        let constant2 = FNode::new(
            "My Variable 1".to_string(),
            1, 
            None, 
            8.0, 
            None,
            ENodeType::CONSTANT,
        );

        let mut operator = FNode::new(
            "My Operator".to_string(), 
            2, 
            Some(vec![constant1.id, constant2.id]), 
            2.0,
            Some(|_x:i32, _y:i32, _z: i32, inputs: Vec<f64>| {inputs[0]*inputs[1]}),
            ENodeType::MULTIPLY
        );

        let mut operator2 = FNode::new(
            "my_op2".to_string(), 
            3, 
            Some(vec![operator.id, constant1.id]), 
            2.0,
            Some(|_x:i32, _y:i32, _z: i32, inputs: Vec<f64>| {inputs[0]+inputs[1]}),
            ENodeType::ADD
        );

        let mut my_assignment = FNode::new( 
            "my_assignment".to_string(),
            4,
            Some(vec![operator2.id]),
            2.0,
            Some(|_x: i32, _y: i32, _z: i32, inputs: Vec<f64>| {inputs[0]}),
            ENodeType::ASSIGNMENT
        );
        */
        

        // let graph = FGraph::new( BTreeMap::new(),  HashMap::new()); //  FGraph {nodes: vec![constant1, constant2, operator, operator2], nodes_pair: HashMap::new()};

        let graph_builder_ = FGraphBuilder::new()
            .create_node("my_var", 3.5)
            .create_node("my_other_var", 2.0)
            .mul(vec![1, 2])
            .assign("my_assignment")
            .create_node("my_new_var", 7.0)
            .create_node("my_new_var2", 14.0)
            .div(vec![5, 6])
            .assign("my_other_assignment")
            .build()
            .as_string();

        println!("built graph: {}", graph_builder_);
    }
}
