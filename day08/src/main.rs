use std::fs::read_to_string;
use itertools::Itertools;

fn main() {
    let data : Vec<u32> = read_to_string("input.txt")
        .expect("bad input")
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();
    
    let image = Image{ data: data, width: 25, height: 6};
    eprintln!("checksum = {:#?}", image.checksum());
}

struct Image {
    data : Vec<u32>,
    width: usize,
    height: usize
}

impl Image {
    fn get_layers(&self) -> Vec<Vec<u32>> {
        let mut layers: Vec<Vec<u32>> = vec![];
        for layer in &self.data.iter().chunks(self.width * self.height) {
            layers.push(layer.collect::<Vec<_>>().iter().map(|&&x| x).collect());
        }
        layers
    }
    
    pub fn checksum(&self) -> usize {
        let layers = self.get_layers();

        let (layer, _) = layers
            .iter()
            .map(|layer| (layer, Image::count_zeros(layer)))
            .min_by_key(|(_, zeros)| *zeros)
            .expect("No max found");


        Image::count_num(layer, 1) * Image::count_num(layer, 2)
    }

    fn count_zeros(slice: &[u32]) -> usize {
        Image::count_num(slice, 0)
    }

    fn count_num(slice: &[u32], num : u32) -> usize {
        slice.iter().filter(|&&x| x== num ).count()
    }
}







