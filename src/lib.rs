use std::io::{BufRead, BufReader};
use std::fs::File;

// so the algorithm is:
// first parse the header, which tells us the types
// that is collected as a vector

// then we repeatedly iterate over that,
// returning a result if we run out of file in the middle of an enumeration (so when the index isn't either 0 or length)


// for (index, column_struct) in column_structs.iter().enumerate()

// https://doc.rust-lang.org/std/str/fn.from_utf8.html
// std::str::from_utf8(v: &[u8]) -> Result<&str, Utf8Error>
// std::String::from_utf8(vec: Vec<u8>) -> Result<String, FromUtf8Error>



pub fn do_stuff() -> Result<(), ()> {
	let mut f = BufReader::new(File::open("./test.typed.csv").expect("open failed"));

	let mut buf = Vec::<u8>::new();

	// grab first line
	f.read_until(b'\n', &mut buf).expect("read_until");
	if let None = buf.pop() {
		return Err(());
	}

	let s = dbg!("some stuff");
	let p = dbg!(&s[..]);
	let _ = dbg!(&p[2..]);

	// let header_column_strings: Vec<&str> = dbg!(String::from_utf8(buf).expect("from_utf8")
	let _: Vec<&str> = dbg!(String::from_utf8(buf).expect("from_utf8")
		.split(',')
		.collect());

	Ok(())
	// for &column_string in header_column_strings {
	// 	let portions: Vec<&str> = column_string.split(':').collect();
	// 	if portions.len() != 2 {
	// 		return Err(())
	// 	}
	// }

	// let mut columns = Vec::new();
	// let column_number = columns.len();

	// buf = s.into_bytes();
	// buf.clear();

	// while f.read_until(b'\n', &mut buf).expect("read_until failed") != 0 {
	// 	// this moves the ownership of the read data to s
	// 	// there is no allocation
	// 	let s = String::from_utf8(buf).expect("from_utf8 failed");
	// 	for c in s.chars() {
	// 		println!("Character: {}", c);
	// 	}
	// 	// this returns the ownership of the read data to buf
	// 	// there is no allocation
	// 	buf = s.into_bytes();
	// 	buf.clear();
	// }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		assert_eq!(do_stuff(), Ok(()));
	}
}
