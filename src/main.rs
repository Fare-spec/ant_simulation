#[derive(Debug, Clone, Copy, PartialEq)]

enum Cell {
    Empty,
    Nest,
    Food,
    Ant,
}
fn main() {
    let test = create_grid(13);
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
//fn modify_cell(x: u32, y: u32,)
