use aoc_runner_derive::aoc;
use std::cell::RefCell;
use std::collections::HashSet;
use std::ops::Range;
use std::rc::Rc;

#[aoc(day20, part1)]
fn solve_part1(input: &str) -> u64 {
    let graph: Graph = input.into();
    let top_left = graph.orient();
    let mut product = top_left.borrow().id;

    let mut curr = top_left;
    loop {
        let neighbor = curr.borrow().right_neighbor();

        match neighbor {
            Some(tile) => {
                curr = tile;
            }
            None => break,
        }
    }
    product *= curr.borrow().id;

    loop {
        let neighbor = curr.borrow().bottom_neighbor();

        match neighbor {
            Some(tile) => {
                curr = tile;
            }
            None => break,
        }
    }
    product *= curr.borrow().id;

    loop {
        let neighbor = curr.borrow().left_neighbor();

        match neighbor {
            Some(tile) => {
                curr = tile;
            }
            None => break,
        }
    }
    product *= curr.borrow().id;

    product
}

#[aoc(day20, part2)]
fn solve_part2(input: &str) -> usize {
    use Monster::*;
    let graph: Graph = input.into();
    let image: Image = graph.orient().into();

    for monster in &[
        RightUp, RightDown, LeftUp, LeftDown, UpRight, UpLeft, DownRight, DownLeft,
    ] {
        let found = image.find_monsters(*monster);

        if !found.is_empty() {
            let total_hash: usize = image
                .data
                .iter()
                .map(|line| line.iter().filter(|c| **c == '#').count())
                .sum();
            return total_hash - found.len();
        }
    }

    0
}

struct Image {
    data: Vec<Vec<char>>,
}

impl Image {
    fn find_monsters(&self, monster: Monster) -> HashSet<(usize, usize)> {
        let mut found = HashSet::new();

        let (x_bounds, y_bounds) = self.get_bounds(monster);

        for x in x_bounds {
            for y in y_bounds.clone() {
                if self.is_monster(x, y, monster) {
                    found.extend(monster.points(x, y));
                }
            }
        }

        found
    }

    fn is_monster(&self, x: usize, y: usize, monster: Monster) -> bool {
        for (x1, y1) in monster.points(x, y) {
            if self.data[y1][x1] != '#' {
                return false;
            }
        }
        true
    }

