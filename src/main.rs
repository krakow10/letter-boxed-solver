#[cfg(feature="fetcher")]
mod fetcher;

#[derive(Debug,Clone)]
struct LetterMap{
	next_letter:[Option<Box<LetterMap>>;26],
	is_complete_word:bool,
}
impl LetterMap{
	fn new()->Self{
		LetterMap{
			next_letter:core::array::from_fn(|_|None),
			is_complete_word:false,
		}
	}
}

pub struct Puzzle{
	sides:[[char;Self::SIDE_WIDTH];Self::SIDES],
}
impl Puzzle{
	const SIDES:usize=4;
	const SIDE_WIDTH:usize=3;
	const SIZE:usize=Self::SIDES*Self::SIDE_WIDTH;
	fn from_sides(sides:[String;4])->Self{
		Self{
			sides:sides.map(|side|{
				let side=side.to_ascii_lowercase();
				let mut it=side.chars();
				core::array::from_fn(|_|it.next().unwrap())
			})
		}
	}
}

pub struct PuzzleIter<'a>{
	puzzle:&'a Puzzle,
	index:usize,
}
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
		if self.index<Puzzle::SIZE{
			let side=self.index as usize/Puzzle::SIDE_WIDTH;
			let value=self.puzzle.sides[side][(self.index as usize).rem_euclid(Puzzle::SIDE_WIDTH)];
			self.index+=1;
			Some((side,value))
		}else{
			None
		}
	}
}

struct Words{
	starting_letter:[Vec<String>;26],
}
impl Words{
	fn new()->Self{
		Self{
			starting_letter:core::array::from_fn(|_|Vec::new()),
		}
	}
}

#[derive(Debug,serde::Deserialize)]
#[expect(nonstandard_style,dead_code)]
pub struct LetterBoxed{
	id: u32, //2431,
	expiration: i64, //1754031600,
	ourSolution: Vec<String>, //["DOJO", "OVERHYPING"],
	printDate: String, //"2025-07-31",
	sides: [String;4], //["GJH", "NVY", "EID", "ORP"],
	date: String, //"July 31, 2025",
	dictionary: Vec<String>,
	par: u32, //5,
	yesterdaysSolution: Vec<String>, //["FAILURES", "SYNONYM"],
	yesterdaysSides: [String;4], //["UMA", "IFE", "OLY", "RNS"],
	isFree: bool, //false,
	editor: String, //"Sam Ezersky",
	editorImage: String, //"https:\u002F\u002Fstorage.googleapis.com\u002Fnyt-games-prd.appspot.com\u002Favatars\u002Fsam-ezersky.png"
}

fn generate_tree<'a,I:IntoIterator<Item=&'a str>>(dictionary:I)->LetterMap{
	let mut word_map=LetterMap::new();

	// generate a tree of all words in the dictionary
	'outer: for word in dictionary{
		let mut letter_map=&mut word_map;
		for &letter in word.as_bytes(){
			// skip words containing non-letters
			let letter_id;
			if b'a'<=letter&&letter<=b'z'{
				// lower case
				letter_id=(letter-b'a') as usize;
			}else if b'A'<=letter&&letter<=b'Z'{
				// upper case
				letter_id=(letter-b'A') as usize;
			}else{
				continue 'outer;
			}
			// if the letter map does not exist for this letter,
			// make it, since we just came across a sample word.
			if letter_map.next_letter[letter_id].is_none(){
				letter_map.next_letter[letter_id].replace(Box::new(LetterMap::new()));
			}

			// step letter_map into the next letter
			letter_map=match &mut letter_map.next_letter[letter_id]{
				Some(thing)=>thing,
				None=>panic!("Next letter map (just inserted) does not exist"),
			};
		}
		// mark the end of a word.
		letter_map.is_complete_word=true;
	}

	word_map
}

fn add_next_letter(
	all_words:&mut Vec<String>,
	current_word:&mut String,
	puzzle:&Puzzle,
	letter_map:&LetterMap,
	current_side:usize,
){
	// check if current word is a real word
	if letter_map.is_complete_word{
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

			// push letter onto the end of the current word
			current_word.push(letter);
			add_next_letter(all_words, current_word, puzzle, next_letter_map, side);
			// remove letter
			current_word.pop();
		}
	}
}

fn find_valid_words(word_map:&LetterMap,puzzle:&Puzzle)->Words{
	// initialize an empty list of words
	let mut valid_words=Words::new();

	// brute force search for all valid words
	for (side,starting_letter) in puzzle{
		let starting_letter_id=(starting_letter as u8-b'a') as usize;

		// this will only be skipped if the dictionary is
		// missing all words of a specific starting letter
		if let Some(letter_map)=&word_map.next_letter[starting_letter_id]{
			let mut all_words=Vec::new();
			let mut current_word=String::new();
			current_word.push(starting_letter);
			add_next_letter(&mut all_words, &mut current_word, puzzle, letter_map, side);
			current_word.pop();
			valid_words.starting_letter[starting_letter_id]=all_words;
		}
	}

	valid_words
}

fn find_solutions<'a>(valid_words:&'a Words,puzzle:&Puzzle)->Vec<[&'a str;2]>{
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

	let mut solutions:Vec<[&str;2]> =Vec::new();

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
				if check_solved(puzzle, solution){
					solutions.push([word1,word2]);
				}
			}
		}
	}

	solutions
}

fn main() {
	let letter_boxed=fetcher::get_today().unwrap();

	// const WORDS:&str=include_str!("words.txt");

	let start_time=std::time::Instant::now();
	// let word_map=generate_tree(WORDS.lines());

	let time_generate_word_map=start_time.elapsed();
	let time_1=std::time::Instant::now();

	// println!("{count} in {:?}",start_time.elapsed());
	// println!("total words = {word_count}");

	let todays_puzzle=Puzzle::from_sides(letter_boxed.sides);

	// list all valid words
	// perform a depth first search of the words tree, filtering sides.
	// let valid_words=find_valid_words(&word_map,&todays_puzzle);

	let time_find_valid_words=time_1.elapsed();
	let time_2=std::time::Instant::now();

	// print out all the words
	// for word_list in &valid_words.starting_letter{
	// 	for word in word_list{
	// 		println!("{word}");
	// 	}
	// }

	let valid_words={
		let mut words=Words::new();
		// sort all words and make them lowercase
		for mut word in letter_boxed.dictionary{
			word.make_ascii_lowercase();
			if let Some(first_letter)=word.chars().next(){
				let letter_id=(first_letter as u8-b'a') as usize;
				words.starting_letter[letter_id].push(word);
			}
		}
		words
	};

	let solutions=find_solutions(&valid_words, &todays_puzzle);

	let time_find_solutions=time_2.elapsed();
	let time_total=start_time.elapsed();

	// println!("generate word tree: {time_generate_word_map:?}");
	// println!("find valid words: {time_find_valid_words:?}");
	println!("find solutions: {time_find_solutions:?}");
	println!("total elapsed: {time_total:?}");
	for [word1,word2] in solutions{
		println!("{word1} - {word2}");
	}
}
