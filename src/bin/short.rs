use std::io::*;static G:&str="rownsholrelegontoecoshgcrbyntybltboyswgwhc";fn main(){BufReader::new(
std::fs::File::open("/usr/share/dict/words").unwrap()).lines().for_each(|w|q(w.unwrap()));}
fn q(w:String){for p in w.chars().collect::<Vec<char>>().windows(2){if G.contains(&c(p)){continue};
let mut p=p.to_owned();p.reverse();if !G.contains(&c(p.as_slice())){return};}println!("{}",w);}
fn c(s:&[char])->String{s.iter().collect()}
