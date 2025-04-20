use serde::{Deserialize, Serialize};
use std::{error::Error, fmt::format, string};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Node {
    id: String,
    label: String,
    children: Vec<Node>,
}

/***
 * 根据id查找节点
 */
fn find_node_by_id(data: &[Node], child_id: &str) -> Option<Node> {
    for node in data {
        if node.id == child_id {
            return Some(node.clone());
        }
        if node.children.len() > 0 {
            let result = find_node_by_id(&node.children, child_id);
            if result.is_some() {
                return result;
            }
        }
    }

    // return Node when no node is found
    None
}

/**
 * 指定父级id，添加子节点
 */
fn append_node(data: &[Node], parent_id: &str, new_node: Node) -> Vec<Node>{
    // create a deep copy of the data to modify
    let mut result = data.to_vec();

    // helper function to recursively find and modify the parent node
    fn find_and_append(nodes: &mut[Node], parent_id: &str, new_node: Node) -> bool {
        for node in nodes.iter_mut() {
            // check if this is the parent node
            if node.id == parent_id {
                // add the new node to children
                node.children.push(new_node);
                return true ;
            }

            // recursively check children
            if find_and_append(&mut node.children, parent_id, new_node.clone()) {
                return true
            }
        }
        false
    }

    // try to find and modify the parent node
    find_and_append(&mut result, parent_id, new_node);

    result
}

fn main() {
    match fetch_tree_data() {
        Ok(tree_data) => {
            println!("tree_data: {:?}", tree_data);
            
            match find_node_by_id(&tree_data, "4") {
                Some(node) => println!("find node by id {:?}", node),
                None => println!("not found")
            }

            let new_node = Node {
                id: "8".to_string(),
                label: "label-8888".to_string(),
                children: vec![]
            };
            let updated_tree = append_node(&tree_data, "4", new_node);
            println!("new tree data is: {:?}", updated_tree);
        },
        Err(e) => println!("Error fetching tree data: {}", e)
    }

    let node_id = "4";
    match get_node_by_id(node_id)  {
        Ok(node) => println!("Node fetched from server: {:?}", node),
        Err(e) => println!("Error fetching node : {}", e),
    }

    match delete_node_by_id(node_id) {
        Ok(node) => {
            println!("node deleted from server: {:?}", node);

            // fetch tree data again after successful deletion
            match fetch_tree_data()  {
                Ok(updated_tree_data) => {
                    println!("Updated tree data after deletion: {:?}", updated_tree_data);
                    
                    // verify the node is gone
                    match find_node_by_id(&updated_tree_data, node_id) {
                        Some(_) => println!("Warning: Node {} still exists!", node_id),
                        None => println!("Confirmed: Node {} was successfully deleted", node_id)
                    }
                },
                Err(e) => println!("Error fetching updated tree data: {}", e)
            }
        },
        Err(e) => println!("Error deleting node : {}", e),
    }
}

/***
 * 从后端项目web_server的接口 http://localhost:7878/api/treedata获取tree_data
 */
fn fetch_tree_data() -> Result<Vec<Node>, Box<dyn Error>> {
    // create a blocking client
    let client = reqwest::blocking::Client::new();

    // make the GET request
    let resp = client.get("http://localhost:7878/api/treedata").send()?.text()?;

    // parse the json response
    let parsed: serde_json::Value = serde_json::from_str(&resp)?;

    // extract the nodes array
    let nodes = parsed["nodes"].as_array().ok_or("Invalid response format:'nodes' array not found")?;

    // deserialize into Vec<Node>
    let tree_data: Vec<Node> = serde_json::from_value(nodes.clone().into())?;

    Ok(tree_data)
}

/***
 * 从后端项目web_server的接口 http://localhost:7878/api/getTreeDataById获取指定ID的节点
 */
fn get_node_by_id(id: &str) -> Result<Node, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let url = format!("http://localhost:7878/api/getNode?id={}", id);
    let resp = client.get(url).send()?.text()?;

    // parse the json response
    let parsed: serde_json::Value = serde_json::from_str(&resp)?;

    // extract the nodes array
    let node = parsed["node"].as_object().ok_or("Invalid response format:'node' not found")?;

    // deserialize into Vec<Node>
    let node_data: Node = serde_json::from_value(parsed["node"].clone())?;

    Ok(node_data)
}

/***
 * 从后端项目web_server的接口 http://localhost:7878/api/deleteNodeById删除指定ID的节点
 */
fn delete_node_by_id(id: &str) -> Result<Node, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let url = format!("http://localhost:7878/api/deleteNode?id={}", id);
    let resp = client.get(url).send()?.text()?;

    println!("Raw response: {}", resp);

    // parse the json response
    let parsed: serde_json::Value = serde_json::from_str(&resp)?;

    println!("Is deleted_node an object? {}", parsed["deleted_node"].is_object());

    // check if the response contains a deleted_node filld
    if parsed["deleted_node"].is_object() {
       println!("Deleted node content: {}", parsed["deleted_node"]);
        // deseriallize into Node
        let node_data: Node = serde_json::from_value(parsed["deleted_node"].clone())?;
        Ok(node_data)
    } else if parsed["error"].is_string() {
        // Handle error response
        Err(format!("Server error: {}", parsed["error"].as_str().unwrap_or("Unknown error")).into())
    } else {
        // Unexpected response format
        Err(format!("Unexpected response format: {}", resp).into())
    }
}

