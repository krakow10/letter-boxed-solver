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
const TERMINATOR:usize=26;

fn main() {
	const WORDS:&str=include_str!("words.txt");

	let start_time=std::time::Instant::now();

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
		// mark the end of a word.
		letter_map[26].replace(Box::new(LetterMap::new()));
		word_count+=1;
	}

	// println!("{count} in {:?}",start_time.elapsed());
	// println!("total words = {word_count}");

	struct Puzzle{
		sides:[[char;3];4],
	}

	let todays_puzzle=Puzzle{
		sides:[
			['g','j','h'],
			['n','v','y'],
			['d','i','e'],
			['p','r','o'],
		],
	};

	struct PuzzleIter<'a>{
		puzzle:&'a Puzzle,
		index:usize,
	};
	impl<'a> IntoIterator for &'a Puzzle{
		type IntoIter=PuzzleIter<'a>;
		type Item=(usize,char);
		fn into_iter(self)->Self::IntoIter{
			PuzzleIter{
				puzzle:self,
				index:0,
			}
		}
	}
	impl Iterator for PuzzleIter<'_>{
		type Item=(usize,char);
		fn next(&mut self)->Option<Self::Item>{
			if self.index<12{
				let side=self.index as usize/3;
				let value=self.puzzle.sides[side][(self.index as usize).rem_euclid(3)];
				self.index+=1;
				Some((side,value))
			}else{
				None
			}
		}
	}

	// list all valid words
	// perform a depth first search of the words tree, filtering sides.
	fn add_next_letter(
		all_words:&mut Vec<String>,
		current_word:&mut String,
		puzzle:&Puzzle,
		letter_map:&LetterMap,
		current_side:usize,
	){
		// check if current word is a real word
		if letter_map.next_letter[TERMINATOR].is_some(){
			// write that down!!!
			all_words.push(current_word.clone());
		}
		for (side,letter) in puzzle{
			// skip letters on the same side during the search
			if side==current_side{
				continue;
			}
			let letter_id=(letter as u8-b'a') as usize;
			if let Some(next_letter_map)=&letter_map.next_letter[letter_id]{
				// adding this letter to the current word can form one or more words.
				current_word.push(letter);
				add_next_letter(all_words, current_word, puzzle, next_letter_map, side);
				current_word.pop();
			}
		}
	}

	struct Words{
		starting_letter:[Vec<String>;26],
	}

	// initialize an empty list of words
	let mut valid_words=Words{
		starting_letter:core::array::from_fn(|_|Vec::new()),
	};
	for (side,letter) in &todays_puzzle{
		let letter_id=(letter as u8-b'a') as usize;
		if let Some(letter_map)=&word_map.next_letter[letter_id]{
			let letter=(letter_id as u8+b'a') as char;
			let mut all_words=Vec::new();
			let mut current_word=String::new();
			current_word.push(letter);
			add_next_letter(&mut all_words, &mut current_word, &todays_puzzle, letter_map, side);
			current_word.pop();
			valid_words.starting_letter[letter_id]=all_words;
		}
	}

	// print out all the words
	// for word_list in &valid_words.starting_letter{
	// 	for word in word_list{
	// 		println!("{word}");
	// 	}
	// }

	// helper functions
	fn add_word(solution:&mut [bool;26],word:&str){
		for &letter in word.as_bytes(){
			let letter_id=(letter-b'a') as usize;
			solution[letter_id]=true;
		}
	}
	fn check_solved(puzzle:&Puzzle,solution:[bool;26])->bool{
		for (_,letter) in puzzle{
			let letter_id=(letter as u8-b'a') as usize;
			if !solution[letter_id]{
				return false;
			}
		}
		return true;
	}

	let mut solutions=Vec::new();

	// print out all two word solutions
	for word_list1 in &valid_words.starting_letter{
		for word1 in word_list1{
			let end_letter=*word1.as_bytes().last().unwrap() as char;
			let end_letter_id=(end_letter as u8-b'a') as usize;
			let word_list2=&valid_words.starting_letter[end_letter_id];
			for word2 in word_list2{
				// check if this is a solution
				let mut solution=[false;26];
				add_word(&mut solution, word1);
				add_word(&mut solution, word2);
				if check_solved(&todays_puzzle, solution){
					solutions.push([word1,word2]);
				}
			}
		}
	}

	println!("total elapsed: {:?}",start_time.elapsed());
}
