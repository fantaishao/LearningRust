pub struct TreeNode {
    id: String,
    label: String,
    children: Vec<TreeNode>,
}

pub struct TreeData {
    nodes: Vec<TreeNode>,
}

impl TreeData {
    pub fn new() -> Self {
        // You can load this from a file, database, or hardcode for testing
        let nodes: Vec<TreeNode> = vec![
            TreeNode {
            id: "1".to_string(),
            label: "root".to_string(),
            children: vec![
                TreeNode {
                    id: "2".to_string(),
                    label: "label-2".to_string(),
                    children: vec![
                        TreeNode {
                            id: "4".to_string(),
                            label: "label-3".to_string(),
                            children: vec![]
                        }
                    ]
                },
                TreeNode {
                    id: "3".to_string(),
                    label: "label-3".to_string(),
                    children: vec![]
                }
            ]
        },
        TreeNode {
            id: "5".to_string(),
            label: "label-5".to_string(),
            children: vec![
                TreeNode {
                    id: "6".to_string(),
                    label: "label-6".to_string(),
                    children: vec![
                        TreeNode {
                            id: "7".to_string(),
                            label: "label-7".to_string(),
                            children: vec![]
                        }
                    ]
                }
            ]
        }
    ];

        TreeData { nodes }
    }

    pub fn to_json(&self) -> String {
        // A simple JSON serialization
        // In a real application, you would use serde_json
        let mut json = String::from("{\n  \"nodes\": [\n");
        
        for (i, node) in self.nodes.iter().enumerate() {
            json.push_str(&self.node_to_json(node, 4));
            if i < self.nodes.len() - 1 {
                json.push_str(",\n");
            } else {
                json.push_str("\n");
            }
        }
        
        json.push_str("  ]\n}");
        json
    }

    fn node_to_json(&self, node: &TreeNode, indent: usize) -> String {
        let indent_str = " ".repeat(indent);
        let mut json = format!("{}{{\"id\": \"{}\", \"label\": \"{}\", \"children\": [", 
        indent_str, node.id, node.label);

        if !node.children.is_empty() {
            for (i, child) in node.children.iter().enumerate() {
                json.push_str(&self.node_to_json(child, indent + 2));
                if i < node.children.len() - 1 {
                    json.push_str(",\n");
                } else {
                    json.push_str("\n");
                }
            }
            json.push_str(&format!("{}]", indent_str));
        } else {
            json.push_str("]");
        }
        
        json.push_str("}");
        json
    }
}

pub fn get_tree_data() -> String {
    let tree_data = TreeData::new();
    tree_data.to_json()
}


pub fn get_node(id: &str) -> String {
    let tree_data = TreeData::new();

    // find the node with the given ID
    if let Some(node) = find_node_by_id(&tree_data.nodes, id) {
        // Return the found node as JSON
        format!("{{\"node\": {}}}", tree_data.node_to_json(&node, 2))
    } else {
        // Return an error message if node not found
        format!("{{\"error\": \"Node with ID '{}' not found\"}}", id)
    }
}

pub fn delete_node(id: &str) -> String {
    let mut tree_data = TreeData::new();

    // try to find and remove the node
    let mut removed_node = None;

    // First check if it's a top-level node
    for i in 0..tree_data.nodes.len() {
        if tree_data.nodes[i].id == id {
            removed_node = Some( tree_data.nodes.remove(i));
            break;
        }
    }

    // if not found at top level, search in children
    if removed_node.is_none() {
        removed_node = remove_node_recursive(&mut tree_data.nodes, id);
    }
    if let Some(node) = removed_node {
        // Return the deleted node as JSON
        format!("{{\"deleted_node\": {}}}", tree_data.node_to_json(&node, 2))
    } else {
        // Return an error message if node not found
        format!("{{\"error\": \"Node with ID '{}' not found\"}}", id)
     }
}

fn find_node_by_id<'a>(nodes: &'a[TreeNode], target_id: &str) -> Option<&'a TreeNode> {
    for node in nodes {
        if node.id == target_id {
            return Some(node);
        }

        // Recursively search in children
        if !node.children.is_empty() {
            if let Some(found) = find_node_by_id(&node.children, target_id) {
                return Some(found);
            }
        }
    }

    None
}

fn remove_node_recursive(nodes: &mut Vec<TreeNode>, target_id: &str) -> Option<TreeNode> {
    let mut i = 0;
    while i < nodes.len() {
        // Check if this node is the target
        if nodes[i].id == target_id {
            return Some(nodes.remove(i));
        }
        
        // Check if the target is in this node's children
        if let Some(removed) = remove_node_recursive(&mut nodes[i].children, target_id) {
            return Some(removed);
        }
        
        i += 1;
    }
    None
}