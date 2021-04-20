// macro_rules! vec_str {
//     ($($x:expr),*) => (vec![$($x.to_string()),*]);
// }
//fn is_date(text: &str) -> usize{
//    let r = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
//    let x = r.is_match(text);
//    let lgl = if x == false { 0 } else { 1 };
//    return lgl
//}
//
//fn is_datetime(text: &str) -> usize{
//    //grex NA Null na null "" None Na N/A NaN NAN nan
//    let rgex = r"\s+(?=\d{2}(?:\d{2})?-\d{1,2}-\d{1,2}\b)";
//    let r = Regex::new(rgex).unwrap();
//    let x = r.is_match(text);
//    let lgl = if x == false { 0 } else { 1 };
//    return lgl
//}


//fn is_na(text: &String) -> usize{
//    //grex NA Null na null "" None Na N/A NaN NAN nan
//    let rgex = r"^$|^(?:N(?:(?:(?:one|AN|a[Nn]|/A)|[Aa])|ull)|n(?:ull|an?))$";
//    let r = Regex::new(rgex).unwrap();
//    let x = r.is_match(&text);
//    let lgl = if x == false { 0 } else { 1 };
//    return lgl
//}
//
//// if is_na then return NA
//fn format_if_na(text: &String) -> &str{
//    let s = is_na(&text);
//    // 1=missing string
//    let missing_string_value: &str = "NA";
//    let string: &str = if s == 1 { missing_string_value } else {text};
//    return string
//}

fn main() {

    //let bold_header: bool = true;
    //let subtle: bool  = true;
    //let neg: bool  = true;
    //let sigfig: usize = 3;

   //grex NA Null na null "" None Na N/A NaN NAN nan
   let strings: Vec<_> = vec!["a","NaN","None","jklkjhjklhjklkj","asdfjkl"].into_iter()
        .map(String::from)
        .collect();
   let floats: Vec<_> = vec!["1.0101","10101.","Na", "0.01","nan"].into_iter()
        .map(String::from)
        .collect();
   
   let vs = ibeam::StringType(strings);
   let vf = ibeam::StringType(floats);

   println!("{}\n\n{}", vs, vf);
}
