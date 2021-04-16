//let val = { name };
//let expr = || { val }; // Defuses the same expression
//let late_val = expr(); // Evaluates it back
// pub struct BoolType(Vec<bool>);
// pub struct StringType(Vec<String>);
// pub struct Int64Type(Vec<i64>);
// pub struct Float64Type(Vec<f64>);

macro_rules! vec_str {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

fn main() {
    let ints: Vec<i64> = vec![1; 3];
    let strings: Vec<String> = vec_str!["a","b","c","defghijklmnop"];
    let doubles: Vec<f64> = vec![3.14; 3];
    let vi = ibeam::Int64Type(ints);
    let vs = ibeam::StringType(strings);
    let vd = ibeam::Float64Type(doubles);
    println!("{}", vi);
    println!("{}", vs);
    println!("{}", vd);
}
