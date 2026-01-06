use std::{collections::HashMap, iter::zip, ops::Div};
mod operations;

pub type GrayNumber = u32;
pub type BinaryCord = usize;
use OutputCases::*;

use crate::operations::{boolean_to_cords::bool_to_cords};
/// | inputs  | output     |
/// |---------|------------|
/// | xxxxxxx | 1 care     |
/// | xxxxxxx | x DontCare |
#[derive(Debug)]
#[derive(Clone)]
pub enum OutputCases {
    Care(bool),
    DontCare
}
///
/// #TruthTable
/// 
/// should be like this 
/// | Inputs    | Outputs   |
/// |-----------|-----------|
/// | 0 0 0 0   | 0 0 1 0   |
/// | 0 0 0 1   | 0 1 0 1   |
/// | 0 0 1 0   | 0 1 1 0   |
/// | 0 0 1 1   | 1 0 0 1   |
/// | 0 1 0 0   | 1 0 1 0   |
/// | 0 1 0 1   | 1 1 0 1   |
/// 
/// this truth struct table is any size
///
/// | Inputs                | Output |
/// |-----------------------|--------|
/// | 0 0 0 0 0 0 0 0       | 0      |
/// | 0 0 0 0 0 0 0 1       | 1      |
/// | 0 0 0 0 0 0 1 0       | 1      |
/// | 0 0 0 0 0 0 1 1       | 0      |
/// | 0 0 0 0 0 1 0 0       | 1      |
/// | 0 0 0 0 0 1 0 1       | 0      |
#[derive(Debug)]
pub struct TruthTable<const I: usize,const O : usize> {
    table: HashMap<[bool;I],[OutputCases ;O]>,
    pub i_size: u8,
    pub o_size: u8,
}

impl <const I: usize,const O : usize> TruthTable<I,O> {
    pub fn new() -> Self{
        TruthTable { 
            table: HashMap::new(),
            i_size: I as u8,
            o_size: O as u8
        }
    }
    pub fn insert(&mut self,i: [bool;I],o: [OutputCases;O]){
        self.table.insert(i, o);
    }

    pub fn as_maps(&self) -> Vec<KarnaugthMap> {
        let mut allmaps = Vec::new();
        
        //this line add N maps depending of TruthTable Outputs
        for _ in 0..O {
            allmaps.push(KarnaugthMap::new_map(I));
        }
        

        for (map, out_idx) in zip(allmaps.iter_mut(),0..O) {
            for inputs in self.table.keys() {
                //rowcords and columns cords as binary for insertion
                let (row_cord,column_cord) = bool_to_cords(inputs);

                let outputs = self.table.get(inputs).expect("error in updating");
                
                map.update(row_cord, column_cord, outputs[out_idx].clone());
            }
        }
        allmaps
    }
}

///
/// This a karnaught map struct
/// the map could be small or big
/// the rows and columns use gray numbers as identifier
/// 
/// #SMALL
/// 
/// | **ab/cd**  |**00**|**01**|**11**|**10**|
/// |------------|------|------|------|------|
/// | **00**     |   0  |   0  |  0   |  0   |
/// | **01**     |   0  |   0  |  0   |  0   |
/// | **11**     |   0  |   0  |  0   |  0   |
/// | **10**     |   0  |   0  |  0   |  0   |
/// 
/// #BIG
/// | **abc\def** | 000 | 001 | 011 | 010 | 110 | 111 | 101 | 100 |
/// |-------------|-----|-----|-----|-----|-----|-----|-----|-----|
/// | **000**     | 0   | 0   | 0   | 0   | 0   | 0   | 0   | 0   |
/// | **001**     | 0   | 0   | 0   | 0   | 0   | 0   | 0   | 0   |
/// | **011**     | 0   | 0   | 0   | 0   | 0   | 0   | 0   | 0   |
/// | **010**     | 0   | 0   | 0   | 0   | 0   | 0   | 0   | 0   |
/// | **110**     | 0   | 0   | 0   | 0   | 0   | 0   | 0   | 0   |
/// | **111**     | 0   | 0   | 0   | 0   | 0   | 0   | 0   | 0   |
/// | **101**     | 0   | 0   | 0   | 0   | 0   | 0   | 0   | 0   |
/// | **100**     | 0   | 0   | 0   | 0   | 0   | 0   | 0   | 0   |
/// 
/// #IRREGULAR
/// 
/// | **abc\de** | 00  | 01  | 11  | 10  | 00  | 01  | 11  | 10  |
/// |------------|-----|-----|-----|-----|-----|-----|-----|-----|
/// | **000**    | 0   | 0   | 0   | 0   | 0   | 0   | 0   | 0   |
/// | **001**    | 0   | 0   | 0   | 0   | 0   | 0   | 0   | 0   |
/// | **011**    | 0   | 0   | 0   | 0   | 0   | 0   | 0   | 0   |
/// | **010**    | 0   | 0   | 0   | 0   | 0   | 0   | 0   | 0   |
/// | **110**    | 0   | 0   | 0   | 0   | 0   | 0   | 0   | 0   |
/// | **111**    | 0   | 0   | 0   | 0   | 0   | 0   | 0   | 0   |
/// | **101**    | 0   | 0   | 0   | 0   | 0   | 0   | 0   | 0   |
/// | **100**    | 0   | 0   | 0   | 0   | 0   | 0   | 0   | 0   |
pub struct KarnaugthMap{
    /// no se modifica
    matriz: Vec<Vec<OutputCases>>
}

impl KarnaugthMap {
    pub fn solve_as_str() -> String {
        panic!("not implemented");
    }
    ///
    ///
    /// This function make a new map by default by DontCare cases
    /// 
    /// |  |00|01|11|10|
    /// |--|--|--|--|--|
    /// |00|x |x |x |x |
    /// |01|x |x |x |x |
    /// |11|x |x |x |x |
    /// |10|x |x |x |x |
    /// 
    pub fn new_map(input_size: usize) -> Self{
        let rows = input_size.div(2) << 1;
        let colum = input_size.div_ceil(2) << 1;
        let mut matriz = Vec::new();
        for _ in 0..rows {
            let mut vector = Vec::new();
            for _ in 0..colum {
                vector.push(DontCare);
            }
            matriz.push(vector);
        }
        KarnaugthMap { matriz: matriz }
    }

    pub fn printtable(&self) {

        for i in self.matriz.iter() {
            for j in i.iter() {
                match j {
                    DontCare => print!("x"),
                    Care(value) => match value {
                        true => print!("1"),
                        false => print!("0")
                    }
                }
                print!("|")
            }
            println!("");
        }
    }

    /// !Caution
    /// this function isn't protected by outrange
    /// this function change the status
    fn update(&mut self,row_cord: usize, column_cord: usize,caso: OutputCases) {
        self.matriz[row_cord][column_cord] = caso;

    }
}

