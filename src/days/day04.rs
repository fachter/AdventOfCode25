use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

pub fn run_print_optimization() {
    println!("--- Day 4: Printing Department ---");
    day_04("04-test");
    day_04("04");
    println!("--------------------\n\n");
}

fn day_04(file_name: &str) {
    let lines = crate::utils::file::read_lines(file_name);
    let y_len = lines.len();
    let x_len = lines[0].chars().count();
    println!("Dimensions: x={}, y={}", x_len, y_len);

    let mut map: HashMap<(usize, usize), RoleRef> = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char != '@' {
                continue;
            }
            let paper_roll = PaperRoll::new(x, y);
            map.insert((x, y), paper_roll);
            let role_ref = map.get(&(x, y)).unwrap();
            PaperRoll::find_and_connect_neighbors(role_ref, &map);
        }
    }

    let mut movable_rolls = get_movable_rolls(&map);
    let counter = movable_rolls.len();
    println!("Initial Round removed: {}", counter);
    let mut total_counter = movable_rolls.len();

    while movable_rolls.len() > 0 {
        remove_paper_rolls(&mut map, &mut movable_rolls);
        movable_rolls = get_movable_rolls(&map);
        total_counter += movable_rolls.len();
    }

    println!("Total removed: {}", total_counter);

    // Your code goes here
}

fn get_movable_rolls(map: &HashMap<(usize, usize), Rc<RefCell<PaperRoll>>>) -> Vec<RoleRef> {
    let mut movable_rolls: Vec<RoleRef> = Vec::new();
    for paper_roll in map.values() {
        let borrowed = paper_roll.borrow();
        if borrowed.neighbors.len() < 4 {
            movable_rolls.push(Rc::clone(paper_roll));
        }
    }
    movable_rolls
}

fn remove_paper_rolls(
    map: &mut HashMap<(usize, usize), RoleRef>,
    movable_rolls: &mut Vec<RoleRef>,
) {
    for paper_roll in movable_rolls.iter() {
        let borrowed = paper_roll.borrow();
        let position = (borrowed.x, borrowed.y);
        PaperRoll::remove_from_neighbors(&paper_roll);
        map.remove(&position);
    }
}

type RoleRef = Rc<RefCell<PaperRoll>>;

#[derive(Clone, Debug)]
struct PaperRoll {
    x: usize,
    y: usize,
    neighbors: Vec<Weak<RefCell<PaperRoll>>>,
}

impl PaperRoll {
    pub fn new(x: usize, y: usize) -> RoleRef {
        Rc::new(RefCell::new(PaperRoll {
            x,
            y,
            neighbors: Vec::new(),
        }))
    }

    pub fn connect_neighbors(a: &RoleRef, b: &RoleRef) {
        // avoid self-connection
        if Rc::ptr_eq(a, b) {
            return;
        }

        // Create Weak references first (no RefCell borrows yet)
        let a_weak = Rc::downgrade(a);
        let b_weak = Rc::downgrade(b);

        // Then mutably borrow each and push the other's Weak
        a.borrow_mut().neighbors.push(b_weak);
        b.borrow_mut().neighbors.push(a_weak);
    }

    pub fn find_and_connect_neighbors(me: &RoleRef, map: &HashMap<(usize, usize), RoleRef>) {
        let directions = vec![
            (0isize, 1isize),
            (1, 0),
            (0, -1),
            (-1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];

        for (dx, dy) in directions {
            let neighbor_x = me.borrow().x as isize + dx;
            let neighbor_y = me.borrow().y as isize + dy;

            if neighbor_x >= 0 && neighbor_y >= 0 {
                if let Some(neighbor) = map.get(&(neighbor_x as usize, neighbor_y as usize)) {
                    PaperRoll::connect_neighbors(&me, &neighbor);
                }
            }
        }
    }

    fn remove_from_neighbors(me: &RoleRef) {
        // Upgrade Weak references to strong Rc and remove 'me' from their neighbors
        for neighbor_weak in &me.borrow().neighbors {
            if let Some(neighbor_rc) = neighbor_weak.upgrade() {
                neighbor_rc
                    .borrow_mut()
                    .neighbors
                    .retain(|n| n.upgrade().map_or(false, |strong| !Rc::ptr_eq(&strong, me)));
            }
        }

        // // Clear our own neighbors
        // me.borrow_mut().neighbors.clear();
    }
}
