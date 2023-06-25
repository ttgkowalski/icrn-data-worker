use bitvec::prelude::*;
use std::fs;

fn main() {

    let file_path: &str = "input/64mb-dummy-file.bin";
    let content_bytes: Vec<u8> = fs::read(file_path).expect("Should have been able to read the file").to_vec();

    let content_bits: &BitSlice<u8, Msb0> = content_bytes.view_bits::<Msb0>();
    let bitage_lenght: usize = content_bits.len();

    let first_data_chunk_slice: &BitSlice<u8, Msb0> = &content_bits[0 .. (bitage_lenght-1)/2];
    let last_data_chunk_slice: &BitSlice<u8, Msb0> = &content_bits[bitage_lenght/2 .. bitage_lenght];
    
    // It's u8 because it's the smallest size that rust can work. But it's a vector of zeros and ones
    let mut parity_chunk: Vec<u8> = Vec::<u8>::with_capacity(content_bytes.len());


    // for i in 0..bitage_lenght/2 {
    //     if first_data_chunk_slice[i] == last_data_chunk_slice[i]{
    //         parity_chunk.push(0);
    //     } else {
    //         parity_chunk.push(1);
    //     }
    // }

    // println!("###### First data chunk slice( {} bits ) ######\n{}\n", first_data_chunk_slice.len(), first_data_chunk_slice);
    // println!("###### Last data chunk slice( {} bits ) ######\n{}\n", last_data_chunk_slice.len(), last_data_chunk_slice);
    // println!("###### Parity data chunk( {} bits ) ######\n{:?})\n", parity_chunk.len(), parity_chunk);

    println!("File's MD5: {:?}", md5::compute(&content_bytes));
    println!("First chunk's MD5: {:?}", md5::compute(&content_bytes[0 .. content_bytes.len()/2]));
    println!("Last chunk's MD5: {:?}", md5::compute(&content_bytes[content_bytes.len()/2 .. content_bytes.len()]));
    println!("");

    println!("Content Bits:      {}", bitage_lenght);
    println!("Content Bytes:     {}", content_bytes.len());
    // println!("Parity Chunk Bits: {}", parity_chunk.len());
}
