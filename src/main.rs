use bit_vec::BitVec;
use std::collections::HashSet;

trait SudokuElem {
    fn is_solved(&self) -> bool;
    fn solutions(&self) -> Vec<BitVec>;
    fn is_invalid(&self) -> bool;
    fn print(&self) -> String;
}

trait Sudoku {
    fn print(&self);
    fn print_compact(&self);
    fn cascade(&mut self,idx:&Vec<usize>) -> bool;
    fn cascade_over_sets(&mut self, unique_sets:&Vec<Vec<usize>>);
    fn is_invalid(&self) -> bool; 
    fn row_size(&self) -> usize;
}

impl SudokuElem for BitVec {
    /// Is this square solved?
    ///
    /// A solved sudoku square has only one possible answer
    fn is_solved(&self) -> bool {
	self.iter().filter(|x| *x).count() == 1
    }

    /// Which solutions are left?
    /// 
    /// Since we are storing each possible answer as a bool
    /// we just return a new bitvec for each true in our bitvec
    fn solutions(&self) -> Vec<BitVec> {
        self.iter().enumerate()
            .filter(|(_,r)| *r)
            .map(|(i,_)| {
		let mut a = BitVec::from_elem(9,false); 
		a.set(i,true);
		a
		})
            .collect()
    }

    /// Is this square still valid?
    ///
    /// A sudoku square is invalid if there are no possible answers
    fn is_invalid(&self) -> bool {
        self.none()
    }

    /// Print the possible values of this square
    fn print(&self) -> String {
        let index:String = self.iter().enumerate()
            .filter(|(_,r)| *r)
            .map(|(i,_)| format!("{}",i+1))
            .collect();
        return index;
    }
}

impl Sudoku for Vec<BitVec> {
	fn cascade(&mut self, idx:&Vec<usize>) -> bool {
		let mut changed_ever = false;
		loop { 
			let mut changed = false;
			let solved_mask = idx.iter()
				.map(|i| &self[*i])
				.filter_map(|e| if e.is_solved() {Some(e)} else {None} )
				.fold(BitVec::from_elem(9,false), |mut solved,i| {solved.or(i); solved} );

			let mut set:HashSet<String> = HashSet::with_capacity(9);
			for i in idx {
				let e = &mut self[*i];
				//If its not solved, subtract the solved masked
				if !e.is_solved() && e.difference(&solved_mask) {
					changed = true;
				}
				//Then, if its now solved, dedup it
				if e.is_solved() && !set.insert(e.print()) {
					e.clear(); //Duplicates have no valid solutions
				}
			}
			// if something changed, make a note. Otherwise we are done.
			if changed { changed_ever = true }
			else { break }
		}
		changed_ever
	}

	fn cascade_over_sets(&mut self, unique_sets:&Vec<Vec<usize>>) {
		loop {
			//cascade every set until nothing changes.
			if !unique_sets.iter().map(|s| self.cascade(s)).fold(false, |a,b| a||b) {break}
		}
	}

	fn print(&self) {
		let r = self.row_size();
		for i in 0..self.len() {
			println!("({},{}) {}", i/r,i%r, self[i].print());
		}
	}
	fn print_compact(&self) {
		let r = self.row_size();
		for i in 0..r {
			for j in 0..r {
				print!("{}",match &self[r*i+j] {
				  e if e.is_solved() => e.print(),
				  e if e.is_invalid() => String::from("X"),
				  _ => String::from("?"),
				})
			}
			println!("")
		}
	}

	fn is_invalid(&self) -> bool {
		self.iter().any(|e| e.is_invalid())
	}

	fn row_size(&self) -> usize {
		(self.len() as f64).sqrt() as usize
	}
}

fn main() {
	let sudoku:Vec<BitVec> = {
		let sudoku = vec![
		0, 0, 9, 4, 7, 0, 0, 0, 0,
		8, 0, 6, 2, 0, 0, 7, 0, 0,
		0, 0, 0, 0, 0, 1, 0, 0, 0,
		9, 0, 3, 0, 0, 0, 0, 4, 0,
		7, 1, 0, 0, 0, 0, 0, 5, 6,
		0, 2, 0, 0, 0, 0, 8, 0, 3,
		0, 0, 0, 6, 0, 0, 0, 0, 0,
		0, 0, 7, 0, 0, 4, 9, 0, 8,
		0, 0, 0, 0, 3, 7, 4, 0, 0
		];
		sudoku.iter().map(|x| build_elem(x)).collect()
	};

	let size = sudoku.len();
	let row_size = sudoku.row_size();
	let mut unique_sets:Vec<Vec<usize>> = Vec::new();
	unique_sets.extend((0..size).step_by(row_size).map(|i| (i..i+row_size).collect()));
	unique_sets.extend((0..row_size).map(|i| (i..size).step_by(row_size).collect()));
	unique_sets.extend(
			vec![0,3,6,27,30,33,54,57,60].iter()
			.map(|i| vec![0+i,1+i,2+i,9+i,10+i,11+i,18+i,19+i,20+i])
			);


	match try_solutions(sudoku, 0, &unique_sets) {
		Some(sol) => sol.print_compact(),
			None => println!("no solutions"),
	}
}

fn try_solutions(mut sudoku:Vec<BitVec>, idx:usize, unique_sets:&Vec<Vec<usize>>) -> Option<Vec<BitVec>> {
	sudoku.cascade_over_sets(&unique_sets);
	if sudoku.is_invalid() {
		return None;
	}

	match {
		{idx..sudoku.len()}.filter(|n| match sudoku.get(*n) { 
				Some(x) => !x.is_solved(),
				None => false,
				}).next()
	} {
		Some(next_idx) => {
			sudoku[next_idx].solutions().iter()
				.filter_map(|solution| {
						let mut test = sudoku.to_vec();
						test[next_idx] = solution.clone();
						try_solutions(test, next_idx, unique_sets)
						}).next()
		},
		None =>	Some(sudoku),
	}
}

fn build_elem(num:&usize) -> BitVec {
	let mut bv = BitVec::from_elem(9,false);
	if *num == 0 {
		bv.negate(); // 0 means it can be any number 1-9.
	}
	else {
		bv.set(*num-1,true); 
	}
	return bv;
}
