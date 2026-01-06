use karnaught_solver::TruthTable;
use karnaught_solver::OutputCases::*;
fn main() {
    let mut b = TruthTable::<5,2>::new();
    b.insert([true,false,false,true,false],[Care(true),Care(false)]);
    b.insert([true,true,false,true,false],[DontCare,Care(true)]);
    b.insert([true,false,true,true,true],[Care(true),Care(false)]);

    let mapas = b.as_maps();
   for i in mapas {
    i.printtable();
    println!("nextmap ")
   }

   
}
