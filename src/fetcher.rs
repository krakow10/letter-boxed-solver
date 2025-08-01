// parse the word list out of the official puzzle.
use crate::LetterBoxed;

#[expect(unused)]
#[derive(Debug)]
pub enum Error{
	Reqwuest(reqwest::Error),
	Utf8(core::str::Utf8Error),
	Io(std::io::Error),
	Json(serde_json::Error),
	// generic failure
	Failed,
}
pub fn get_today()->Result<LetterBoxed,Error>{
	// http client
	let client=reqwest::blocking::Client::new();

	// construct a simple GET request
	let request=client.get("https://www.nytimes.com/puzzles/letter-boxed");

	// send the request and throw an error if there is a bad status
	let response=request
		.send()
		.map_err(Error::Reqwuest)?
		.error_for_status()
		.map_err(Error::Reqwuest)?;

	// get the body
	let body_bytes=response.bytes().map_err(Error::Reqwuest)?;
	let body_str=core::str::from_utf8(body_bytes.as_ref()).map_err(Error::Utf8)?;

	// make a handle to the body as an html document
	let doc=select::document::Document::from(body_str);

	// looking for a script that starts with this
	const PREFIX:&str="window.gameData = ";

	// scan for scripts
	for script in doc.find(select::predicate::Attr("type","text/javascript")){
		if let Some(child)=script.first_child(){
			if let Some(text)=child.as_text(){
				if let Some((prefix,rest))=text.split_at_checked(PREFIX.len()){
					if prefix==PREFIX{
						if let Some(first_line)=rest.lines().next(){
							// found it! parse it as json into the LetterBoxed struct
							let value=serde_json::from_str(first_line).map_err(Error::Json)?;
							return Ok(value);
						}
					}
				}
			}
		}
	}

	return Err(Error::Failed);
}
