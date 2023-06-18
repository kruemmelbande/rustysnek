use rand::Rng;
use std::io::Write;
use std::io::{stdout};
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

const BOARD_SIZE:[usize;2] = [10,10];
fn main() -> Result<(), Box<dyn std::error::Error>>  {
    //Snake in rust in the command line

    //Config:

    let start_size:i32= 3;
    let wall_char:&str = "##";
    let snake_char:&str = "[]";
    let food_char:&str = "()";
    let empty_char:&str = "  ";
    let time_between_frames:u64 = 200; //in ms
    let wall_collision_enabled:bool = true; //If false, the snake will wrap around if it hits a wall
    

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
    
    let mut board:[[i32;BOARD_SIZE[0]];BOARD_SIZE[1]] = [[0;BOARD_SIZE[0]];BOARD_SIZE[1]];
    draw_board(&board, &wall_char, &snake_char, &food_char, &empty_char);
    let mut snake_pos:[i32;2] = [(board.len()/2) as i32, (board[0].len()/2) as i32];
    let mut snake_dir:[i32;2] = [0,1];
    let mut snake_len:i32 = start_size;
    let mut food_pos:[usize;2];
    let mut stdout = stdout();
    let mut rng = rand::thread_rng();
    let mut score:usize = 0;
    food_pos = [rng.gen_range(0..board.len()), rng.gen_range(0..board[0].len())];
    board[food_pos[0]][food_pos[1]] = -1;
    clear_screen();
    loop{
        //Get input
        //let stdin = io::stdin();
        //clear events
        /*while event::poll(std::time::Duration::from_millis(1))? {
            if let Event::Key(_) = event::read()? {
            }
        }*/
        draw_board(&board, &wall_char, &snake_char, &food_char, &empty_char);
        print!("Score: {}", score);
        stdout.flush()?;
        enable_raw_mode().ok();
        //wait
        std::thread::sleep(std::time::Duration::from_millis(time_between_frames));
        if event::poll(std::time::Duration::from_millis(1))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') | KeyCode::Esc  => {
                        disable_raw_mode().ok();
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
        disable_raw_mode().ok();
        //Move snake
        snake_pos[0] = (snake_pos[0] as i32 + snake_dir[0]) as i32;
        snake_pos[1] = (snake_pos[1] as i32 + snake_dir[1]) as i32;
        //Check for collisions
        if wall_collision_enabled{
            if snake_pos[0] >= board.len() as i32 || snake_pos[1] >= board[0].len() as i32 || snake_pos[0] < 0 || snake_pos[1] < 0{
                death_screen(score);
            }
        }else{
            if snake_pos[0] >= board.len() as i32{
                snake_pos[0] = 0;
            }
            if snake_pos[1] >= board[0].len() as i32{
                snake_pos[1] = 0;
            }
            if snake_pos[0] < 0{
                snake_pos[0] = (board.len()-1) as i32;
            }
            if snake_pos[1] < 0{
                snake_pos[1] = (board[0].len()-1) as i32;
            }
        }
        if board[snake_pos[0] as usize][snake_pos[1] as usize] > 0{
            death_screen(score);
        }

       
        
        //Check for food
        if snake_pos[0] as i32 == food_pos[0] as i32 && snake_pos[1] as i32 == food_pos[1] as i32{
            snake_len += 1;
            score += 1;
            food_pos = [rng.gen_range(0..board.len()), rng.gen_range(0..board[0].len())];
            let mut i:usize = 0;
            let mut is_win:bool = false;
            while board[food_pos[0]][food_pos[1]] != 0{
                if i > 10000{
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                    println!();
                    println!("Youre win!");
                    is_win = true;
                    break;
                }
            
                i+=1;
                food_pos = [rng.gen_range(0..board.len()), rng.gen_range(0..board[0].len())];
            }
            if is_win{
                break;
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
        board[snake_pos[0] as usize][snake_pos[1] as usize] = snake_len;

    }Ok(())

}
fn draw_board(board: &[[i32;BOARD_SIZE[0]];BOARD_SIZE[1]], wall_char: &str, snake_char: &str, food_char: &str, empty_char: &str){
    //Draws the board to the console
    

    //Draw the top wall
    cursor_to_top();
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

fn cursor_to_top(){
    print!("\x1B[1;1H");
}

fn death_screen(score:usize){
    std::thread::sleep(std::time::Duration::from_millis(1000));
    clear_screen();
    println!("You died!");
    println!("Final score: {}", score);
    std::process::exit(0);
}