use bitboard::{Square, BitBoard, Rank, File};
use serde_json;

fn main() {
    // Demonstrate Square serialization
    let square_a1 = Square::make_square(Rank::First, File::A);
    let serialized_a1 = serde_json::to_string(&square_a1).unwrap();
    println!("Square A1 serialized: {}", serialized_a1);
    
    let deserialized_a1: Square = serde_json::from_str(&serialized_a1).unwrap();
    println!("Square A1 deserialized: {:?}", deserialized_a1);
    
    // Demonstrate BitBoard serialization
    let bitboard = BitBoard::from_square(Square::A1) | 
                   BitBoard::from_square(Square::C3) | 
                   BitBoard::from_square(Square::H8);
    
    let serialized_bb = serde_json::to_string(&bitboard).unwrap();
    println!("BitBoard serialized: {}", serialized_bb);
    
    let deserialized_bb: BitBoard = serde_json::from_str(&serialized_bb).unwrap();
    println!("BitBoard deserialized contains A1: {}", deserialized_bb.contains(Square::A1));
    println!("BitBoard deserialized contains C3: {}", deserialized_bb.contains(Square::C3));
    println!("BitBoard deserialized contains H8: {}", deserialized_bb.contains(Square::H8));
    println!("BitBoard deserialized length: {}", deserialized_bb.len());
}
