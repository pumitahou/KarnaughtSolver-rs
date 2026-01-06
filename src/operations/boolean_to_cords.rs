use crate::{BinaryCord, operations::converters::gray_to_binary};



///
/// the functions converts boolean arrays to two numbers
/// [true,false,true, false,true,true,false] -> (0b101,0b0110)
pub fn bool_to_cords(input: &[bool]) -> (BinaryCord, BinaryCord) {
    let n = input.len();
    let mid = n / 2; // ab (2 bits) | cde (3 bits) para n=5

    let mut row_gray = 0;
    let mut col_gray = 0;

    // Construir el valor numérico de la fila (bits de la izquierda)
    for (i, &b) in input[..mid].iter().enumerate() {
        if b {
            row_gray |= 1 << (mid - 1 - i);
        }
    }

    // Construir el valor numérico de la columna (bits de la derecha)
    for (i, &b) in input[mid..].iter().enumerate() {
        if b {
            col_gray |= 1 << (n - mid - 1 - i);
        }
    }

    // Convertir de la posición en el mapa (Gray) al índice real (Binario)
    (gray_to_binary(row_gray), gray_to_binary(col_gray))
}