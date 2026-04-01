use std::{collections::HashMap, iter::zip, ops::Div};
mod expresion;
pub mod macros;
mod operations;
pub use crate::macros::index_to_bools;
pub use crate::macros::parse_output_str;

pub type GrayNumber = u32;
pub type BinaryCord = usize;
use OutputCases::*;

use crate::operations::boolean_to_cords::bool_to_cords;
/// | inputs  | output     |
/// |---------|------------|
/// | xxxxxxx | 1 care     |
/// | xxxxxxx | x DontCare |
#[derive(Debug, Clone)]
pub enum OutputCases {
    Care(bool),
    DontCare,
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
pub struct TruthTable<const I: usize, const O: usize> {
    table: HashMap<[bool; I], [OutputCases; O]>,
    pub i_size: u8,
    pub o_size: u8,
}

impl<const I: usize, const O: usize> TruthTable<I, O> {
    pub fn new() -> Self {
        TruthTable {
            table: HashMap::new(),
            i_size: I as u8,
            o_size: O as u8,
        }
    }
    pub fn insert(&mut self, i: [bool; I], o: [OutputCases; O]) {
        self.table.insert(i, o);
    }

    pub fn as_maps(&self) -> Vec<KarnaugthMap> {
        let mut allmaps = Vec::new();

        //this line add N maps depending of TruthTable Outputs
        for _ in 0..O {
            allmaps.push(KarnaugthMap::new_map(I));
        }

        //this function fills generate the karnaughtmaps with the correct values
        for (map, out_idx) in zip(allmaps.iter_mut(), 0..O) {
            for inputs in self.table.keys() {
                //rowcords and columns cords as binary for insertion
                let (row_cord, column_cord) = bool_to_cords(inputs);

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
pub struct KarnaugthMap {
    /// no se modifica
    matriz: Vec<Vec<OutputCases>>,
}

impl KarnaugthMap {
    // ///this function must be return a "s1s2'+s2s1' " or "a'b'+bc+c'a" instead s notation if the
    // ///space can be represented by 8 variables in KarnaugthMap
    // ///
    // /// in this solition, if the cases have many gropus like G -> [2,4,15,23]; any case of
    // /// C={Ai ⊆ N∣i∈I}
    // /// the filtrer defined by C′={Ai ∈C∣∃x∈Ai​ must be ∀j =i,x∈/Aj}
    // pub fn solve_as_str(&self) -> String {
    //
    //
    //     let all_lineals = self.solve_lineal_cases();
    //     let all_quads = self.solve_quad_cases();
    //     let all_rectagle_cases = self.solve_rectangular_cases();
    //
    //
    //
    // }

    // fn solve_lineal_cases(&self) -> Vec<Vec<u32>> {
    //    Vec::new()
    // }

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
    pub fn new_map(input_size: usize) -> Self {
        // Calculamos cuántos bits van para las filas y cuántos para las columnas
        let row_bits = input_size / 2;
        let col_bits = input_size - row_bits; // El resto de los bits

        // Elevamos 2 a la potencia de los bits (1 << N es igual a 2^N)
        let rows = 1 << row_bits;
        let colum = 1 << col_bits;

        let mut matriz = Vec::new();
        for _ in 0..rows {
            let mut vector = Vec::new();
            for _ in 0..colum {
                vector.push(DontCare);
            }
            matriz.push(vector);
        }
        KarnaugthMap { matriz }
    }

    // pub fn new_map(input_size: usize) -> Self {
    //     let rows = input_size.div(2) << 1;
    //     let colum = input_size.div_ceil(2) << 1;
    //     let mut matriz = Vec::new();
    //     for _ in 0..rows {
    //         let mut vector = Vec::new();
    //         for _ in 0..colum {
    //             vector.push(DontCare);
    //         }
    //         matriz.push(vector);
    //     }
    //     KarnaugthMap { matriz: matriz }
    // }

    pub fn printtable(&self) {
        for i in self.matriz.iter() {
            for j in i.iter() {
                match j {
                    DontCare => print!("x"),
                    Care(value) => match value {
                        true => print!("1"),
                        false => print!("0"),
                    },
                }
                print!("|")
            }
            println!("");
        }
    }

    /// !Caution
    /// this function isn't protected by outrange
    /// this function change the status
    fn update(&mut self, row_cord: usize, column_cord: usize, caso: OutputCases) {
        self.matriz[row_cord][column_cord] = caso;
    }

    fn group_maker(&self, group: Vec<usize>) -> BooleanGroup {
        let total_vars = self.get_total_vars();
        let mut bits_or = 0;
        let mut bits_and = !0; // Inicia con todos en 1

        for &number in &group {
            bits_or |= number;
            bits_and &= number;
        }

        let mut terms = Vec::new();
        // Iterar desde el bit más significativo (A) al menos significativo
        for i in (0..total_vars).rev() {
            let bit_mask = 1 << i;
            let var_idx = total_vars - 1 - i;

            if (bits_and & bit_mask) != 0 {
                terms.push((var_idx, true)); // Constante 1 -> Positivo
            } else if (bits_or & bit_mask) == 0 {
                terms.push((var_idx, false)); // Constante 0 -> Negado
            }
        }

        BooleanGroup { terms }
    }

    // pub fn getEcuation(&self) -> String {
    //     let all_conjuncts = self.solve_map();
    //     let valid_conjuncts = Self::get_only_valid_conjuncts(all_conjuncts);
    //
    //     let mut groups = Vec::new();
    //     for conj in valid_conjuncts {
    //         groups.push(self.group_maker(conj));
    //     }
    //
    //     let eq = BooleanEquation {
    //         groups,
    //         total_vars: self.get_total_vars(),
    //     };
    //
    //     eq.to_string()
    // }

    pub fn getEcuation(&self) -> String {
        // solve_map ahora devuelve directamente los implicantes finales reducidos
        let valid_conjuncts = self.solve_map();

        let mut groups = Vec::new();
        for conj in valid_conjuncts {
            groups.push(self.group_maker(conj));
        }

        let eq = BooleanEquation {
            groups,
            total_vars: self.get_total_vars(),
        };

        eq.to_string()
    }

    // --- MÉTODOS INTERNOS REESCRITOS PARA OPTIMIZACIÓN ABSOLUTA ---

    fn solve_map(&self) -> Vec<Vec<usize>> {
        let mut all_conjuncts = Vec::new();
        let rows = self.matriz.len();
        if rows == 0 {
            return all_conjuncts;
        }
        let cols = self.matriz[0].len();

        let mut targets = std::collections::HashSet::new();

        for r in 0..rows {
            for c in 0..cols {
                // Registrar todos los '1' reales que DEBEN ser cubiertos
                if self.is_care_true(r, c) {
                    targets.insert(self.get_cell_binary(r, c, cols));
                    // Añadimos la celda individual por si queda aislada
                    all_conjuncts.push(vec![self.get_cell_binary(r, c, cols)]);
                }

                // Obtener TODOS los grupos geométricos posibles sin discriminar
                all_conjuncts.extend(self.get_group_cubic(r, c, rows, cols));
                all_conjuncts.extend(self.get_group_lineal(r, c, rows, cols));
                all_conjuncts.extend(self.get_group_rectangular(r, c, rows, cols));
            }
        }

        // El filtro ahora recibe los targets para buscar la mínima cobertura (Set Cover)
        // println!("Targets: {:?}", targets);
        // println!("Total conjuncts antes de filtrar: {}", all_conjuncts.len());
        let result = Self::get_only_valid_conjuncts(all_conjuncts, targets);
        // println!("Conjuncts finales: {:?}", result);
        result
    }

    fn get_group_cubic(&self, row: usize, col: usize, rows: usize, cols: usize) -> Vec<Vec<usize>> {
        let mut results_general = Vec::new();
        let dirs: [(isize, isize); 4] = [(1, 1), (-1, 1), (-1, -1), (1, -1)];
        let max_iter = rows.max(cols).next_power_of_two().trailing_zeros() as usize;

        for iteration in 1..=max_iter {
            let size = 1 << iteration;
            if size > rows || size > cols {
                continue;
            }

            for &(dr, dc) in &dirs {
                let mut valid = true;
                let mut has_true = false;
                for i in 0..size {
                    for j in 0..size {
                        let r = ((row as isize + dr * i as isize) % rows as isize + rows as isize)
                            % rows as isize;
                        let c = ((col as isize + dc * j as isize) % cols as isize + cols as isize)
                            % cols as isize;
                        if !self.is_valid_care(r as usize, c as usize) {
                            valid = false;
                            break;
                        }
                        if self.is_care_true(r as usize, c as usize) {
                            has_true = true;
                        }
                    }
                    if !valid {
                        break;
                    }
                }

                if valid && has_true {
                    let mut current_result = Vec::new();
                    for i in 0..size {
                        for j in 0..size {
                            let r = ((row as isize + dr * i as isize) % rows as isize
                                + rows as isize)
                                % rows as isize;
                            let c = ((col as isize + dc * j as isize) % cols as isize
                                + cols as isize)
                                % cols as isize;
                            current_result.push(self.get_cell_binary(r as usize, c as usize, cols));
                        }
                    }
                    current_result.sort();
                    current_result.dedup();
                    results_general.push(current_result);
                }
            }
        }
        results_general
    }

    fn get_group_lineal(
        &self,
        row: usize,
        col: usize,
        rows: usize,
        cols: usize,
    ) -> Vec<Vec<usize>> {
        let mut results_general = Vec::new();
        let max_iter = rows.max(cols).next_power_of_two().trailing_zeros() as usize;

        for iteration in 1..=max_iter {
            let size = 1 << iteration;

            // Horizontal (+)
            if size <= cols {
                let mut valid = true;
                let mut has_true = false;
                for i in 0..size {
                    let c = (col + i) % cols;
                    if !self.is_valid_care(row, c) {
                        valid = false;
                        break;
                    }
                    if self.is_care_true(row, c) {
                        has_true = true;
                    }
                }
                if valid && has_true {
                    let mut res = Vec::new();
                    for i in 0..size {
                        res.push(self.get_cell_binary(row, (col + i) % cols, cols));
                    }
                    res.sort();
                    res.dedup();
                    results_general.push(res);
                }
            }
            // Horizontal (-)
            if size <= cols {
                let mut valid = true;
                let mut has_true = false;
                for i in 0..size {
                    let c = ((col as isize - i as isize) % cols as isize + cols as isize)
                        % cols as isize;
                    if !self.is_valid_care(row, c as usize) {
                        valid = false;
                        break;
                    }
                    if self.is_care_true(row, c as usize) {
                        has_true = true;
                    }
                }
                if valid && has_true {
                    let mut res = Vec::new();
                    for i in 0..size {
                        let c = ((col as isize - i as isize) % cols as isize + cols as isize)
                            % cols as isize;
                        res.push(self.get_cell_binary(row, c as usize, cols));
                    }
                    res.sort();
                    res.dedup();
                    results_general.push(res);
                }
            }
            // Vertical (+)
            if size <= rows {
                let mut valid = true;
                let mut has_true = false;
                for i in 0..size {
                    let r = (row + i) % rows;
                    if !self.is_valid_care(r, col) {
                        valid = false;
                        break;
                    }
                    if self.is_care_true(r, col) {
                        has_true = true;
                    }
                }
                if valid && has_true {
                    let mut res = Vec::new();
                    for i in 0..size {
                        res.push(self.get_cell_binary((row + i) % rows, col, cols));
                    }
                    res.sort();
                    res.dedup();
                    results_general.push(res);
                }
            }
            // Vertical (-)
            if size <= rows {
                let mut valid = true;
                let mut has_true = false;
                for i in 0..size {
                    let r = ((row as isize - i as isize) % rows as isize + rows as isize)
                        % rows as isize;
                    if !self.is_valid_care(r as usize, col) {
                        valid = false;
                        break;
                    }
                    if self.is_care_true(r as usize, col) {
                        has_true = true;
                    }
                }
                if valid && has_true {
                    let mut res = Vec::new();
                    for i in 0..size {
                        let r = ((row as isize - i as isize) % rows as isize + rows as isize)
                            % rows as isize;
                        res.push(self.get_cell_binary(r as usize, col, cols));
                    }
                    res.sort();
                    res.dedup();
                    results_general.push(res);
                }
            }
        }
        results_general
    }

    fn get_group_rectangular(
        &self,
        row: usize,
        col: usize,
        rows: usize,
        cols: usize,
    ) -> Vec<Vec<usize>> {
        let mut results_general = Vec::new();
        let dirs: [(isize, isize); 4] = [(1, 1), (-1, 1), (-1, -1), (1, -1)];
        let max_iter = rows.max(cols).next_power_of_two().trailing_zeros() as usize;

        for iteration in 1..=max_iter {
            let size = 1 << iteration;

            // Horizontales (2x * x)
            if (size * 2) <= cols && size <= rows {
                for &(dr, dc) in &dirs {
                    let mut valid = true;
                    let mut has_true = false;
                    for i in 0..(size * 2) {
                        for j in 0..size {
                            let r = ((row as isize + dr * j as isize) % rows as isize
                                + rows as isize)
                                % rows as isize;
                            let c = ((col as isize + dc * i as isize) % cols as isize
                                + cols as isize)
                                % cols as isize;
                            if !self.is_valid_care(r as usize, c as usize) {
                                valid = false;
                                break;
                            }
                            if self.is_care_true(r as usize, c as usize) {
                                has_true = true;
                            }
                        }
                        if !valid {
                            break;
                        }
                    }
                    if valid && has_true {
                        let mut res = Vec::new();
                        for i in 0..(size * 2) {
                            for j in 0..size {
                                let r = ((row as isize + dr * j as isize) % rows as isize
                                    + rows as isize)
                                    % rows as isize;
                                let c = ((col as isize + dc * i as isize) % cols as isize
                                    + cols as isize)
                                    % cols as isize;
                                res.push(self.get_cell_binary(r as usize, c as usize, cols));
                            }
                        }
                        res.sort();
                        res.dedup();
                        results_general.push(res);
                    }
                }
            }

            // Verticales (x * 2x)
            if size <= cols && (size * 2) <= rows {
                for &(dr, dc) in &dirs {
                    let mut valid = true;
                    let mut has_true = false;
                    for i in 0..size {
                        for j in 0..(size * 2) {
                            let r = ((row as isize + dr * j as isize) % rows as isize
                                + rows as isize)
                                % rows as isize;
                            let c = ((col as isize + dc * i as isize) % cols as isize
                                + cols as isize)
                                % cols as isize;
                            if !self.is_valid_care(r as usize, c as usize) {
                                valid = false;
                                break;
                            }
                            if self.is_care_true(r as usize, c as usize) {
                                has_true = true;
                            }
                        }
                        if !valid {
                            break;
                        }
                    }
                    if valid && has_true {
                        let mut res = Vec::new();
                        for i in 0..size {
                            for j in 0..(size * 2) {
                                let r = ((row as isize + dr * j as isize) % rows as isize
                                    + rows as isize)
                                    % rows as isize;
                                let c = ((col as isize + dc * i as isize) % cols as isize
                                    + cols as isize)
                                    % cols as isize;
                                res.push(self.get_cell_binary(r as usize, c as usize, cols));
                            }
                        }
                        res.sort();
                        res.dedup();
                        results_general.push(res);
                    }
                }
            }
        }
        results_general
    }

    /// Filtra subconjuntos y realiza una Cobertura Mínima (Quine-McCluskey Set Cover)
    fn get_only_valid_conjuncts(
        mut all_conjuncts: Vec<Vec<usize>>,
        mut targets: std::collections::HashSet<usize>,
    ) -> Vec<Vec<usize>> {
        for conj in &mut all_conjuncts {
            conj.sort();
            conj.dedup();
        }
        all_conjuncts.sort();
        all_conjuncts.dedup();

        // PARCHE CLAUDE 31 03 26
        // para que se filtren bien 
        let total_vars = (targets.iter().max().copied().unwrap_or(0) + 1)
            .next_power_of_two()
            .trailing_zeros() as usize;
        all_conjuncts.retain(|g| Self::is_valid_karnaugh_group(g, total_vars));
        // ---

        all_conjuncts.sort_by(|a, b| b.len().cmp(&a.len()));

        let mut prime_implicants: Vec<Vec<usize>> = Vec::new();
        for conj in all_conjuncts {
            if conj.is_empty() {
                continue;
            }
            let is_contained = prime_implicants
                .iter()
                .any(|existing| conj.iter().all(|x| existing.contains(x)));
            if !is_contained {
                // ← FIX: elimina los que ahora quedan redundantes
                prime_implicants.retain(|existing| !existing.iter().all(|x| conj.contains(x)));
                prime_implicants.push(conj);
            }
        }
        let mut final_groups = Vec::new();

        // 2. Extraer los Implicantes Primos Esenciales
        loop {
            let mut essential_found = false;
            for &target in &targets {
                let covering_pis: Vec<_> = prime_implicants
                    .iter()
                    .filter(|pi| pi.contains(&target))
                    .collect();
                if covering_pis.len() == 1 {
                    let essential_pi = covering_pis[0].clone();
                    final_groups.push(essential_pi.clone());
                    for t in &essential_pi {
                        targets.remove(t);
                    }
                    essential_found = true;
                    break;
                }
            }
            if !essential_found || targets.is_empty() {
                break;
            }
        }

        // 3. Selección codiciosa para las celdas restantes (Greedy Set Cover)
        while !targets.is_empty() {
            if let Some(best_pi) = prime_implicants
                .iter()
                .max_by_key(|pi| pi.iter().filter(|x| targets.contains(x)).count())
            {
                let count = best_pi.iter().filter(|x| targets.contains(x)).count();
                if count == 0 {
                    break;
                } // Seguridad
                final_groups.push(best_pi.clone());
                for t in best_pi {
                    targets.remove(t);
                }
            } else {
                break;
            }
        }

        final_groups
    }
    // --- FUNCIONES DE AYUDA MODIFICADAS (HELPER) ---

    fn is_valid_care(&self, r: usize, c: usize) -> bool {
        matches!(self.matriz[r][c], Care(true) | DontCare)
    }

    // NUEVO: Para saber si la celda tiene de verdad un 1 y no solo un "No me importa"
    fn is_care_true(&self, r: usize, c: usize) -> bool {
        matches!(self.matriz[r][c], Care(true))
    }

    fn get_total_vars(&self) -> usize {
        let rows = self.matriz.len().next_power_of_two();
        let cols = if rows > 0 {
            self.matriz[0].len().next_power_of_two()
        } else {
            0
        };
        (rows.trailing_zeros() + cols.trailing_zeros()) as usize
    }

    fn get_cell_binary(&self, r: usize, c: usize, cols: usize) -> usize {
        let gray_r = r ^ (r >> 1);
        let gray_c = c ^ (c >> 1);
        let col_bits = cols.next_power_of_two().trailing_zeros(); // Soportar tamaños de col raros
        (gray_r << col_bits) | gray_c
    }

    //This function must be solve the karnaught
    fn is_valid_karnaugh_group(group: &Vec<usize>, total_vars: usize) -> bool {
        let mut bits_or = 0usize;
        let mut bits_and = !0usize;
        for &n in group {
            bits_or |= n;
            bits_and &= n;
        }
        let free_bits = (0..total_vars)
            .filter(|i| {
                let mask = 1 << i;
                (bits_or & mask) != 0 && (bits_and & mask) == 0
            })
            .count();
        free_bits == (group.len().trailing_zeros() as usize)
    }
}

/// Representa un conjunto de términos (ej. A'B)
pub struct BooleanGroup {
    /// Vector de tuplas: (índice_de_variable, es_positiva)
    /// ej: (0, true) -> A, (1, false) -> B'
    pub terms: Vec<(usize, bool)>,
}

/// Representa la ecuación final extraída del mapa de Karnaugh
pub struct BooleanEquation {
    pub groups: Vec<BooleanGroup>,
    pub total_vars: usize,
}

impl BooleanEquation {
    pub fn to_string(&self) -> String {
        if self.groups.is_empty() {
            return "0".to_string();
        }

        let mut out = Vec::new();
        let use_generic = self.total_vars > 26;

        for group in &self.groups {
            let mut group_str = String::new();
            if group.terms.is_empty() {
                group_str.push('1'); // Grupo que abarca todo el mapa
            } else {
                for &(var_idx, is_pos) in &group.terms {
                    if use_generic {
                        group_str.push_str(&format!("X{}", var_idx + 1));
                        if !is_pos {
                            group_str.push('\'');
                        }
                    } else {
                        // 'A' es 65 en ASCII. var_idx = 0 -> 'A', var_idx = 1 -> 'B'
                        let c = (b'A' + var_idx as u8) as char;
                        group_str.push(c);
                        if !is_pos {
                            group_str.push('\'');
                        }
                    }
                }
            }
            out.push(group_str);
        }
        out.join(" + ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 1. TEST DE DIMENSIONES DEL MAPA
    // Verifica que el cálculo de `next_power_of_two` y la división de bits asigne
    // las filas y columnas correctamente para 2, 3, 4 y 5 variables.
    #[test]
    fn test_map_dimensions() {
        let map_2_vars = KarnaugthMap::new_map(2);
        assert_eq!(map_2_vars.matriz.len(), 2); // 2^1 = 2 filas
        assert_eq!(map_2_vars.matriz[0].len(), 2); // 2^1 = 2 columnas

        let map_3_vars = KarnaugthMap::new_map(3);
        assert_eq!(map_3_vars.matriz.len(), 2); // 2^1 = 2 filas
        assert_eq!(map_3_vars.matriz[0].len(), 4); // 2^2 = 4 columnas

        let map_4_vars = KarnaugthMap::new_map(4);
        assert_eq!(map_4_vars.matriz.len(), 4); // 2^2 = 4 filas
        assert_eq!(map_4_vars.matriz[0].len(), 4); // 2^2 = 4 columnas

        let map_5_vars = KarnaugthMap::new_map(5);
        assert_eq!(map_5_vars.matriz.len(), 4); // 2^2 = 4 filas
        assert_eq!(map_5_vars.matriz[0].len(), 8); // 2^3 = 8 columnas
    }

    // 2. TEST DE CONVERSIÓN DE COORDENADAS A BINARIO
    // Verifica que las coordenadas (fila, columna) del mapa de Karnaugh
    // se traduzcan de regreso al índice binario real utilizando código Gray.
    #[test]
    fn test_get_cell_binary() {
        let map = KarnaugthMap::new_map(4); // Mapa 4x4
        let cols = 4;

        // Fila 0, Col 0 -> Gray 00, 00 -> Binario 0
        assert_eq!(map.get_cell_binary(0, 0, cols), 0);

        // Fila 0, Col 2 -> Gray 00, 11 -> Binario 3
        // Recuerda que la columna 2 en el mapa es Gray '11'
        assert_eq!(map.get_cell_binary(0, 2, cols), 3);

        // Fila 2, Col 2 -> Gray 11, 11 -> Binario 15
        assert_eq!(map.get_cell_binary(2, 2, cols), 15);

        // Fila 3, Col 0 -> Gray 10, 00 -> Binario 8
        assert_eq!(map.get_cell_binary(3, 0, cols), 8);
    }

    // 3. TEST DE FILTRADO DE CONJUNTOS REDUNDANTES
    // Verifica que si hay un grupo [0, 1] y un grupo mayor [0, 1, 2, 3],
    // el algoritmo descarte el pequeño por estar contenido en el grande.
    #[test]
    fn test_get_only_valid_conjuncts() {
        let all_groups = vec![
            vec![0, 1],       // Subgrupo (debe eliminarse)
            vec![0, 1, 2, 3], // Grupo mayor (debe mantenerse)
            vec![5],          // Grupo aislado (debe mantenerse)
            vec![0, 2],       // Subgrupo
        ];

        // NUEVO: Creamos los targets para pasarlos como segundo argumento
        let mut targets = std::collections::HashSet::new();
        targets.insert(0);
        targets.insert(1);
        targets.insert(2);
        targets.insert(3);
        targets.insert(5);

        // Ahora sí pasamos los 2 argumentos correctamente
        let valid = KarnaugthMap::get_only_valid_conjuncts(all_groups, targets);

        assert_eq!(valid.len(), 2);
        assert!(valid.contains(&vec![0, 1, 2, 3]));
        assert!(valid.contains(&vec![5]));
    }
    // 4. TEST DE GENERACIÓN DE EXPRESIONES (GROUP MAKER)
    // Verifica que los conjuntos de números binarios se traduzcan a los términos booleanos correctos.
    #[test]
    fn test_group_maker() {
        let map = KarnaugthMap::new_map(3); // A, B, C (3 bits)
        // Grupo que abarca las celdas 0 (000) y 1 (001)
        // A=0, B=0, C varía. Por ende, la ecuación debería ser A'B'
        let group = vec![0, 1];
        let boolean_group = map.group_maker(group);

        assert_eq!(boolean_group.terms.len(), 2);
        assert_eq!(boolean_group.terms[0], (0, false)); // A' (Índice 0, Negado)
        assert_eq!(boolean_group.terms[1], (1, false)); // B' (Índice 1, Negado)
        // La C (Índice 2) no debe estar porque varió.
    }

    // 5. TEST DE ECUACIÓN FINAL COMPLETA (CASOS CONOCIDOS)
    // Probamos inyectando 1s directamente en posiciones estratégicas y sacando la ecuación.
    #[test]
    fn test_known_ecuations() {
        // --- CASO 1: AND COMPLETO (Todas las celdas en 1) ---
        let mut map_all = KarnaugthMap::new_map(2);
        map_all.update(0, 0, Care(true));
        map_all.update(0, 1, Care(true));
        map_all.update(1, 0, Care(true));
        map_all.update(1, 1, Care(true));
        // Si todo el mapa es 1, la ecuación es 1.
        assert_eq!(map_all.getEcuation(), "1");

        // --- CASO 2: MAPA VACÍO (Todos ceros) ---
        let mut map_zero = KarnaugthMap::new_map(2);
        map_zero.update(0, 0, Care(false));
        map_zero.update(0, 1, Care(false));
        map_zero.update(1, 0, Care(false));
        map_zero.update(1, 1, Care(false));
        // Si todo el mapa es 0 (o DontCare resuelto a 0), la ecuación es 0.
        assert_eq!(map_zero.getEcuation(), "0");

        // --- CASO 3: UN SOLO TÉRMINO ---
        // Mapa 2 vars (A, B). Ponemos 1 en (A=1, B=1) -> Celda Gray Fila 1 (A=1), Col 1 (B=1)
        let mut map_single = KarnaugthMap::new_map(2);
        // Inicializar a 0 por seguridad
        for r in 0..2 {
            for c in 0..2 {
                map_single.update(r, c, Care(false));
            }
        }
        map_single.update(1, 1, Care(true)); // (Fila 1 = 1, Col 1 = 1) -> Binario 3 (11) -> AB
        assert_eq!(map_single.getEcuation(), "AB");
    }

    // 6. TEST DE EXTRACCIÓN CÚBICA (CUBIC SOLVER)
    // Verifica que el solver logre detectar un cubo válido y no mezcle variables a lo loco
    #[test]
    fn test_cubic_solver() {
        let mut map = KarnaugthMap::new_map(4); // 4x4 = 16 celdas
        // Vamos a hacer un "cubo" en el centro de un mapa 4x4
        for r in 0..4 {
            for c in 0..4 {
                map.update(r, c, Care(false));
            }
        }

        map.update(1, 1, Care(true));
        map.update(1, 2, Care(true));
        map.update(2, 1, Care(true));
        map.update(2, 2, Care(true));

        let cubic_group = map.get_group_cubic(1, 1, 4, 4);

        // CORRECCIÓN: Ahora cubic_group es un Vec<Vec<usize>>, medimos el tamaño del primer grupo
        assert_eq!(
            cubic_group[0].len(),
            4,
            "Debería encontrar exactamente 4 elementos en el grupo"
        );
    }

    #[test]
    fn test_image_truth_table() {
        // 5 entradas (A,B,C,D,E) y 1 salida (Y)
        let mut b = TruthTable::<5, 1>::new();

        // Filas 0 a 4: Y = 1
        b.insert([false, false, false, false, false], [Care(true)]); // 0
        b.insert([false, false, false, false, true], [Care(true)]); // 1
        b.insert([false, false, false, true, false], [Care(true)]); // 2
        b.insert([false, false, false, true, true], [Care(true)]); // 3
        b.insert([false, false, true, false, false], [Care(true)]); // 4

        // Fila 5: Y = 0
        b.insert([false, false, true, false, true], [Care(false)]); // 5

        // Filas 6 a 15: Y = 1
        b.insert([false, false, true, true, false], [Care(true)]); // 6
        b.insert([false, false, true, true, true], [Care(true)]); // 7
        b.insert([false, true, false, false, false], [Care(true)]); // 8
        b.insert([false, true, false, false, true], [Care(true)]); // 9
        b.insert([false, true, false, true, false], [Care(true)]); // 10
        b.insert([false, true, false, true, true], [Care(true)]); // 11
        b.insert([false, true, true, false, false], [Care(true)]); // 12
        b.insert([false, true, true, false, true], [Care(true)]); // 13
        b.insert([false, true, true, true, false], [Care(true)]); // 14
        b.insert([false, true, true, true, true], [Care(true)]); // 15

        // Filas 16 a 31: Y = x (DontCare)
        b.insert([true, false, false, false, false], [DontCare]); // 16
        b.insert([true, false, false, false, true], [DontCare]); // 17
        b.insert([true, false, false, true, false], [DontCare]); // 18
        b.insert([true, false, false, true, true], [DontCare]); // 19
        b.insert([true, false, true, false, false], [DontCare]); // 20
        b.insert([true, false, true, false, true], [DontCare]); // 21
        b.insert([true, false, true, true, false], [DontCare]); // 22
        b.insert([true, false, true, true, true], [DontCare]); // 23
        b.insert([true, true, false, false, false], [DontCare]); // 24
        b.insert([true, true, false, false, true], [DontCare]); // 25
        b.insert([true, true, false, true, false], [DontCare]); // 26
        b.insert([true, true, false, true, true], [DontCare]); // 27
        b.insert([true, true, true, false, false], [DontCare]); // 28
        b.insert([true, true, true, false, true], [DontCare]); // 29
        b.insert([true, true, true, true, false], [DontCare]); // 30
        b.insert([true, true, true, true, true], [DontCare]); // 31

        // Para ver los mapas generados
        let mapas = b.as_maps();
        for i in mapas {
            print!("Expr: {}", i.getEcuation());
        }
    }

    #[test]
    fn casos_formulas() {
        let t = truth_table!(3, 1, ["1", "1", "0", "1", "1", "0", "1", "x",]);

        let r = t.as_maps()[0].getEcuation();

        // CORRECCIÓN: El algoritmo elige determinísticamente esta variante
        // que es matemáticamente idéntica y óptima a A'B' + A'C + AC'
        assert_eq!(r, "AC' + A'C + B'C'".to_string());
    }
}
