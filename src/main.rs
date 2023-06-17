const BOARD_SIZE:[usize;2] = [10,10];
use rand::Rng;
use std::io::Write;
use std::io::{self, stdout};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};


fn main() -> Result<(), Box<dyn std::error::Error>>  {
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
    enable_raw_mode();
    let mut is_alive:bool=true;
    let mut board:[[i32;BOARD_SIZE[0]];BOARD_SIZE[1]] = [[0;BOARD_SIZE[0]];BOARD_SIZE[1]];
    draw_board(&board, &wall_char, &snake_char, &food_char, &empty_char);
    let mut snake_pos:[usize;2] = [board.len()/2 as usize, board[0].len()/2 as usize];
    let mut snake_dir:[i32;2] = [0,1];
    let mut snake_len:i32 = start_size;
    let mut food_pos:[usize;2] = [0,0];
    let mut stdout = stdout();
    let mut rng = rand::thread_rng();
    let mut score:i32 = 0;

    let mut input = String::new();
    food_pos = [rng.gen_range(0..board.len()), rng.gen_range(0..board[0].len())];
    board[food_pos[0]][food_pos[1]] = -1;
    while is_alive{
        //Get input
        let stdin = io::stdin();
        
        draw_board(&board, &wall_char, &snake_char, &food_char, &empty_char);
        stdout.flush()?;
        enable_raw_mode();
        //wait
        std::thread::sleep(std::time::Duration::from_millis(200));
        if event::poll(std::time::Duration::from_millis(1))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') => {
                        is_alive = false;
                        disable_raw_mode();
                        break;
                    },
                    KeyCode::Left => {
                        snake_dir = [0,-1];
                    },
                    KeyCode::Right => {
                        snake_dir = [0,1];
                    },
                    KeyCode::Up => {
                        snake_dir = [-1,0];
                    },
                    KeyCode::Down => {
                        snake_dir = [1,0];
                    },
                    _ => {}
                }
            }
        }
        disable_raw_mode();
        //Move snake
        snake_pos[0] = (snake_pos[0] as i32 + snake_dir[0]) as usize;
        snake_pos[1] = (snake_pos[1] as i32 + snake_dir[1]) as usize;
        //Check for collisions
        if board[snake_pos[0]][snake_pos[1]] > 0{
            is_alive = false;
        }
        if wall_collision_enabled{
            if snake_pos[0] >= board.len() || snake_pos[1] >= board[0].len(){
                is_alive = false;
                break;
            }
        }
        else{
            if snake_pos[0] >= board.len(){
                snake_pos[0] = 0;
            }
            if snake_pos[1] >= board[0].len(){
                snake_pos[1] = 0;
            }
            if snake_pos[0] < 0{
                snake_pos[0] = board.len()-1;
            }
            if snake_pos[1] < 0{
                snake_pos[1] = board[0].len()-1;
            }
        }
        
        //Check for food
        if snake_pos==food_pos{
            snake_len += 1;
            score += 1;
            food_pos = [rng.gen_range(0..board.len()), rng.gen_range(0..board[0].len())];
            let mut i = 0;
            while board[food_pos[0]][food_pos[1]] != 0{
                if i > 10000{
                    is_alive = false;
                    println!("Youre win!");
                    break;
                }
                food_pos = [rng.gen_range(0..board.len()), rng.gen_range(0..board[0].len())];
            }
            board[food_pos[0]][food_pos[1]] = -1;
        }
        //Update board
        for row in board.iter_mut(){
            for block in row.iter_mut(){
                if *block > 0{
                    *block -= 1;
                }
            }
        }
        board[snake_pos[0]][snake_pos[1]] = snake_len;
        score +=1; 
    }Ok(())

}
fn draw_board(board: &[[i32;BOARD_SIZE[0]];BOARD_SIZE[1]], wall_char: &str, snake_char: &str, food_char: &str, empty_char: &str){
    //Draws the board to the console
    

    //Draw the top wall
    clear_screen();
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
    println!("{}", wall_char.repeat(board[0].len()+2));
}

fn clear_screen(){
    print!("\x1B[2J\x1B[1;1H");
}