    fn get_bounds(&self, monster: Monster) -> (Range<usize>, Range<usize>) {
        use Monster::*;
        let y_len = self.data.len();
        let x_len = self.data[0].len();

        match monster {
            RightUp | RightDown => (0..x_len - 19, 1..y_len - 1),
            LeftUp | LeftDown => (19..x_len, 1..y_len - 1),
            DownLeft | DownRight => (1..x_len - 1, 0..y_len - 19),
            UpLeft | UpRight => (1..x_len - 1, 19..y_len),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Monster {
    RightUp,
    RightDown,
    LeftUp,
    LeftDown,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Monster {
    fn points(self, x: usize, y: usize) -> Vec<(usize, usize)> {
        use Monster::*;
        match self {
            RightUp => vec![
                (x, y),
                (x + 5, y),
                (x + 6, y),
                (x + 11, y),
                (x + 12, y),
                (x + 17, y),
                (x + 18, y),
                (x + 19, y),
                (x + 18, y - 1),
                (x + 1, 1),
                (x + 4, 1),
                (x + 7, 1),
                (x + 10, 1),
                (x + 13, 1),
                (x + 16, 1),
            ],
            RightDown => vec![
                (x, y),
                (x + 5, y),
                (x + 6, y),
                (x + 11, y),
                (x + 12, y),
                (x + 17, y),
                (x + 18, y),
                (x + 19, y),
                (x + 18, y + 1),
                (x + 1, y - 1),
                (x + 4, y - 1),
                (x + 7, y - 1),
                (x + 10, y - 1),
                (x + 13, y - 1),
                (x + 16, y - 1),
            ],
            LeftUp => vec![
                (x, y),
                (x - 6, y),
                (x - 5, y),
                (x - 11, y),
                (x - 12, y),
                (x - 17, y),
                (x - 18, y),
                (x - 19, y),
                (x - 18, y - 1),
                (x - 1, y + 1),
                (x - 4, y + 1),
                (x - 7, y + 1),
                (x - 10, y + 1),
                (x - 13, y + 1),
                (x - 16, y + 1),
            ],
            LeftDown => vec![
                (x, y),
                (x - 5, y),
                (x - 6, y),
                (x - 11, y),
                (x - 12, y),
                (x - 17, y),
                (x - 18, y),
                (x - 19, y),
                (x - 18, y + 1),
                (x - 1, y - 1),
                (x - 4, y - 1),
                (x - 7, y - 1),
                (x - 10, y - 1),
                (x - 13, y - 1),
                (x - 16, y - 1),
            ],
            DownLeft => vec![
                (x, y),
                (x, y + 5),
                (x, y + 6),
                (x, y + 11),
                (x, y + 12),
                (x, y + 17),
                (x, y + 18),
                (x, y + 19),
                (x - 1, y + 18),
                (x + 1, y + 1),
                (x + 1, y + 4),
                (x + 1, y + 7),
                (x + 1, y + 10),
                (x + 1, y + 13),
                (x + 1, y + 16),
            ],
            DownRight => vec![
                (x, y),
                (x, y + 5),
                (x, y + 6),
                (x, y + 11),
                (x, y + 12),
                (x, y + 17),
                (x, y + 18),
                (x, y + 19),
                (x + 1, y + 18),
                (x - 1, y + 1),
                (x - 1, y + 4),
                (x - 1, y + 7),
                (x - 1, y + 10),
                (x - 1, y + 13),
                (x - 1, y + 16),
            ],
            UpLeft => vec![
                (x, y),
                (x, y - 5),
                (x, y - 6),
                (x, y - 11),
                (x, y - 12),
                (x, y - 17),
                (x, y - 18),
                (x, y - 19),
                (x - 1, y - 18),
                (x + 1, y - 1),
                (x + 1, y - 4),
                (x + 1, y - 7),
                (x + 1, y - 10),
                (x + 1, y - 13),
                (x + 1, y - 16),
            ],
            UpRight => vec![
                (x, y),
                (x, y - 5),
                (x, y - 6),
                (x, y - 11),
                (x, y - 12),
                (x, y - 17),
                (x, y - 18),
                (x, y - 19),
                (x, y - 18),
                (x - 1, y - 1),
                (x - 1, y - 4),
                (x - 1, y - 7),
                (x - 1, y - 10),
                (x - 1, y - 13),
                (x - 1, y - 16),
            ],
        }
    }
}

impl From<Rc<RefCell<GraphTile>>> for Image {
    fn from(top_left: Rc<RefCell<GraphTile>>) -> Self {
        let mut data = Vec::new();

        let mut curr = top_left;
        loop {
            process_row(curr.clone(), &mut data);

            let next = curr.borrow().bottom_neighbor();
            match next {
                Some(tile) => {
                    curr = tile;
                }
                None => break,
            }
        }

        Image { data }
    }
}

fn process_row(start: Rc<RefCell<GraphTile>>, image: &mut Vec<Vec<char>>) {
    let index_offset = image.len();

    for row in start.borrow().data() {
        image.push(row.collect());
    }

    let mut curr = start;
    loop {
        let next = curr.borrow().right_neighbor();
        match next {
            Some(tile) => {
                curr = tile;
            }
            None => break,
        }

        for (index, row) in curr.borrow().data().enumerate() {
            image[index + index_offset].extend(row);
        }
    }
}

struct GraphTile {
    data: Vec<Vec<char>>,
    id: u64,
    top: Option<Rc<RefCell<GraphTile>>>,
    bottom: Option<Rc<RefCell<GraphTile>>>,
    left: Option<Rc<RefCell<GraphTile>>>,
    right: Option<Rc<RefCell<GraphTile>>>,
}

impl GraphTile {
    fn top_border(&self) -> String {
        self.data[0].iter().collect()
    }

    fn bottom_border(&self) -> String {
        self.data[9].iter().collect()
    }

    fn left_border(&self) -> String {
        self.data.iter().map(|line| line[0]).collect()
    }

    fn right_border(&self) -> String {
        self.data.iter().map(|line| line[9]).collect()
    }

    fn data(&self) -> impl Iterator<Item = impl Iterator<Item = char> + '_> + '_ {
        self.data
            .iter()
            .skip(1)
            .take(8)
            .map(|row| row.iter().skip(1).take(8).copied())
    }

    // fn top_neighbor(&self) -> Option<Rc<RefCell<GraphTile>>> {
    //     self.top.clone()
    // }

    fn bottom_neighbor(&self) -> Option<Rc<RefCell<GraphTile>>> {
        self.bottom.clone()
    }

    fn left_neighbor(&self) -> Option<Rc<RefCell<GraphTile>>> {
        self.left.clone()
    }

    fn right_neighbor(&self) -> Option<Rc<RefCell<GraphTile>>> {
        self.right.clone()
    }

    fn rotate_right(&mut self) {
        let mut vec_0 = Vec::with_capacity(10);
        let mut vec_1 = Vec::with_capacity(10);
        let mut vec_2 = Vec::with_capacity(10);
        let mut vec_3 = Vec::with_capacity(10);
        let mut vec_4 = Vec::with_capacity(10);
        let mut vec_5 = Vec::with_capacity(10);
        let mut vec_6 = Vec::with_capacity(10);
        let mut vec_7 = Vec::with_capacity(10);
        let mut vec_8 = Vec::with_capacity(10);
        let mut vec_9 = Vec::with_capacity(10);

        for row in self.data.iter().rev() {
            vec_0.push(row[0]);
            vec_1.push(row[1]);
            vec_2.push(row[2]);
            vec_3.push(row[3]);
            vec_4.push(row[4]);
            vec_5.push(row[5]);
            vec_6.push(row[6]);
            vec_7.push(row[7]);
            vec_8.push(row[8]);
            vec_9.push(row[9]);
        }

        self.data = vec![
            vec_0, vec_1, vec_2, vec_3, vec_4, vec_5, vec_6, vec_7, vec_8, vec_9,
        ];
    }

    fn rotate_left(&mut self) {
        let mut vec_0 = Vec::with_capacity(10);
        let mut vec_1 = Vec::with_capacity(10);
        let mut vec_2 = Vec::with_capacity(10);
        let mut vec_3 = Vec::with_capacity(10);
        let mut vec_4 = Vec::with_capacity(10);
        let mut vec_5 = Vec::with_capacity(10);
        let mut vec_6 = Vec::with_capacity(10);
        let mut vec_7 = Vec::with_capacity(10);
        let mut vec_8 = Vec::with_capacity(10);
        let mut vec_9 = Vec::with_capacity(10);

        for row in self.data.iter() {
            vec_0.push(row[9]);
            vec_1.push(row[8]);
            vec_2.push(row[7]);
            vec_3.push(row[6]);
            vec_4.push(row[5]);
            vec_5.push(row[4]);
            vec_6.push(row[3]);
            vec_7.push(row[2]);
            vec_8.push(row[1]);
            vec_9.push(row[0]);
        }

        self.data = vec![
            vec_0, vec_1, vec_2, vec_3, vec_4, vec_5, vec_6, vec_7, vec_8, vec_9,
        ];
    }

    fn rotate_180(&mut self) {
        self.data = self
            .data
            .iter()
            .rev()
            .map(|row| row.iter().copied().rev().collect())
            .collect();
    }

    fn flip_horizontal(&mut self) {
        self.data = self
            .data
            .iter()
            .map(|row| row.iter().copied().rev().collect())
            .collect();
    }

    fn flip_vertical(&mut self) {
        self.data = self.data.iter().rev().cloned().collect();
    }
}

impl PartialEq for GraphTile {
    fn eq(&self, other: &GraphTile) -> bool {
        self.id == other.id
    }
}

impl<'a> From<&'a str> for GraphTile {
    fn from(input: &'a str) -> Self {
        let mut lines = input.lines();
        let first = lines.next().unwrap();
        let id = first[5..first.len() - 1].parse().unwrap();
        let data = lines.map(|l| l.chars().collect()).collect();

        GraphTile {
            data,
            id,
            top: None,
            bottom: None,
            left: None,
            right: None,
        }
    }
}

struct Graph {
    tiles: Vec<Rc<RefCell<GraphTile>>>,
}

impl Graph {
    fn orient(mut self) -> Rc<RefCell<GraphTile>> {
        let start = self.tiles.pop().unwrap();

        self.orient_inner(start.clone());

        let mut current = start;
        loop {
            let maybe_tile = current.borrow().top.as_ref().cloned();
            match maybe_tile {
                Some(tile) => {
                    current = tile;
                }
                None => {
                    break;
                }
            }
        }
        loop {
            let maybe_tile = current.borrow().left.as_ref().cloned();
            match maybe_tile {
                Some(tile) => {
                    current = tile;
                }
                None => {
                    break;
                }
            }
        }
        current
    }

    fn orient_inner(&mut self, curr: Rc<RefCell<GraphTile>>) {
        // If curr.top is None -> Try to find a match for top
        // If we find something, then orient it, attach it, and store it for recursion
        // Repeat for bottom, left and right
        // At the end, recurse into each stored value
        let mut matched = Vec::with_capacity(4);
        // Top
        if curr.borrow().top.is_none() {
            let top = curr.borrow().top_border();
            let top_rev: String = top.chars().rev().collect();

            let mut found = None;
            for tile in &self.tiles {
                if curr.borrow().id != tile.borrow().id {
                    let cmp_top = tile.borrow().top_border();
                    if cmp_top == top {
                        tile.borrow_mut().flip_vertical();
                        found = Some(tile.clone());
                        break;
                    } else if cmp_top == top_rev {
                        tile.borrow_mut().rotate_180();
                        found = Some(tile.clone());
                        break;
                    }

                    let cmp_bottom = tile.borrow().bottom_border();
                    if cmp_bottom == top {
                        found = Some(tile.clone());
                        break;
                    } else if cmp_bottom == top_rev {
                        tile.borrow_mut().flip_horizontal();
                        found = Some(tile.clone());
                        break;
                    }

                    let cmp_left = tile.borrow().left_border();
                    if cmp_left == top {
                        tile.borrow_mut().rotate_left();
                        found = Some(tile.clone());
                        break;
                    } else if cmp_left == top_rev {
                        tile.borrow_mut().rotate_right();
                        tile.borrow_mut().flip_vertical();
                        found = Some(tile.clone());
                        break;
                    }

                    let cmp_right = tile.borrow().right_border();
                    if cmp_right == top {
                        tile.borrow_mut().rotate_left();
                        tile.borrow_mut().flip_vertical();
                        found = Some(tile.clone());
                        break;
                    } else if cmp_right == top_rev {
                        tile.borrow_mut().rotate_right();
                        found = Some(tile.clone());
                        break;
                    }
                }
            }

            if let Some(tile) = found {
                tile.borrow_mut().bottom = Some(curr.clone());
                curr.borrow_mut().top = Some(tile.clone());
                matched.push(tile);
            }
        }

        // Bottom
        if curr.borrow().bottom.is_none() {
            let bottom = curr.borrow().bottom_border();
            let bottom_rev: String = bottom.chars().rev().collect();

            let mut found = None;
            for tile in &self.tiles {
                if curr.borrow().id != tile.borrow().id {
                    let cmp_top = tile.borrow().top_border();
                    if cmp_top == bottom {
                        found = Some(tile.clone());
                        break;
                    } else if cmp_top == bottom_rev {
                        tile.borrow_mut().flip_horizontal();
                        found = Some(tile.clone());
                        break;
                    }

                    let cmp_bottom = tile.borrow().bottom_border();
                    if cmp_bottom == bottom {
                        tile.borrow_mut().flip_vertical();
                        found = Some(tile.clone());
                        break;
                    } else if cmp_bottom == bottom_rev {
                        tile.borrow_mut().rotate_180();
                        found = Some(tile.clone());
                        break;
                    }

                    let cmp_left = tile.borrow().left_border();
                    if cmp_left == bottom {
                        tile.borrow_mut().rotate_left();
                        tile.borrow_mut().flip_vertical();
                        found = Some(tile.clone());
                        break;
                    } else if cmp_left == bottom_rev {
                        tile.borrow_mut().rotate_right();
                        found = Some(tile.clone());
                        break;
                    }

                    let cmp_right = tile.borrow().right_border();
                    if cmp_right == bottom {
                        tile.borrow_mut().rotate_left();
                        found = Some(tile.clone());
                        break;
                    } else if cmp_right == bottom_rev {
                        tile.borrow_mut().rotate_right();
                        tile.borrow_mut().flip_vertical();
                        found = Some(tile.clone());
                        break;
                    }
                }
            }

            if let Some(tile) = found {
                tile.borrow_mut().top = Some(curr.clone());
                curr.borrow_mut().bottom = Some(tile.clone());
                matched.push(tile);
            }
        }

        // Left
        if curr.borrow().left.is_none() {
            let left = curr.borrow().left_border();
            let left_rev: String = left.chars().rev().collect();

            let mut found = None;
            for tile in &self.tiles {
                if curr.borrow().id != tile.borrow().id {
                    let cmp_top = tile.borrow().top_border();
                    if cmp_top == left {
                        tile.borrow_mut().rotate_right();
                        found = Some(tile.clone());
                        break;
                    } else if cmp_top == left_rev {
                        tile.borrow_mut().rotate_right();
                        tile.borrow_mut().flip_vertical();
                        found = Some(tile.clone());
                        break;
                    }

                    let cmp_bottom = tile.borrow().bottom_border();
                    if cmp_bottom == left {
                        tile.borrow_mut().rotate_left();
                        tile.borrow_mut().flip_vertical();
                        found = Some(tile.clone());
                        break;
                    } else if cmp_bottom == left_rev {
                        tile.borrow_mut().rotate_left();
                        found = Some(tile.clone());
                        break;
                    }

                    let cmp_left = tile.borrow().left_border();
                    if cmp_left == left {
                        tile.borrow_mut().flip_horizontal();
                        found = Some(tile.clone());
                        break;
                    } else if cmp_left == left_rev {
                        tile.borrow_mut().rotate_180();
                        found = Some(tile.clone());
                        break;
                    }

                    let cmp_right = tile.borrow().right_border();
                    if cmp_right == left {
                        found = Some(tile.clone());
                        break;
                    } else if cmp_right == left_rev {
                        tile.borrow_mut().flip_vertical();
                        found = Some(tile.clone());
                        break;
                    }
                }
            }

            if let Some(tile) = found {
                tile.borrow_mut().right = Some(curr.clone());
                curr.borrow_mut().left = Some(tile.clone());
                matched.push(tile);
            }
        }

        // Right
        if curr.borrow().right.is_none() {
            let right = curr.borrow().right_border();
            let right_rev: String = right.chars().rev().collect();

            let mut found = None;
            for tile in &self.tiles {
                if curr.borrow().id != tile.borrow().id {
                    let cmp_top = tile.borrow().top_border();
                    if cmp_top == right {
                        tile.borrow_mut().rotate_left();
                        tile.borrow_mut().flip_vertical();
                        found = Some(tile.clone());
                        break;
                    } else if cmp_top == right_rev {
                        tile.borrow_mut().rotate_left();
                        found = Some(tile.clone());
                        break;
                    }

                    let cmp_bottom = tile.borrow().bottom_border();
                    if cmp_bottom == right {
                        tile.borrow_mut().rotate_right();
                        found = Some(tile.clone());
                        break;
                    } else if cmp_bottom == right_rev {
                        tile.borrow_mut().rotate_right();
                        tile.borrow_mut().flip_vertical();
                        found = Some(tile.clone());
                        break;
                    }

                    let cmp_left = tile.borrow().left_border();
                    if cmp_left == right {
                        found = Some(tile.clone());
                        break;
                    } else if cmp_left == right_rev {
                        tile.borrow_mut().flip_vertical();
                        found = Some(tile.clone());
                        break;
                    }

                    let cmp_right = tile.borrow().right_border();
                    if cmp_right == right {
                        tile.borrow_mut().flip_horizontal();
                        found = Some(tile.clone());
                        break;
                    } else if cmp_right == right_rev {
                        tile.borrow_mut().rotate_180();
                        found = Some(tile.clone());
                        break;
                    }
                }
            }

            if let Some(tile) = found {
                tile.borrow_mut().left = Some(curr.clone());
                curr.borrow_mut().right = Some(tile.clone());
                matched.push(tile);
            }
        }

        for tile in matched {
            self.orient_inner(tile);
        }
    }
}

impl<'a> From<&'a str> for Graph {
    fn from(input: &'a str) -> Self {
        let tiles = input
            .split("\n\n")
            .map(|tile| Rc::new(RefCell::new(tile.into())))
            .collect();

        Graph { tiles }
    }
}
