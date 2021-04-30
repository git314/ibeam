use std::fmt::{Display, Formatter, Error};
use owo_colors::OwoColorize;
//use regex::Regex;

pub enum Num{
        Int(i32),
        Float(f64),
        Text(String),
        NA(String)
}

//pub struct Pillar(pub Vec<Vec<String>>);
//impl Display for Pillar{
//    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
//    }
//}

pub struct StringType(pub Vec<String>);
impl Display for StringType{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        println!("{}", "<pillar>".truecolor(129,161,193).bold().dimmed());
        println!("{}", "<char>".truecolor(129,161,193).bold().dimmed());
        //let mut comma_separated = String::new();
        let mut comma_separated = String::new();
        for num in &self.0[0..self.0.len()] {
            //let v = format_if_na(&num).to_string();
            comma_separated.push_str(&num.to_string());
            comma_separated.push_str("\n");
            //if is_na(&v) == true{
            //    return writeln!(f, "{}", v.truecolor(180,142,173));
            //}else{
            //    return writeln!(f, "{}", v.truecolor(143,188,187));
            //}
        }
        // this line is just to prevent fn fmt from throwing an error
        comma_separated.push_str(&self.0[self.0.len() - 1].to_string());
        //writeln!(f, "<pillar>\n<int>\n{}\n", comma_separated);
        write!(f, "{:>}", comma_separated)
    }
}

// pub struct VecStringType(pub Vec<Vec<String>>);
// impl Display for VecStringType{
//     fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
// 
//     }
// }

//Strings
// return elipse for strings that are too long

//floats
// Defaults:
//  pad the decimals
//  if negative then color
// set digits - characters after the decimal point.
// set sigfig - who knows
// set label - a column header
// scale - a multiplier
// notation
//  fit - if 13 digits or less show all digits else use scientific notation (default)
//  sci - use scientific notation
//  dec - use decimal notation reguardless of width
//  si -  https://www.nist.gov/pml/weights-and-measures/metric-si-prefixes
