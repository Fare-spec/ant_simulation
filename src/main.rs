use std::vec;

#[derive(Debug, Clone, Copy, PartialEq)]

enum Cell {
    Empty,
    Nest,
    Food,
    Ant,
}
fn main() {
    let mut test: Vec<Vec<Cell>> = create_grid(13);
    test = modify_cell(1, 1, Cell::Nest, test);
    display_grid(&test);

}

fn create_grid(size: usize) -> Vec<Vec<Cell>> {
    let grille = vec![vec![Cell::Empty; size]; size];
    grille
}
fn display_grid(grid: &Vec<Vec<Cell>>) {
    for row in grid {
        for celle in row {
            let symbol = match celle {
                Cell::Empty => '.',
                Cell::Ant => 'A',
                Cell::Nest => 'N',
                Cell::Food => 'F',
            };
            print!("{}", symbol);
        }
        println!();
    }
}
fn modify_cell(x: u32, y: u32,value_cell: Cell, mut grid: Vec<Vec<Cell>> )->Vec<Vec<Cell>>{
    grid[y as usize][x as usize] = value_cell;
    grid
}
