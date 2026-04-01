use karnaught_solver::TruthTable;

fn main() {
    let mut b = TruthTable::<13, 2>::new();

    karnaught_solver::truth_table!(b, ["01","11","x1"]);

    let mapas = b.as_maps();
    // for i in mapas.iter().into() {
    //     i.printtable();
    // println!("nextmap ")
    // }

    mapas.iter().for_each(|m| {
        println!("Expr: {}", m.getEcuation());
    });
}
