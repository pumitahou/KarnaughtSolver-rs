pub mod boolean_to_cords;
pub mod converters;

/*
ri = real_index in the column or row
ki = karnaught_index in the colum or row

if the test fails is because it can't find the correct cords

|------|-----|-----|-----|-----|-----|-----|-----|-----|---|
| \ cde| 000 | 001 | 010 | 011 | 100 | 101 | 110 | 111 | ri|
|------|-----|-----|-----|-----|-----|-----|-----|-----|---|
|ab \  | 000 | 001 | 011 | 010 | 110 | 111 | 101 | 100 | k1|
|------|-----|-----|-----|-----|-----|-----|-----|-----|---|
|00|00 |     |     |     |     |     |     |     |     |
|01|01 |     |     |     |     |     |     |     |     |
|10|11 |     |     |     |     |     |     |     |     |
|11|10 |     |     |     |     |     |     |  1  |     |
|--|---|-----|-----|-----|-----|-----|-----|-----|-----|
|ri| ki|
|--|---|

in this case [true,true,true,false,true] will be 11101, and the algorithm split the code into rows
and colums based in the lenght of the array, the column always be the biggest another half

|   |ki   | ri |
|---|-----|----| 
|ab | 11  | 10 |
|cde| 101 |110 |

and the code should return the "ri" 

and should return both "ri"
*/

#[test]
fn convert(){
    let (rc,cc) = boolean_to_cords::bool_to_cords(&[true,true,true,false,true]);
    assert!((rc,cc)==(0b10,0b110),",b");
}




