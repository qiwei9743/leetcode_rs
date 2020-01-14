/*
 * @lc app=leetcode id=981 lang=rust
 *
 * [981] Time Based Key-Value Store
 *
 * https://leetcode.com/problems/time-based-key-value-store/description/
 *
 * algorithms
 * Medium (48.06%)
 * Total Accepted:    3.4K
 * Total Submissions: 7K
 * Testcase Example:  '["TimeMap","set","get","get","set","get","get"]\n[[],["foo","bar",1],["foo",1],["foo",3],["foo","bar2",4],["foo",4],["foo",5]]'
 *
 * Create a timebased key-value store class TimeMap, that supports two
 * operations.
 * 
 * 1. set(string key, string value, int timestamp)
 * 
 * 
 * Stores the key and value, along with the given timestamp.
 * 
 * 
 * 2. get(string key, int timestamp)
 * 
 * 
 * Returns a value such that set(key, value, timestamp_prev) was called
 * previously, with timestamp_prev <= timestamp.
 * If there are multiple such values, it returns the one with the largest
 * timestamp_prev.
 * If there are no values, it returns the empty string ("").
 * 
 * 
 * 
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: inputs = ["TimeMap","set","get","get","set","get","get"], inputs =
 * [[],["foo","bar",1],["foo",1],["foo",3],["foo","bar2",4],["foo",4],["foo",5]]
 * Output: [null,null,"bar","bar",null,"bar2","bar2"]
 * Explanation:   
 * TimeMap kv;   
 * kv.set("foo", "bar", 1); // store the key "foo" and value "bar" along with
 * timestamp = 1   
 * kv.get("foo", 1);  // output "bar"   
 * kv.get("foo", 3); // output "bar" since there is no value corresponding to
 * foo at timestamp 3 and timestamp 2, then the only value is at timestamp 1 ie
 * "bar"   
 * kv.set("foo", "bar2", 4);   
 * kv.get("foo", 4); // output "bar2"   
 * kv.get("foo", 5); //output "bar2"   
 * 
 * 
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: inputs = ["TimeMap","set","set","get","get","get","get","get"],
 * inputs =
 * [[],["love","high",10],["love","low",20],["love",5],["love",10],["love",15],["love",20],["love",25]]
 * Output: [null,null,null,"","high","high","low","low"]
 * 
 * 
 * 
 * 
 * 
 * 
 * Note:
 * 
 * 
 * All key/value strings are lowercase.
 * All key/value strings have length in the range [1, 100]
 * The timestamps for all TimeMap.set operations are strictly increasing.
 * 1 <= timestamp <= 10^7
 * TimeMap.set and TimeMap.get functions will be called a total of 120000 times
 * (combined) per test case.
 * 
 */
struct TimeMap {
    tree: std::collections::BTreeMap<String, Vec<(i32, String)>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            tree: std::collections::BTreeMap::new()
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.tree.entry(key).or_insert_with(|| vec![]).push((timestamp,value))
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        self.tree.get(&key).map(|v| {
            match v.binary_search_by_key(&timestamp, |x| x.0) {
                Ok(p) => v[p].1.clone(),
                Err(p) => if p > 0 { v[p-1].1.clone() } else { String::from("") }
            }
        }).unwrap_or(String::from(""))
    }

    fn get2(&self, key: String, timestamp: i32) -> String {
        self.tree.get(&key).and_then(|v| {
            Self::lessthan_or_equal(v, timestamp)
        }).unwrap_or(String::from(""))
    }
    fn lessthan_or_equal(arr: &[(i32, String)], timestamp: i32) -> Option<String> {
        let (mut low, mut high) = (0, arr.len());
        while low < high {
            let mid = low + (high - low) / 2;
            if arr[mid].0 > timestamp {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        if low > 0 { Some(arr[low-1].1.clone()) } else { None }
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */
