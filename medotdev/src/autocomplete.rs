use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, Box<TrieNode>>,
    is_end_of_command: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end_of_command: false,
        }
    }
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, command: &str) {
        let mut node = &mut self.root;

        for ch in command.chars() {
            node = node.children.entry(ch).or_insert(Box::new(TrieNode::new()));
        }

        node.is_end_of_command = true;
    }

    pub fn search(&self, prefix: &str) -> Vec<String> {
        let mut node = &self.root;
        for ch in prefix.chars() {
            if let Some(next) = node.children.get(&ch) {
                node = next;
            } else {
                return Vec::new();
            }
        }

        self.collect_all_commands(prefix, node)
    }

    fn collect_all_commands(&self, prefix: &str, node: &TrieNode) -> Vec<String> {
        let mut results = Vec::new();
        if node.is_end_of_command {
            results.push(prefix.to_string());
        }

        for (ch, child) in &node.children {
            results.extend(self.collect_all_commands(&format!("{}{}", prefix, ch), child));
        }

        results
    }
}
