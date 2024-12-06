use std::{thread::sleep, vec};

use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]

enum Cell {
    Empty,
    Nest,
    Food,
    Ant,
}
fn main() {
    let size: u32 = 13;
    let mut test: Vec<Vec<Cell>> = create_grid(size);
    test = modify_cell(12, 12, Cell::Nest, test);
    test = put_random_food(test, size);
    display_grid(&test);

}

fn create_grid(size: u32) -> Vec<Vec<Cell>> {
    let grille = vec![vec![Cell::Empty; size as usize]; size as usize];
    grille
}

fn get_cell(grid: &Vec<Vec<Cell>>, x: u32, y: u32)->Cell{
    grid[y as usize][x as usize]
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

fn put_random_food(mut grid: Vec<Vec<Cell>>, size: u32)->Vec<Vec<Cell>>{
    let mut i: u16 = 0;
    loop{
        
        let x = rand::thread_rng().gen_range(0..size);
        let y = rand::thread_rng().gen_range(0..size);
        if get_cell(&grid, x, y) == Cell::Empty{
            grid[y as usize][x as usize] = Cell::Food;
            return grid;
        }
        i+=1;
        if i as u32>= size*size{
            return grid;
        }
    }
}