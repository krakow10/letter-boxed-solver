#[derive(Debug,Clone)]
struct LetterMap{
	next_letter:[Option<Box<LetterMap>>;27],
}
impl LetterMap{
	fn new()->Self{
		LetterMap{
			next_letter:[None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None],
		}
	}
}

struct Puzzle{
	sides:[[u8;3];4],
}

fn main() {
	const WORDS:&str=include_str!("words.txt");
	let mut word_map=LetterMap::new();

	let mut count=0;
	let mut word_count=0;
	'outer: for word in WORDS.lines(){
		let mut letter_map=&mut word_map.next_letter;
		for &letter in word.as_bytes(){
			if letter<b'a'||b'z'<letter{
				continue 'outer;
			}
			let index=letter-b'a';
			if letter_map[index as usize].is_none(){
				letter_map[index as usize].replace(Box::new(LetterMap::new()));
				count+=1;
			}
			letter_map=match &mut letter_map[index as usize]{
				Some(thing)=>&mut thing.next_letter,
				None=>{
					panic!("Next letter map just inserted does not exist");
				}
			};
		}
		word_count+=1;
	}

	let todays_puzzle=Puzzle{
		sides:[
			[b'g',b'j',b'h'],
			[b'n',b'v',b'y'],
			[b'd',b'i',b'e'],
			[b'p',b'r',b'o'],
		],
	};
}
