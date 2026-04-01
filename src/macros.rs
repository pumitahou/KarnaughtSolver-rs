use crate::OutputCases;


/// Convierte un índice (0, 1, 2...) a un array de booleanos de tamaño fijo I
pub fn index_to_bools<const I: usize>(index: usize) -> [bool; I] {
    let mut res = [false; I];
    for i in 0..I {
        // Extraemos los bits de derecha a izquierda
        res[I - 1 - i] = (index >> i) & 1 == 1;
    }
    res
}

/// Parsea un string tipo "1x01" a un array de OutputCases de tamaño O
pub fn parse_output_str<const O: usize>(s: &str) -> [OutputCases; O] {
    let mut res = std::array::from_fn(|_| OutputCases::DontCare);
    let chars: Vec<char> = s.chars().collect();
    
    if chars.len() != O {
        panic!("Error: El string '{}' tiene tamaño {}, pero se esperaban {} salidas.", s, chars.len(), O);
    }

    for (i, c) in chars.into_iter().enumerate() {
        res[i] = match c {
            '1' => OutputCases::Care(true),
            '0' => OutputCases::Care(false),
            _   => OutputCases::DontCare, // 'x' o cualquier otro caracter
        };
    }
    res
}

#[macro_export]
macro_rules! truth_table {
    // Caso 1: Llenar una tabla ya existente 'b'
    ($table:ident, [ $($bits:expr),* $(,)? ]) => {
        let mut _current_idx = 0;
        $(
            let out_array = $crate::parse_output_str($bits);
            let in_array = $crate::index_to_bools(_current_idx);
            $table.insert(in_array, out_array);
            _current_idx += 1;
        )*
    };

    // Caso 2: Crear una tabla nueva de forma anónima
    // Uso: let b = truth_table!(3, 4, ["1101", "0x11"]);
    ($ins:expr, $outs:expr, [ $($bits:expr),* $(,)? ]) => {
        {
            let mut t = TruthTable::<$ins, $outs>::new();
            let mut _current_idx = 0;
            $(
                let out_array = $crate::parse_output_str($bits);
                let in_array = $crate::index_to_bools(_current_idx);
                t.insert(in_array, out_array);
                _current_idx += 1;
            )*
            // El resto de la tabla ya es DontCare por defecto en tu new_map
            t
        }
    };
}
