use std::io::*;fn main(){let g="chsybrcrbyshcerlbtynswhgcghwsnytblrecoeltnwgegwntleorolobotoyonosowohogoc";
BufReader::new(std::fs::File::open("/usr/share/dict/words").unwrap()).lines().for_each(|w|{let w=w.unwrap();
for p in w.chars().collect::<Vec<char>>().windows(2){if!g.contains(&p.iter().collect::<String>()){return}};println!("{}",w)})}
