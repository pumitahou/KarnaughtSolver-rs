use crate::{BinaryCord, GrayNumber};

#[allow(unused)]
pub fn gray(n: u32) -> GrayNumber{
    n ^ (n >> 1)
}

pub fn gray_to_binary(mut n: GrayNumber) -> BinaryCord {
    let mut mask = n >> 1;
    while mask != 0 {
        n ^= mask;
        mask >>= 1;
    }
    n as BinaryCord
}
#[test]
#[ignore]
fn test_gray(){
    for i in 0..(4<<1) as u32 {
        println!("{:03b} -> {:03b}",i,gray(i))
    } 
}