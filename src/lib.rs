
//#![feature(proc_macro_hygiene)]
pub use leetcode_prelude::{linkedlist, btree, leetcode_test, vec_string, ListNode, TreeNode, assert_eq_sorted};

pub mod string_to_integer_atoi;
pub mod add_and_search_word_data_structure_design;
pub mod add_binary;
pub mod add_digits;
pub mod add_strings;
pub mod add_two_numbers;
pub mod all_possible_full_binary_trees;
pub mod binary_tree_level_order_traversal;
pub mod binary_tree_level_order_traversal_ii;
pub mod climbing_stairs;
pub mod count_and_say;
pub mod count_primes;
pub mod counting_bits;
pub mod design_hashmap;
pub mod design_hashset;
pub mod design_linked_list;
pub mod symmetric_tree;
pub mod serialize_and_deserialize_bst;
pub mod excel_sheet_column_title;
pub mod evaluate_reverse_polish_notation;
pub mod find_first_and_last_position_of_element_in_sorted_array;
pub mod generate_parentheses;
pub mod group_anagrams;
pub mod implement_stack_using_queues;
pub mod implement_queue_using_stacks;
pub mod implement_trie_prefix_tree;
pub mod integer_to_roman;
pub mod roman_to_integer;
pub mod isomorphic_strings;
pub mod excel_sheet_column_number;
pub mod largest_number;
pub mod flatten_binary_tree_to_linked_list;
pub mod longest_common_prefix;
pub mod length_of_last_word;
pub mod longest_palindromic_substring;
pub mod fraction_to_recurring_decimal;
pub mod kth_largest_element_in_an_array;
pub mod longest_substring_with_at_least_k_repeating_characters;
pub mod longest_palindromic_subsequence;
pub mod best_time_to_buy_and_sell_stock;
pub mod best_time_to_buy_and_sell_stock_ii;
pub mod jewels_and_stones;
pub mod maximum_depth_of_binary_tree;
pub mod merge_k_sorted_lists;
pub mod merge_two_sorted_lists;
pub mod maximum_subarray;
pub mod min_stack;
pub mod merge_sorted_array;
pub mod majority_element;
pub mod majority_element_ii;
pub mod palindrome_number;
pub mod pascals_triangle;
pub mod pascals_triangle_ii;
pub mod power_of_two;
pub mod plus_one;
pub mod powx_n;
pub mod permutations;
pub mod permutations_ii;
pub mod remove_nth_node_from_end_of_list;
pub mod reverse_integer;
pub mod remove_linked_list_elements;
pub mod remove_duplicates_from_sorted_array;
pub mod remove_element;
pub mod remove_duplicates_from_sorted_list;
pub mod remove_duplicates_from_sorted_list_ii;
pub mod remove_duplicates_from_sorted_array_ii;
pub mod rotate_image;
pub mod reverse_nodes_in_k_group;
pub mod reverse_string;
pub mod reverse_string_ii;
pub mod rotate_list;
pub mod non_decreasing_array;
pub mod two_sum;
pub mod zigzag_conversion;
pub mod valid_parentheses;
pub mod swap_nodes_in_pairs;
pub mod search_insert_position;
pub mod valid_sudoku;
pub mod sudoku_solver;
pub mod sqrtx;
pub mod wildcard_matching;
pub mod subsets;
pub mod same_tree;
pub mod valid_palindrome;
pub mod single_number;
pub mod single_number_ii;
pub mod two_sum_ii_input_array_is_sorted;
pub mod ugly_number;
pub mod ugly_number_ii;
pub mod super_ugly_number;
pub mod valid_palindrome_ii;
pub mod to_lower_case;
pub mod self_dividing_numbers;
pub mod unique_morse_code_words;
pub mod sort_array_by_parity;
pub mod sort_array_by_parity_ii;
pub mod water_and_jug_problem;
pub mod find_common_characters;
pub mod maximum_profit_in_job_scheduling;
pub mod unique_email_addresses;
pub mod ugly_number_iii;
pub mod maximum_side_length_of_a_square_with_sum_less_than_or_equal_to_threshold;


pub mod range_sum_query_mutable;

// union-find
pub mod redundant_connection;
//pub mod redundant_connection_ii;
pub mod accounts_merge;

//tree traversal
pub mod binary_tree_postorder_traversal;
pub mod binary_tree_inorder_traversal;
pub mod binary_tree_preorder_traversal;


// seg tree && binary indexed tree
pub mod count_of_smaller_numbers_after_self;
// pub mod count_of_range_sum; *hard* // https://www.hrwhisper.me/leetcode-count-of-range-sum/


// can not bug free
// 146 721

// dijkstra  // https://www.youtube.com/watch?v=9wV1VxlfBlI
// 743
// bellman-ford
// 787  https://www.youtube.com/watch?v=PLY-lbcxEjg


// unknown order
pub mod validate_binary_search_tree;
pub mod unique_binary_search_trees;
// 95.unique-binary-search-trees-ii.py
pub mod online_election;
pub mod koko_eating_bananas;
pub mod peak_index_in_a_mountain_array;
pub mod search_in_rotated_sorted_array_ii;
// 787.cheapest-flights-within-k-stops.py
pub mod find_smallest_letter_greater_than_target;
// 743.network-delay-time.py
// 721.accounts-merge.py
pub mod arranging_coins;
pub mod median_of_two_sorted_arrays;
pub mod intersection_of_two_arrays;
pub mod intersection_of_two_arrays_ii;
pub mod divide_two_integers;
pub mod h_index_ii;
pub mod count_complete_tree_nodes;
// 222.count-complete-tree-nodes.py
// 146.lru-cache.py
pub mod find_the_smallest_divisor_given_a_threshold;
// 1237.find-positive-integer-solution-for-a-given-equation.py
pub mod shortest_path_in_binary_matrix;

// union find
// 959.regions-cut-by-slashes.py


// tree
pub mod construct_binary_tree_from_preorder_and_inorder_traversal;
pub mod construct_binary_tree_from_inorder_and_postorder_traversal;
pub mod construct_binary_tree_from_preorder_and_postorder_traversal;
pub mod convert_sorted_array_to_binary_search_tree;
pub mod balanced_binary_tree;
pub mod minimum_depth_of_binary_tree;
pub mod binary_search;
// 116.populating-next-right-pointers-in-each-node.py failed
// 117.populating-next-right-pointers-in-each-node-ii.py


// binary search
pub mod kth_smallest_element_in_a_sorted_matrix;
pub mod search_in_rotated_sorted_array;
pub mod capacity_to_ship_packages_within_d_days;
// 278.first-bad-version.py;

// todo
// pub mod minimum_ascii_delete_sum_for_two_strings;
// pub mod redundant_connection_ii;
// pub mod h_index;
// pub mod lru-cache;
// 528.random-pick-with-weight.rs


// dfs
// 133.clone-graph.py
// 130.surrounded-regions.py
// 129.sum-root-to-leaf-numbers.py
pub mod convert_sorted_list_to_binary_search_tree;
