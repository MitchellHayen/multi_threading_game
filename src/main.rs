use std::io;
use rand::Rng;
use std::thread;
use std::sync::mpsc;


fn main()
{
    let bugs :(i8,i8,char) = (0,0,'B');
    let taz:(i8,i8,char) = (1,0,'D');
    let tweety:(i8,i8,char) = (0,4,'T');
    let marvin:(i8,i8,char) = (2,2,'M');
    game_board(bugs, taz, tweety, marvin, 10);
}

fn game_board(bugs:(i8,i8,char),taz:(i8,i8,char), tweety:(i8,i8,char), marvin:(i8,i8,char),mut counter:i32) -> bool{
    counter = counter - 1;

    const M: usize = 5;
    const N: usize = 5;

    let (bugs_tx, rx) = mpsc::channel();
    let taz_tx = bugs_tx.clone();
    let tweety_tx = bugs_tx.clone();
    let marvin_tx = bugs_tx.clone();

    let mut grid = [['-' as char; N]; M];
    let bugs_handle = thread::spawn(move || {
        let bugs = character_move(bugs);
        bugs_tx.send(bugs).unwrap();
    });
    let taz_handle = thread::spawn(move || {
        let taz = character_move(taz);
        taz_tx.send(taz).unwrap();
    });
    let tweety_handle = thread::spawn(move || {
        let tweety = character_move(tweety);
        tweety_tx.send(tweety).unwrap();
    });
    let marvin_handle = thread::spawn(move || {
        let marvin = character_move(marvin);
        marvin_tx.send(marvin).unwrap();
    });


    for received in rx{
        println!("Received: {}",received.2);
    }


/*    let bugs = bugs_rx.recv().unwrap();
    let bugs_y:usize = bugs[0] as usize;
    let bugs_x:usize = bugs[1] as usize;

    let taz = taz_rx.recv().unwrap();
    let taz_y:usize = taz[0] as usize;
    let taz_x:usize = taz[1] as usize;

    let tweety = tweety_rx.recv().unwrap();
    let tweety_y:usize = tweety[0] as usize;
    let tweety_x:usize = tweety[1] as usize;

    let marvin = marvin_rx.recv().unwrap();
    let marvin_y:usize = marvin[0] as usize;
    let marvin_x:usize = marvin[1] as usize;

    if grid[bugs_y][bugs_x] == '-'
    {
        grid[bugs_y][bugs_x] = 'B';
    }
    else
    {
        println!("collision at {}",grid[bugs_y][bugs_x]);
    }
    if grid[taz_y][taz_x] == '-'
    {
        grid[taz_y][taz_x] = 'D';
    }
    else
    {
        println!("collision at {}",grid[taz_y][taz_x]);
    }
    if grid[tweety_y][tweety_x] == '-'
    {
        grid[tweety_y][tweety_x] = 'T';
    }
    else
    {
        println!("collision at {}",grid[tweety_y][tweety_x]);
    }
    if grid[marvin_y][marvin_x] == '-'
    {
        grid[marvin_y][marvin_x] = 'M';
    }
    else
    {
        println!("collision at {}",grid[marvin_y][marvin_x]);
    }*/
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

fn character_move(mut character:(i8,i8,char)) -> (i8,i8,char) {
    let mut rng = rand::thread_rng();
    let mut vertical = rng.gen_range(0,9);
    let mut horizontal = rng.gen_range(0,10);
    if vertical > horizontal
    {
        if vertical > 4 {vertical = 1;}
        else { vertical = -1;}
        if character.1 + vertical < 0 || character.1 + vertical > 4
        {
            character.1 = character.1 - vertical;
        }
        else
        {
            character.1 = character.1 + vertical;
        }
    }
    else
    {
        if horizontal > 4 {horizontal = 1;}
        else { horizontal = -1;}
        if character.0 + horizontal < 0 || character.0 + horizontal > 4
        {
            character.0 = character.0 - horizontal;
        }
        else
        {
            character.0 = character.0 + horizontal;
        }
    }
    character
}
