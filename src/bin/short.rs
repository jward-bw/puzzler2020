use std::io::*;fn main(){let g="chsybrcrbyshcerlbtynswhgcghwsnytblrecoeltnwgegwntleorolobotoyonosowohogoc";
BufReader::new(std::fs::File::open("/usr/share/dict/words").unwrap()).lines().for_each(|w|{let w=w.unwrap();
for p in w.as_bytes().windows(2){if!g.contains(&std::str::from_utf8(p).unwrap().to_lowercase()){return}};println!("{}",w)})}
