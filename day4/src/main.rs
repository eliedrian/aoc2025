use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Missing path");
    }

    let file_path: &str = &args[1];
    let file_content: String = fs::read_to_string(file_path).expect("Unable to read file.");
    solution1(&file_content);
    solution2(&file_content);
}

#[derive(Debug,Clone)]
struct Grid {
    data: Vec<Vec<char>>,
}

impl Grid {
    fn construct(input: &str) -> Grid {
        let lines = input.lines();
        let mut rows: Vec<Vec<char>> = vec![];

        for line in lines {
            let row_chars: Vec<char> = line.chars().collect();
            rows.push(row_chars);
        };

        Grid { data: rows }
    }

    fn index(&self, i: usize, j: usize) -> char {
        self.data[i][j]
    }

    fn row_len(&self) -> usize {
        self.data.len()
    }

    fn col_len(&self) -> usize {
        self.data[0].len()
    }

    fn neighbors(&self, i: &usize, j: &usize) -> Vec<char> {
        let ii: isize = (*i).try_into().expect("Unable to convert");
        let jj: isize = (*j).try_into().expect("Unable to convert");
        let neighbor_indices: [(isize, isize); 8] = [
            (ii - 1, jj - 1), (ii - 1, jj), (ii - 1, jj +1),
            (ii, jj - 1), (ii, jj + 1),
            (ii + 1, jj - 1), (ii + 1, jj), (ii + 1, jj + 1),
        ];

        neighbor_indices.iter().map(|(r, c)| {
            let row_length: isize = self.row_len().try_into().expect("Unable to convert");
            let col_length: isize = self.col_len().try_into().expect("Unable to convert");
            if (0..row_length).contains(r) && (0..col_length).contains(c) {
                let r: usize = (*r).try_into().expect("Unable to convert");
                let c: usize = (*c).try_into().expect("Unable to convert");
                self.index(r, c)
            } else {
                ' '
            }
        }).filter(|x| *x != ' ')
        .collect()
    }

    fn remove(&mut self, (i, j): (usize, usize)) {
        self.data[i][j] = '.';
    }
}

fn solution1(input: &str) {
    let g = Grid::construct(input);

    let row_range = 0..g.row_len();
    let col_range = 0..g.col_len();
    let positions = row_range.flat_map(|i| {
        col_range.clone().map(move |j| {
            (i, j)
        })
    });
    let neighbors = positions
        .filter(|(i, j)| g.index(*i, *j) == '@')
        .map(|(i, j)| g.neighbors(&i, &j));

    let neighbor_count = neighbors.map(|n| n.iter().filter(|&c| *c == '@').count());
    let counts = neighbor_count.filter(|c| *c < 4_usize);
    println!("Solution 1 count: {}", counts.count());
}

fn count_neighbors((pos, n): ((usize, usize), Vec<char>)) -> ((usize, usize), usize) {
    (pos, n.iter().filter(|&c| *c == '@').count())
}

fn is_free_roll((_, count): ((usize, usize), &usize)) -> bool {
    *count < 4_usize
}

fn solution2(input: &str) {
    let mut g = Grid::construct(input);

    let row_range = 0..g.row_len();
    let col_range = 0..g.col_len();
    let positions = row_range.flat_map(|i| {
        col_range.clone().map(move |j| { (i, j) })
    });

    let mut free_rolls: Vec<((usize, usize), usize)> = positions.clone()
        .filter(|(i, j)| g.index(*i, *j) == '@')
        .map(|(i, j)| ((i, j), g.neighbors(&i, &j)))
        .map(count_neighbors)
        .filter(|(_, count)| *count < 4_usize)
        .collect();

    let mut total = 0;

    while !free_rolls.is_empty() {
        total += free_rolls.len();
        let positions_to_remove = free_rolls.iter().map(|(pos, _)| pos);

        for pos in positions_to_remove {
            g.remove(*pos);
        }

        free_rolls = positions.clone()
            .filter(|(i, j)| g.index(*i, *j) == '@')
            .map(|(i, j)| ((i, j), g.neighbors(&i, &j)))
            .map(count_neighbors)
            .filter(|(_, count)| *count < 4_usize)
            .collect();
    }

    println!("Total after removes: {total}");

}
