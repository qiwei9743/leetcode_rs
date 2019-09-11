/*
 * @lc app=leetcode id=208 lang=rust
 *
 * [208] Implement Trie (Prefix Tree)
 *
 * https://leetcode.com/problems/implement-trie-prefix-tree/description/
 *
 * algorithms
 * Medium (39.89%)
 * Total Accepted:    199.7K
 * Total Submissions: 494.4K
 * Testcase Example:  '["Trie","insert","search","search","startsWith","insert","search"]\n[[],["apple"],["apple"],["app"],["app"],["app"],["app"]]'
 *
 * Implement a trie with insert, search, and startsWith methods.
 * 
 * Example:
 * 
 * 
 * Trie trie = new Trie();
 * 
 * trie.insert("apple");
 * trie.search("apple");   // returns true
 * trie.search("app");     // returns false
 * trie.startsWith("app"); // returns true
 * trie.insert("app");   
 * trie.search("app");     // returns true
 * 
 * 
 * Note:
 * 
 * 
 * You may assume that all inputs are consist of lowercase letters a-z.
 * All inputs are guaranteed to be non-empty strings.
 * 
 * 
 */

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

#[derive(Default)]
struct Node {
    children: [Option<Box<Node>>; 26],
    end: bool,
}

impl Node {
    pub fn new() -> Self {
        Default::default()
    }
}

struct Trie {
    root: Node,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self{ root: Default::default() }
    }
    
    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let mut root = &mut self.root;
        for widx in word.bytes().map(|c| (c - b'a') as usize) {
            if let Some(next) = root.children[widx].as_mut() {
                root = root.children[widx].as_mut().unwrap();
            } else {
                let node = Box::new(Node::new());
                root.children[widx] = Some(node);
                root = root.children[widx].as_mut().unwrap();
            }
        }
        root.end = true;
    }
    
    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        let mut root = &self.root;
        for widx in word.bytes().map(|c| (c - b'a') as usize) {
            if let Some(next) = root.children[widx].as_ref() {
                root = next;
            } else {
                return false;
            }
        }
        root.end == true
    }
    
    
    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        let mut root = &self.root;
        for widx in prefix.bytes().map(|c| (c - b'a') as usize) {
            if let Some(next) = root.children[widx].as_ref() {
                root = next;
            } else {
                return false;
            }
        }
        return true
    }
}
