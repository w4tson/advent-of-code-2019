use std::fs::read_to_string;
use itertools::Itertools;

fn main() {
    let data : Vec<u32> = read_to_string("input.txt")
        .expect("bad input")
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();
    
    let image = Image { data, width: 25, height: 6 };
    eprintln!("checksum = {:#?}", image.checksum());

    image.preview();
    
}

struct Image {
    data : Vec<u32>,
    width: usize,
    height: usize
}

impl Image {
    
    pub fn preview(&self) {
        let data = self.decode();
        for y in 0..self.height {
            for x in 0..self.width {
                let i = data[y * self.width + x];
                print!("{}", if i == 0 { " " } else { "x" } );
            }
            println!();
        }
    }
    
    fn decode(&self) -> Vec<u32> {
        let layers = self.get_layers();
        
        (0..(self.width * self.height))
            .map(|pixel|{
                (0..layers.len())
                    .filter(|&layer_index| layers[layer_index][pixel] != 2)
                    .map(|layer_index| layers[layer_index][pixel] )
                    .next()
                    .unwrap_or(2)
            })
            .collect()
    }
    
    fn get_layers(&self) -> Vec<Vec<u32>> {
        let mut layers: Vec<Vec<u32>> = vec![];
        for layer in &self.data.iter().chunks(self.width * self.height) {
            layers.push(layer.collect::<Vec<_>>().iter().map(|&&x| x).collect());
        }
        layers
    }
    
    pub fn checksum(&self) -> usize {
        self.get_layers()
            .iter()
            .map(|layer| (layer, Image::count_zeros(layer)))
            .min_by_key(|(_, zeros)| *zeros)
            .map(|(layer,_)| Image::count_num(layer, 1) * Image::count_num(layer, 2))
            .expect("No max found")
    }

    fn count_zeros(slice: &[u32]) -> usize {
        Image::count_num(slice, 0)
    }

    fn count_num(slice: &[u32], num : u32) -> usize {
        slice.iter().filter(|&&x| x== num ).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test() {
        let data: Vec<u32> = vec![
            0,2,2,2,
            1,1,2,2,
            2,2,1,2,
            0,0,0,0
        
        ];
        let image = Image { data, width: 2, height: 2 };
        let decoded = image.decode();
        
        assert_eq!(decoded[0], 0);
        assert_eq!(decoded[1], 1);
        assert_eq!(decoded[2], 1);
        assert_eq!(decoded[3], 0);
    }
}







