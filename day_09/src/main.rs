use std::fmt::Display;
use std::ops::Div;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum NumericBlockType {
    Space,
    Data(u32)
}

impl Display for NumericBlockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumericBlockType::Space => {write!(f, ".")}
            NumericBlockType::Data(k) => {write!(f, "({})", k)}
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct Block {
    pub(crate) block_type: NumericBlockType,
    pub(crate) start: u32,
    pub(crate) length: u32
}

fn main() {
    let data = std::fs::read_to_string("day_09/src/input.txt").unwrap();
    // let data = "2333133121414131402";

    let mut data_blocks: Vec<Block> = Vec::new();
    let mut space_blocks: Vec<Block> = Vec::new();
    let mut offset: u32 = 0;

    for (i, chunk) in data.trim().chars().enumerate() {
        let n: u32 = chunk.to_digit(10).unwrap();

        if i % 2 == 0 {
            data_blocks.push(Block{
                block_type: NumericBlockType::Data(i.div(2) as u32),
                start: offset,
                length: n,
            })
        } else {
            space_blocks.push(Block{
                block_type: NumericBlockType::Space,
                start: offset,
                length: n,
            })
        }

        offset += n;
    }

    let missing = data_blocks.len() - space_blocks.len();
    for _ in 0..missing {
        space_blocks.push(Block {
            block_type: NumericBlockType::Space,
            start: offset,
            length: 0
        })
    }

    let mut disk: Vec<NumericBlockType> = Vec::new();
    for i in 0..data_blocks.len() {
        for _ in 0..data_blocks[i].length {
            disk.push(data_blocks[i].block_type);
        }

        for _ in 0..space_blocks[i].length {
            disk.push(space_blocks[i].block_type);
        }
    }

    let mut all_blocks: Vec<Block> = Vec::new();
    for (i, space_block) in space_blocks.iter().enumerate() {
        all_blocks.push(data_blocks[i]);
        all_blocks.push(*space_block);
    }

    // println!("all blocks: {:?}", all_blocks);

    compact_full_files(&mut all_blocks);

    let mut disk: Vec<NumericBlockType> = Vec::new();
    for data in all_blocks {
        for _ in 0..data.length {
            disk.push(data.block_type);
        }
    }

    // println!("this is disk: {:?}", disk);
    let sum = disk_checksum(&disk);
    println!("Sum: {}", sum);
}

fn disk_checksum(disk: &[NumericBlockType]) -> u64 {
    let mut sum = 0;
    for (i, block) in disk.iter().enumerate() {
        match block {
            NumericBlockType::Space => {}
            NumericBlockType::Data(k) => {
                sum += (i as u64) * (*k as u64);
            }
        }
    }

    sum
}

fn compact_full_files(blocks: &mut Vec<Block>) {
    let last_idx = blocks.len()-1;

    // Need this allow, because without declaring the variable, I can't set it in the match
    // following, and without setting a value, rust will complain that it might not be initialised.
    #[allow(unused_assignments)]
    let mut last_id: u32 = 0;

    let last_block_type = blocks.get(last_idx-1).unwrap().block_type;
    match last_block_type {
        NumericBlockType::Space => {
            panic!("len-2 element was a space")
        }
        NumericBlockType::Data(i) => {
            last_id = i;
        }
    }

    if last_id == 0 {
        panic!("last data is 0");
    }


    while last_id > 0 {
        // println!("this is last id: {}", last_id);
        find_space_for_data_id(blocks, last_id);
        // println!("last idx: {}, block: {:?}", last_idx, blocks[last_idx]);
        last_id -= 1;
    }
}

fn find_space_for_data_id(blocks: &mut Vec<Block>, _data_id: u32) {
    blocks.push(Block{
        block_type: NumericBlockType::Space,
        start: 0,
        length: 0,
    });

    // println!("incoming blocks vec is\n{:?}", blocks);

    // filter out all blocks that are length 0, we do not need them.
    blocks.retain(|&b| b.length != 0);
    let mut length_needed = 0;
    let mut data_start = 0;

    // println!("incoming blocks filtered (retained):\n{:?}\n", blocks);
    // println!("starting the search for the next data block to replace. Incoming data ID is {}, so we need to move that, or a data block that has a lower ID", _data_id);

    // find the last data we have with id _data_id.
    let mut data_idx = blocks.len() - 1;
    // println!("starting the search at index {} in the new block", data_idx);
    while data_idx > 0 {
        // println!("-- while loop start at data idx {}", data_idx);
        match blocks[data_idx].block_type {
            NumericBlockType::Space => {
                // println!("-- X block was a space, decrementing idx and continuing");
                data_idx -= 1;
            }
            NumericBlockType::Data(id) => {
                if id != _data_id {
                    // println!("-- P block was data, but its id {} is too big, we're looking for {}. Decrementing and continuing.", id, _data_id);
                    data_idx -= 1;
                    continue;
                }

                // store the length needed for the space, and where this data starts, so we can
                // figure out whether a space is big enough, and whether it's to the left of it.
                length_needed = blocks[data_idx].length;
                data_start = blocks[data_idx].start;

                // println!("found the data block: {:?} at index {}, set length needed to {} and data start to {}",
                //          blocks[data_idx], data_idx, length_needed, data_start );
                break;
            }
        }
    }

    // println!("\nNext up let's find the space that's big enough to take our data");
    for (i, &block) in blocks.iter().enumerate() {
        // println!("-- i {}", i);
        match block.block_type {
            NumericBlockType::Space => {
                // println!("-- Ok: this is a space");
                if block.length < length_needed {
                    // println!("-- X: its length {} is too short for what we need {}", block.length, length_needed);
                    continue;
                }

                if block.start > data_start {
                    // println!("-- X: its start {} is too late for what we need {}, returning", block.start, data_start);
                    return;
                }

                // println!("-- Ok: found the block! {:?}", block);

                let replacement_space = vec![Block {
                    block_type: NumericBlockType::Space,
                    length: length_needed,
                    start: data_start,
                }];

                // println!("created a replacement space: {:?}", replacement_space);
                // println!("about to replace the data block we found with the space. The incoming blocks vec is\n{:?}", blocks);
                // replace the data from the end with an equal sized space
                let _replaced_block = blocks
                    .splice(data_idx..data_idx + 1, replacement_space)
                    .collect::<Vec<Block>>();

                // println!("replaced the data at idx with the replacement space\n{:?}", blocks);

                let moved_data = Block {
                    block_type: NumericBlockType::Data(_data_id),
                    start: block.start,
                    length: length_needed,
                };

                // println!("-- created a new data block for the same data: {:?}", moved_data);

                let diff = block.length - length_needed;
                let extra_space = Block {
                    block_type: NumericBlockType::Space,
                    length: diff,
                    start: block.start + length_needed
                };

                // println!("-- also created an extra space block so we can replace the space fully: {:?}", extra_space);
                // println!("-- for context, need to replace space length {} with data len {} + extra space len {}",
                // block.length, length_needed, diff);

                // println!("this is the vec we're doing a splice on\n{:?}", blocks);
                // println!("it has {} len and we're doing the range {}..{}", blocks.len(), i, i+1);

                let replacement = vec![moved_data, extra_space];

                let _replaced_space = blocks
                    .splice(i..i+1, replacement).collect::<Vec<Block>>();

                // println!("replacement supposedly done, whit is new_blocks?\n{:?}", blocks);
                return;
            }
            NumericBlockType::Data(_) => {
                // println!("-- X block was a data block, we're not messing with it");
                continue;
            }
        }
    }
}