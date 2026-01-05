use karnaught_solver::{OutputCases, TruthTable};
fn main() {

    let mut b = TruthTable::<3,1>::new();
    b.insert([true,false,false],[OutputCases::Care(true)]);
    b.insert([true,true,false],[OutputCases::DontCare]);
    b.insert([true,false,true],[OutputCases::Care(true)]);


    let mapa = karnaught_solver::KarnaugthMap::new_map(8);
    println!("{:?}",b);
    mapa.printtable();
}
