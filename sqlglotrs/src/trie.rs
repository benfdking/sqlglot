use rustc_hash::FxHashMap as HashMap;

#[derive(Debug)]
pub struct TrieNode {
    is_word: bool,
    children: HashMap<char, TrieNode>,
}

#[derive(Debug)]
pub enum TrieResult {
    Failed,
    Prefix,
    Exists,
}

impl TrieNode {
    pub fn contains(&self, key: &str) -> (TrieResult, &TrieNode) {
        if key.is_empty() {
            return (TrieResult::Failed, self);
        }

        let mut current = self;
        for c in key.chars() {
            match current.children.get(&c) {
                Some(node) => current = node,
                None => return (TrieResult::Failed, current),
            }
        }

        if current.is_word {
            (TrieResult::Exists, current)
        } else {
            (TrieResult::Prefix, current)
        }
    }
}

#[derive(Debug)]
pub struct Trie {
    pub root: TrieNode,
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode {
                is_word: false,
                children: HashMap::default(),
            },
        }
    }

    pub fn add<'a, I>(&mut self, keys: I)
    where
        I: Iterator<Item = &'a String>,
    {
        for key in keys {
            let mut current = &mut self.root;
            for c in key.chars() {
                current = current.children.entry(c).or_insert(TrieNode {
                    is_word: false,
                    children: HashMap::default(),
                });
            }
            current.is_word = true;
        }
    }
}
