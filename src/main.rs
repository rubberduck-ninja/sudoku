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
    fn cascade_once(&mut self,idx:&Vec<usize>)->bool;
    fn cascade(&mut self,idx:&Vec<usize>) -> bool;
    fn cascade_over_sets(&mut self, unique_sets:&Vec<Vec<usize>>);
    fn is_invalid(&self) -> bool; 
}

impl SudokuElem for BitVec {
    fn is_solved(&self) -> bool {
	self.iter().filter(|x| *x).count() == 1
    }

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

    fn is_invalid(&self) -> bool {
        self.none()
    }

    fn print(&self) -> String {
        let index:String = self.iter().enumerate()
            .filter(|(_,r)| *r)
            .map(|(i,_)| format!("{}",i+1))
            .collect();
        return index;
    }
}

impl Sudoku for Vec<BitVec> {
	fn cascade_once(&mut self, idx:&Vec<usize>) -> bool {
		let mut changed = false;
		let mut solved_mask = BitVec::from_elem(9,false);
		for i in idx {
			if self[*i].is_solved() {
				solved_mask.or(&self[*i]);
			}
		}
		for i in idx {
			let e = &mut self[*i];
			if ! e.is_solved() {
				if e.difference(&solved_mask) {
					changed = true;
				}
			}
		}
		let mut set:HashSet<String> = HashSet::with_capacity(9);
		for i in idx {
			if self[*i].is_solved() { 
				if !set.insert(self[*i].print()) {
					self[*i] = BitVec::from_elem(9,false); //Duplicates are wrong
				}
			}
		}
		changed
	}

	fn cascade(&mut self, idx:&Vec<usize>) -> bool {
		let mut changed = false;
		while { self.cascade_once(idx) } {
			changed = true;
		}
		changed
	}
	fn cascade_over_sets(&mut self, unique_sets:&Vec<Vec<usize>>) {
		let mut changed = true;
		while changed {
			changed = false;
			for s in unique_sets.iter() {
				if self.cascade(s) {
					changed = true;
				}
			}
		}
	}

	fn print(&self) {
		for i in 0..81 {
			println!("({},{}) {}", i/9,i%9, self[i].print());
		}
	}
	fn print_compact(&self) {
		for i in 0..9 {
			for j in 0..9 {
				let e = &self[9*i+j];
				if e.is_solved() {
					print!("{}", self[9*i+j].print());
				} else if e.is_invalid() {
					print!("X");
				} else {
					print!("?");
				}
			}
			println!("")
		}
	}

	fn is_invalid(&self) -> bool {
		self.iter().any(|e| e.is_invalid())
	}
}

fn main() {
    let mut unique_sets:Vec<Vec<usize>> = Vec::new();
    unique_sets.extend((0..81).step_by(9).map(|i| (i..i+9).collect()));
    unique_sets.extend((0..9).map(|i| (i..81).step_by(9).collect()));
    unique_sets.extend(
		    vec![0,3,6,27,30,33,54,57,60].iter()
		    .map(|i| vec![0+i,1+i,2+i,9+i,10+i,11+i,18+i,19+i,20+i])
		    );

    let sudoku = vec![
        //0, 0, 9, 4, 7, 0, 0, 0, 0,
        1, 5, 9, 4, 7, 8, 6, 0, 0,
        8, 0, 6, 2, 0, 0, 7, 0, 0,
        0, 0, 0, 0, 0, 1, 0, 0, 0,
        9, 0, 3, 0, 0, 0, 0, 4, 0,
        7, 1, 0, 0, 0, 0, 0, 5, 6,
        0, 2, 0, 0, 0, 0, 8, 0, 3,
        0, 0, 0, 6, 0, 0, 0, 0, 0,
        0, 0, 7, 0, 0, 4, 9, 0, 8,
        0, 0, 0, 0, 3, 7, 4, 0, 0
    ];

    let mut sudoku:Vec<BitVec> = sudoku.iter().map(|x| build_elem(x)).collect();
    sudoku.cascade_over_sets(&unique_sets);
    match try_solutions(sudoku, 0, &unique_sets) {
	Some(sol) => sol.print_compact(),
	None => println!("no solutions"),
    }
}

fn try_solutions(sudoku:Vec<BitVec>, idx:usize, unique_sets:&Vec<Vec<usize>>) -> Option<Vec<BitVec>> {
	if sudoku.is_invalid() {
		return None;
	}

	match {
	{idx..80}.filter(|n| match sudoku.get(*n) { 
		Some(x) => !x.is_solved(),
		None => false,
	}).next()
	} {
		Some(next_idx) => {
			sudoku[next_idx].solutions().iter()
				.filter_map(|solution| {
						let mut test = sudoku.to_vec();
						test[next_idx] = solution.clone();
						test.cascade_over_sets(unique_sets);
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
