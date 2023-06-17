const BOARD_SIZE:[usize;2] = [10,10];
use std::io;
use rand::Rng;
fn main() {
    //Snake in rust in the command line

    //Config:

    let start_size= 3;
    let wall_char = "#";
    let snake_char = "O";
    let food_char = "X";
    let empty_char = " ";
    let wall_collision_enabled = true; //If false, the snake will wrap around if it hits a wall


    /*so here is my idea.
    I represent the board as a 2d array of integers.
    0 means, the block is empty.
    The snake isnt a big thing, but rather a single block that moves around.
    Wherever the snake is, the block is set to the value of the length of the snake.
    If now, we decrement every block that isnt zero, we can simly check if a block is larger than zero, and if so, we know that the snake is there.
    This way, we can easily move the snake, and check for collisions, without having to remember the whole snake.
    The food can also be represented with -1.

    The drawing function can then easily turn that into a string, and print it.
    */
    let mut is_alive:bool=true;
    let mut board:[[i32;BOARD_SIZE[0]];BOARD_SIZE[1]] = [[0;BOARD_SIZE[0]];BOARD_SIZE[1]];
    draw_board(&board, &wall_char, &snake_char, &food_char, &empty_char);
    let mut snake_pos:[usize;2] = [(int)(BOARD_SIZE[0]/2),(int)(BOARD_SIZE[1]/2)];
    let mut snake_dir:[i32;2] = [0,1];
    let mut snake_len:i32 = start_size;
    let mut food_pos:[usize;2] = [0,0];
    let mut rng = rand::thread_rng();

}
fn draw_board(board: &[[i32;BOARD_SIZE[0]];BOARD_SIZE[1]], wall_char: &str, snake_char: &str, food_char: &str, empty_char: &str){
    //Draws the board to the console
    

    //Draw the top wall
    println!("{}", wall_char.repeat(board[0].len()+2));
    for row in board.iter(){
        print!("{}", wall_char);
        for block in row.iter(){
            match block{
                0 => print!("{}", empty_char),
                -1 => print!("{}", food_char),
                _ => print!("{}", snake_char),
            }
        }
        print!("{}", wall_char);
        println!();
    }
    print!("{}", wall_char.repeat(board[0].len()+2));
}