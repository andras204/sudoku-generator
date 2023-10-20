use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    bitmap: u16
}


impl Cell {
    pub fn new() -> Cell {
        return Cell { bitmap: 0b_0000_0001_1111_1111 }
    }

    pub fn collapse(&mut self, num: u8) -> Result<u8, &str> {
        if self.is_collapsed() { return Err("cell already collapsed"); }

        let mask: u16 = 0b_0001 << num;
        self.bitmap = mask;

        Ok(0)
    }

    pub fn reduce(&mut self, num: u8) -> Result<bool, &str> {
        let mask: u16 = 0b_0001 << num;
        self.bitmap |= mask;
        self.bitmap ^= mask;

        if self.get_count() == 0 {
            self.bitmap |= mask;
            return Err("Invalid collapse");
        }

        Ok(self.is_collapsed())
    }

    pub fn reset(&mut self) {
        self.bitmap = 0b_0000_0001_1111_1111;
    }

    pub fn get_possible_numbers(&self) -> Vec<u8> {
        let mut nums: Vec<u8> = Vec::with_capacity(self.get_count() as usize);
        for x in 0..9 {
            if self.bitmap & (0b_0000_0001 << x) as u16 != 0 {
                nums.push(x as u8);
            }
        }
        nums
    }
    
    pub fn get_number(&self) -> Result<u8, &str> {
        if !self.is_collapsed() {
            return Err("Cell not collapsed");
        }
        for x in 0..9 {
            if self.bitmap & (0b_0000_0001 << x) as u16 != 0 {
                return Ok(x);
            }
        }
        return Err("Cell invalid");
    }
    
    pub fn get_count(&self) -> u8 {
        let mut count: u8 = 0;
        for x in 0..9 {
            if self.bitmap & (0b_0000_0001 << x) as u16 != 0 {
                count += 1;
            }
        }
        count
    }

    pub fn is_collapsed(&self) -> bool {
        self.get_count() == 1
    }
}

#[test]
fn get_number_test() {
    let mut rng = rand::thread_rng();
    let mut t_set: Vec<u8> = (0..9).collect();
    let mut t_cell: Cell = Cell::new();

    t_set.shuffle(&mut rng);

    while t_set.len() > 1 {
        assert_eq!(t_cell.get_count(), t_set.len() as u8);
        t_cell.reduce(t_set.pop().unwrap()).unwrap();
        assert_eq!(t_cell.get_count(), t_set.len() as u8);
    }
}

#[test]
fn is_collapsed_test() {
    let mut rng = rand::thread_rng();
    let mut t_set: Vec<u8> = (0..9).collect();
    let mut t_cell: Cell = Cell::new();

    t_set.shuffle(&mut rng);

    t_set.pop();

    for x in t_set {
        assert!(!t_cell.is_collapsed());
        t_cell.reduce(x).unwrap();
    }
    assert!(t_cell.is_collapsed());
}

#[test]
fn collapse_test() {
    let mut rng = rand::thread_rng();
    let mut t_set: Vec<u8> = (0..9).collect();
    let mut t_cell: Cell = Cell::new();

    t_set.shuffle(&mut rng);

    t_cell.collapse(t_set.pop().unwrap()).unwrap();

    while t_set.len() > 0 {
        match t_cell.collapse(t_set.pop().unwrap()) {
            Ok(_ok) => panic!(),
            Err(_err) => continue
        }
    }

}

#[test]
fn reduce_test() {
    let mut rng = rand::thread_rng();
    let mut t_set: Vec<u8> = (0..9).collect();
    let mut t_cell: Cell = Cell::new();

    t_set.shuffle(&mut rng);

    while t_set.len() > 1 {
        t_cell.reduce(t_set.pop().unwrap()).unwrap();
    }

    match t_cell.reduce(t_set.pop().unwrap()) {
        Ok(_ok) => panic!(),
        Err(_err) => {}
    }
}

#[test]
fn if_collapse_get_number() {
    let mut rng = rand::thread_rng();
    let mut t_set: Vec<u8> = (0..9).collect();
    let mut t_cell: Cell = Cell::new();

    t_set.shuffle(&mut rng);

    while t_set.len() > 1 {
        if t_cell.reduce(t_set.pop().unwrap()).unwrap() {
            t_cell.get_number().unwrap();
        }
        else {
            match t_cell.get_number() {
                Ok(_ok) => panic!(),
                Err(_err) => continue
            }
        }
    }
}
