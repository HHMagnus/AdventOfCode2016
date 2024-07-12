use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/day14.txt").unwrap();

	let hashes = (0..50000).map(|x| md5::compute(format!("{}{}", file, x))).map(|x| format!("{:?}", x)).map(|x| x.chars().into_iter().collect::<Vec<_>>()).collect::<Vec<_>>();

	let mut fives = Vec::new();

	for (i, x) in hashes.iter().enumerate() {
		for window in x.windows(5) {
			if window.iter().all(|&w| w == window[0]) {
				fives.push((i, window[0]));
			}
		}
	}

	let mut key = 0;

	for (i, x) in hashes.iter().enumerate() {
		for window in x.windows(3) {
			if window[0] == window[1] && window[1] == window[2] {
				if fives.iter().any(|&(i0, x0)| i < i0 && i0 < i+1000 && x0 == window[0]) {
					key += 1;
					if key == 64 {
						println!("Day 14 part 1: {}", i);
					}
				}
				break;
			}
		}
	}

	let mut i = -1000;
	let mut key = 0;
	let mut fives = Vec::new();
	let mut hashes = Vec::new();

	while key < 64 {
		let mut hash = format!("{}{}", file, i+1000);

		for _ in 0..2017 {
			hash = format!("{:?}", md5::compute(hash));
		}
		let hash = hash.chars().into_iter().collect::<Vec<_>>();
		hashes.push(hash.clone());
		
		for window in hash.windows(5) {
			if window.iter().all(|&w| w == window[0]) {
				fives.push((i+1000, window[0]));
			}
		}

		if i >= 0 {
			let hash = &hashes[i as usize];
			for window in hash.windows(3) {
				if window[0] == window[1] && window[1] == window[2] {
					if fives.iter().any(|&(i0, x0)| i < i0 && i0 < i+1000 && x0 == window[0]) {
						key += 1;
						if key == 64 {
							println!("Day 14 part 2: {}", i);
						}
					}
					break;
				}
			}
		}

		i += 1;
	}
}