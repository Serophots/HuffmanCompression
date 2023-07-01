use std::collections::HashMap;
use crate::bit_stream::BitWriter;
use crate::huffman_types::{HuffmanNode, HuffmanBranch, HuffmanValue};

pub fn compress(input: String, output_stream: &mut Box<BitWriter>) {

    //Count the frequency of each character
    let mut huffman_tree: Box<Vec<HuffmanNode>> = Box::new(Vec::new());

    for char in input.chars() {

        if let Some(node) = (*huffman_tree).iter_mut().find(|i| {
            if let HuffmanNode::Value(huffman_value) = i {
                return huffman_value.char == char
            }else{
                unreachable!()
            }
        }) {
            //Already found the character in the list - increase its frequency
            if let HuffmanNode::Value(counted_character) = node {
                counted_character.count += 1;
            } else {
                unreachable!()
            }
        } else {
            //Character is not in the list - insert it
            (*huffman_tree).push(HuffmanNode::Value(HuffmanValue {
                char,
                count: 1,
            }));
        }
    }

    //Sort the frequencies by descending frequency
    (*huffman_tree).sort_by(|node_a, node_b| {
        if let HuffmanNode::Value(a) = node_a {
            if let HuffmanNode::Value(b) = node_b {
                return b.count.cmp(&a.count);
            }
        }
        unreachable!()
    });

    //'Compress' HuffmanValues into HuffmanBranches of the tree
    while huffman_tree.len() > 1 {
        //Get first and second items in the list which are currently least frequent
        if let Some(first) = (*huffman_tree).pop() {
            if let Some(second) = (*huffman_tree).pop() {
                //Combine them into a new HuffmanNode::Branch
                let count = first.count_sub_branches() + second.count_sub_branches();
                let new = HuffmanNode::Branch(Box::new(HuffmanBranch {
                    t: Box::new(first),
                    f: Box::new(second),
                    count
                }));


                //Insert this into the ordered vec at the correct spot
                let insert_pos = match (*huffman_tree).binary_search_by(|compare| count.cmp(&compare.count_sub_branches())) {
                    Ok(pos) => pos,
                    Err(pos) => pos
                };

                huffman_tree.insert(insert_pos, new);
            }else{
                unreachable!()
            }
        }else{
            unreachable!()
        }
    }
    //Tree complete!

    //Get the master branch
    if let Some(master_node) = huffman_tree.get(0) {
        //Traverse the entire tree creating a memory cache hashmap of character to compressed binary representation
        let mut cache_encode: HashMap<char, Vec<bool>> = HashMap::new();
        let mut highest_depth: Box<u64> = Box::new(0);

        fn recurse_node(node: &HuffmanNode, cache_encode: &mut HashMap<char, Vec<bool>>, branch_path: Vec<bool>, depth: u64, highest_depth: &mut Box<u64>) {
            match node {
                HuffmanNode::Branch(branch) => {
                    //Recurse on left branch
                    let mut left = branch_path.clone();
                    left.push(false);
                    recurse_node(&*branch.t, cache_encode, left, depth+1, highest_depth);

                    //Recurse on right branch
                    let mut right = branch_path.clone();
                    right.push(true);
                    recurse_node(&*branch.f, cache_encode, right, depth+1, highest_depth);
                }
                HuffmanNode::Value(value) => {
                    //Found a character - update highest_depth & insert into cache_encode
                    if depth > **highest_depth {**highest_depth = depth}

                    cache_encode.insert(value.char, branch_path);
                },
                _ => unreachable!()
            }
        }
        recurse_node(master_node, &mut cache_encode, Vec::new(), 0, &mut highest_depth);

        //Encode the tree using 1 of the 3 methods
        if true == true {
            tree_struct_encode(output_stream, master_node);

        } else if true == false {
            hash_map_encode(output_stream, &cache_encode, *highest_depth);

        } else if true == false {
            branch_length_encode(output_stream, master_node, *highest_depth);

        }


        //Write compressed versions of each character to output_stream (as per cache_encode)
        output_stream.write_u16(input.len() as u16);
        for char in input.chars() {
            if let Some(encoded_char) = cache_encode.get(&char) {
                output_stream.write_bits_vec(encoded_char)

            } else {
                unreachable!();
            }
        }

        output_stream.print();
    }
}

fn tree_struct_encode(output_stream: &mut Box<BitWriter>, tree: &HuffmanNode) {
    //Extract character data to be written (writing it later to preserve order)
    let mut ordered_characters: Vec<char> = Vec::new();
    let mut tree_structure: Vec<bool> = Vec::new();

    fn recurse_node(node: &HuffmanNode, ordered_characters: &mut Vec<char>, tree_structure: &mut Vec<bool>) {
        match node {
            HuffmanNode::Branch(branch) => {
                tree_structure.push(true);
                recurse_node(&branch.t, ordered_characters, tree_structure);
                recurse_node(&branch.f, ordered_characters, tree_structure);
            },
            HuffmanNode::Value(value) => {
                tree_structure.push(false);
                ordered_characters.push(value.char);
            },
            _ => unreachable!()
        }
    }
    recurse_node(tree, &mut ordered_characters, &mut tree_structure);

    // Write ordered_characters
    output_stream.write_u16(ordered_characters.len() as u16); //TODO: Adapt u8/16/32/... per input text
    for character in ordered_characters {
        output_stream.write_u8(character as u8)
    }

    // Write tree structure - No need to store the length of this data segment - the decoder will naturally know when the tree is full :)
    output_stream.write_bits_vec(&tree_structure);
}
fn hash_map_encode(output_stream: &mut Box<BitWriter>, cache_encode: &HashMap<char, Vec<bool>>, highest_depth: u64) {
    //Write number of key-value pairs
    output_stream.write_u16(cache_encode.len() as u16);
    //Write highest tree-depth = single-width for vec<bool>s
    output_stream.write_u16(highest_depth as u16);

    for (char, vec_bool) in cache_encode {
        //Doing the vec_bool backward

        //Pad it
        for _ in 0..(highest_depth as usize - vec_bool.len()) {
            output_stream.write_bit(false);
        }

        //Add terminator bit
        output_stream.write_bit(true);

        //Write vec_bool (also backward)
        for bool in vec_bool.iter() {
            output_stream.write_bit(*bool);
        }


        //Write char
        output_stream.write_u8(*char as u8);

    }
}
fn branch_length_encode(output_stream: &mut Box<BitWriter>, tree: &HuffmanNode, highest_depth: u64) {
    let mut ordered_characters: Vec<char> = Vec::new();
    let mut ordered_depths: Vec<i32> = Vec::new(); //List of RELATIVE, SIGNED depths. In output, designate 1 bit for determining between a u7 and a u12, or some such thing

    fn recurse_node(node: &HuffmanNode, output_stream: &mut Box<BitWriter>, ordered_characters: &mut Vec<char>, depth: u64) {
        match node {
            HuffmanNode::Branch(branch) => {
                recurse_node(&branch.t, output_stream, ordered_characters, depth + 1);
                recurse_node(&branch.f, output_stream, ordered_characters, depth + 1);
            },
            HuffmanNode::Value(value) => {

                ordered_characters.push(value.char);

            },
            _ => unreachable!()
        }
    }

    recurse_node(tree, output_stream, &mut ordered_characters, 0);

}
