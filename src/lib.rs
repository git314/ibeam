use std::fmt::{Display, Formatter, Error};
use owo_colors::OwoColorize;
use owo_colors::colors::*;

pub struct Int64Type(pub Vec<i64>);
impl Display for Int64Type{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut comma_separated = String::new();
        for num in &self.0[0..self.0.len() - 1] {
            comma_separated.push_str(&num.to_string());
            comma_separated.push_str("\n");
        }
        comma_separated.push_str(&self.0[self.0.len() - 1].to_string());
        println!("{}", "<pillar>".dimmed());
        println!("{}", "<int>".dimmed());
        write!(f, "{}", comma_separated.fg::<Black>().bg::<Yellow>().blink())
    }
}

pub struct StringType(pub Vec<String>);
impl Display for StringType{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut comma_separated = String::new();
        for num in &self.0[0..self.0.len() - 1] {
            comma_separated.push_str(&num.to_string());
            comma_separated.push_str("\n");
        }
        comma_separated.push_str(&self.0[self.0.len() - 1].to_string());
        println!("{}", "<pillar>".dimmed());
        println!("{}", "<char>".dimmed());
        write!(f, "{}", comma_separated.fg::<BrightYellow>())
    }
}
pub struct BoolType(pub Vec<bool>);
impl Display for BoolType{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut comma_separated = String::new();
        for num in &self.0[0..self.0.len() - 1] {
            comma_separated.push_str(&num.to_string());
            comma_separated.push_str("\n");
        }
        comma_separated.push_str(&self.0[self.0.len() - 1].to_string());
        write!(f, "<pillar>\n<lgl>\n{}\n", comma_separated)
    }
}
pub struct Float64Type(pub Vec<f64>);
impl Display for Float64Type{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut comma_separated = String::new();
        for num in &self.0[0..self.0.len() - 1] {
            comma_separated.push_str(&num.to_string());
            comma_separated.push_str("\n");
        }
        comma_separated.push_str(&self.0[self.0.len() - 1].to_string());
        write!(f, "<pillar>\n<int>\n{}\n", comma_separated)
    }
}

