/*
 * @lc app=leetcode id=211 lang=rust
 *
 * [211] Add and Search Word - Data structure design
 *
 * https://leetcode.com/problems/add-and-search-word-data-structure-design/description/
 *
 * algorithms
 * Medium (31.07%)
 * Total Accepted:    122.8K
 * Total Submissions: 393.4K
 * Testcase Example:  '["WordDictionary","addWord","addWord","addWord","search","search","search","search"]\n[[],["bad"],["dad"],["mad"],["pad"],["bad"],[".ad"],["b.."]]'
 *
 * Design a data structure that supports the following two operations:
 * 
 * 
 * void addWord(word)
 * bool search(word)
 * 
 * 
 * search(word) can search a literal word or a regular expression string
 * containing only letters a-z or .. A . means it can represent any one
 * letter.
 * 
 * Example:
 * 
 * 
 * addWord("bad")
 * addWord("dad")
 * addWord("mad")
 * search("pad") -> false
 * search("bad") -> true
 * search(".ad") -> true
 * search("b..") -> true
 * 
 * 
 * Note:
 * You may assume that all words are consist of lowercase letters a-z.
 * 
 */
use std::borrow::BorrowMut;
use std::borrow::Borrow;

struct WordDictionary {
    root: Trie,
}

#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    end: bool,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            root: Default::default()
        }
    }
    
    /** Adds a word into the data structure. */
    fn add_word(&mut self, word: String) {
        let mut root = &mut self.root;
        for idx in word.bytes().map(|c| (c-b'a') as usize) {
            if root.children[idx].is_none() {
                root.children[idx] = Some(Default::default());
            }
            root = root.children[idx].as_mut().unwrap().borrow_mut();
        }
        root.end = true;
    }
    
    /** Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter. */
    fn search(&mut self, word: String) -> bool {
        self._search(&self.root, &word[..])
    }

    fn _search(&self, root: &Trie, word: &str) -> bool {
        if word.is_empty() {
            return if root.end { true } else { false };
        }
        let mut root = root;
        for (i, cb) in word.bytes().enumerate() {
            if cb == b'.' {
                return root.children.iter().any(
                    | c| {
                        if let Some(ref boxed_root) = c {
                            let root = boxed_root.borrow();

                            return self._search(root,&word[i+1..]);
                        }
                        false
                    });
            } else {
                let idx = (cb - b'a') as usize;
                if root.children[idx].is_none() {
                    return false
                }
                root = root.children[idx].as_ref().unwrap().borrow();
            }
        }
        root.end
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */
#[cfg(test)]
mod test {
    //use crate::leetcode_test;
    use super::WordDictionary;

    #[test]
    fn test1() {

        // leetcode_test!(
        //     ["WordDictionary","addWord","addWord","addWord","search","search","search","search"]
        //     [[],["bad"],["dad"],["mad"],["pad"],["bad"],[".ad"],["b.."]]
        //     [null, null, null, null, false, true, true, true]
        // );


        
        let mut wd = WordDictionary::new();
        wd.add_word("bad".to_string());
        //wd.search("bad".to_string());
        wd.search(".ad".to_string());
        

    }
}
