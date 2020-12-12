use super::member::*;
use super::message::*;

#[derive(Debug, Default)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub members: Vec<Member>,
    pub messages: Vec<Message>,
    pub sub_nodes: Vec<Node>,
}

impl Node {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Self::default()
        }
    }

    pub fn flatten(&self) -> Vec<&Node> {
        let mut res = Vec::with_capacity(self.sub_nodes.len() + 1);
        res.push(self);
        for subnode in &self.sub_nodes {
            res.extend(subnode.flatten());
        }
        res
    }

    pub fn find_node(&self, id: &str) -> Option<&Node> {
        self.find_nodes(&[id]).first().map(|n| *n)
    }

    pub fn find_nodes(&self, ids: &[&str]) -> Vec<&Node> {
        self.flatten()
            .into_iter()
            .filter_map(|node| {
                if ids.contains(&node.id) {
                    Some(node)
                } else {
                    None
                }
            })
            .collect()
    }
}
