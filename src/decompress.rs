use std::collections::HashMap;
use crate::bit_stream::BitReader;

pub fn decompress(mut compressed: BitReader) -> String {
    //Decode tree
    let mut cache_decode: Box<HashMap<Vec<bool>, char>> = Box::new(HashMap::new());

    // tree_struct_decode(&mut compressed, &mut cache_decode);
    hash_map_decode(&mut compressed, &mut cache_decode);
    // branch_length_decode(&mut compressed, &mut cache_decode);

    println!("cache_decode {:?}", cache_decode);
    println!("Reading encoded at bit pos {}", compressed.bit_position);

    //Reconstruct message per tree

    let mut current: Vec<bool> = Vec::with_capacity(10);
    let mut output = String::new();

    if let Some(characters_len) = compressed.read_u16() {

        let mut i = 0;
        while i < characters_len {

            if let Some(bit) = compressed.read_bit() {
                current.push(bit);

                if let Some(decoded_char) = cache_decode.get(&current) {
                    current.clear();
                    output.push(*decoded_char);
                    i+=1;
                }

            }else{
                unreachable!()
            }

        }
    } else {
        unreachable!()
    }

    output
}



fn tree_struct_decode(struct_reader: &mut BitReader, mut cache_decode: &mut Box<HashMap<Vec<bool>, char>>) {
    if let Some(ordered_chars_len) = struct_reader.read_u16() {
        let mut chars_reader = BitReader {
            buffer: struct_reader.buffer.clone(),
            bit_position: struct_reader.bit_position,
        };
        struct_reader.progress((8 * ordered_chars_len) as usize);


        fn recurse(structure_reader: &mut BitReader, character_reader: &mut BitReader, cache_decode: &mut Box<HashMap<Vec<bool>, char>>, location: Vec<bool>) {
            if let Some(bit) = structure_reader.read_bit() {
                if bit {
                    //Branch, recurse with LEFT and with RIGHT

                    let mut left_location = location.clone();
                    left_location.push(false);
                    recurse(structure_reader, character_reader, cache_decode, left_location);

                    let mut right_location = location.clone();
                    right_location.push(true);
                    recurse(structure_reader, character_reader, cache_decode, right_location);


                }else{
                    //Value

                    if let Some(char_u8) = character_reader.read_u8() {
                        cache_decode.insert(location, char_u8 as char);
                    }else{
                        unreachable!()
                    }
                    //Quite cleverly we also stop recursing naturally when we reach the end of the tree because Value's dont call the recurse function
                }
            }else{
                unreachable!()
            }
        }
        recurse(struct_reader, &mut chars_reader, &mut cache_decode, Vec::new());
    } else {
        unreachable!()
    }
}
fn hash_map_decode(compressed: &mut BitReader, mut cache_decode: &mut Box<HashMap<Vec<bool>, char>>) {
    //Read number of key-value pairs
    if let Some(cache_len) = compressed.read_u16() {
        if let Some(highest_depth) = compressed.read_u16() {

            for _ in 0..cache_len { //For each key-value pair
                //Read vec-bool, then refine it understanding that farthest left positive bit is the terminator
                let mut vec_bool: Vec<bool> = Vec::with_capacity((highest_depth) as usize);
                let mut reached_terminator = false;

                for _ in 0..highest_depth+1 {
                    if let Some(bit) = compressed.read_bit() {
                        if reached_terminator {
                            vec_bool.push(bit);
                        }else if bit {
                            reached_terminator = true;
                        }
                    }else{
                        unreachable!()
                    }
                }

                //Read u8 char
                if let Some(char_u8) = compressed.read_u8() {
                    cache_decode.insert(vec_bool, char_u8 as char);
                }else{
                    unreachable!()
                }
            }

        }else{
            unreachable!()
        }
    }else{
        unreachable!()
    }
}
fn branch_length_decode() {

}

