use std::io::*;static G:&str="chsybrcrbyshcerlbtynswhgcghwsnytblrecoeltnwgegwntleorolobotoyonosowohogoc";
fn main(){BufReader::new(std::fs::File::open("/usr/share/dict/words").unwrap()).lines().for_each(|w|q(w.unwrap()));}
fn q(w:String){for p in w.chars().collect::<Vec<char>>().windows(2){if !G.contains(&p.iter().collect::<String>()){return}}println!("{}",w);}
