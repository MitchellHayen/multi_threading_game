use std::io;
use rand::Rng;
use std::thread;


fn main()
{
    let bugs = [0,0];
    let taz = [4,0];
    let tweety = [0,4];
    let marvin = [2,2];
    game_board(bugs, taz, tweety, marvin, 10);
}

fn game_board(bugs:[i8;2],taz:[i8;2], tweety:[i8;2], marvin:[i8;2],mut counter:i32) -> bool{
    counter = counter - 1;



    const M: usize = 5;
    const N: usize = 5;

    let mut grid = [['-' as char; N]; M];
    let bugs_handle = thread::spawn(move || {
        let bugs = character_move(bugs);
        let bugs_y: usize = bugs[0] as usize;
        let bugs_x: usize = bugs[1] as usize;
        if grid[bugs_y][bugs_x] == '-'
        {
            grid[bugs_y][bugs_x] = 'B';
        }
        //else statement -- need to choose new move
    });
    let taz_handle = thread::spawn(move || {
        let taz = character_move(taz);
        let taz_y: usize = taz[0] as usize;
        let taz_x: usize = taz[1] as usize;
        if grid[taz_y][taz_x] == '-'
        {
            grid[taz_y][taz_x] = 'D';
        }
        //else statement
    });
    let tweety_handle = thread::spawn(move || {
        let tweety = character_move(tweety);
        let tweety_y: usize = tweety[0] as usize;
        let tweety_x: usize = tweety[1] as usize;
        if grid[tweety_y][tweety_x] == '-'
        {
            grid[tweety_y][tweety_x] = 'T';
        }
        //else
    });
    let marvin_handle = thread::spawn(move || {
        let marvin = character_move(marvin);
        let marvin_y: usize = marvin[0] as usize;
        let marvin_x: usize = marvin[1] as usize;
        if grid[marvin_y][marvin_x] == '-'
        {
            grid[marvin_y][marvin_x] = 'M';
        }
        //else
        //println!("grid[marvin_y][marvin_x] is: {}",grid[marvin_y][marvin_x]);
    });

    bugs_handle.join().unwrap();
    taz_handle.join().unwrap();
    tweety_handle.join().unwrap();
    marvin_handle.join().unwrap();


    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            print!("{}  ", col);
        }
        println!()
    }
    println!();
    if counter > 0 {
        game_board(bugs, taz, tweety, marvin, counter);
    }
    true

}


fn game_round() {

}

fn character_move(mut character:[i8;2]) -> [i8;2] {
    let mut rng = rand::thread_rng();
    let mut vertical = rng.gen_range(0,9);
    let mut horizontal = rng.gen_range(0,10);
    if vertical > horizontal
    {
        if vertical > 4 {vertical = 1;}
        else { vertical = -1;}
        if character[0] + vertical < 0 || character[0] + vertical > 4
        {
            character[0] = character[0] - vertical;
        }
        else
        {
            character[0] = character[0] + vertical;
        }
    }
    else
    {
        if horizontal > 4 {horizontal = 1;}
        else { horizontal = -1;}
        if character[1] + horizontal < 0 || character[1] + horizontal > 4
        {
            character[1] = character[1] - horizontal;
        }
        else
        {
            character[1] = character[1] + horizontal;
        }
    }
    character
}
