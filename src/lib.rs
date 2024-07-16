use std::collections::HashMap;

use extism_pdk::*;
use schnuffel_types::graph::{Graph, Node};
use schnuffel_types::plugin::{Input, Output};

#[plugin_fn]
pub fn default_config(_: Input<String>) -> FnResult<Output<HashMap<String, String>>> {
    Ok(Output {
        data: HashMap::new(),
    })
}

#[plugin_fn]
pub fn exec_on_node(_: Input<Node>) -> FnResult<Output<Graph>> {
    Ok(Output { data: Graph::new() })
}

#[plugin_fn]
pub fn exec_on_graph(_: Input<Graph>) -> FnResult<Output<Graph>> {
    Ok(Output { data: Graph::new() })
}
