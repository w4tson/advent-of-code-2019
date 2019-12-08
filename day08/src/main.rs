use std::fs::read_to_string;
use itertools::Itertools;

fn main() {
    let image : Vec<u32> = read_to_string("input.txt")
        .expect("bad input")
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();
    
    let checksum = part1(&image, 25,6);
    eprintln!("checksum = {:#?}", checksum);
}

fn part1(image: &[u32], width: usize, height: usize) -> usize {
    let mut layers: Vec<Vec<u32>> = vec![];
    for layer in &image.iter().chunks(width * height) {
        layers.push(layer.collect::<Vec<_>>().iter().map(|&&x| x).collect());
    }

    eprintln!("found {} layers", layers.len());

    let (layer, _) = layers
        .iter()
        .map(|layer| (layer, count_zeros(layer)))
        .min_by_key(|(_, zeros)| *zeros)
        .expect("No max found");


    count_num(layer, 1) * count_num(layer, 2)
}



fn count_zeros(slice: &[u32]) -> usize {
    count_num(slice, 0)
}

fn count_num(slice: &[u32], num : u32) -> usize {
    slice.iter().filter(|&&x| x== num ).count()
}




