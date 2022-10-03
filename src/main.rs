use std::io;
use rand::Rng;
use std::thread;
use std::sync::mpsc;


fn main()
{
    let mut bugs :(i8,i8,char) = (0,0,'B');
    let mut taz:(i8,i8,char) = (1,0,'D');
    let mut tweety:(i8,i8,char) = (1,2,'T');
    let mut marvin:(i8,i8,char) = (0,0,'M');
    game_board(bugs, taz, tweety, marvin, 10);
}

fn game_board(mut bugs:(i8,i8,char),mut taz:(i8,i8,char), mut tweety:(i8,i8,char),mut marvin:(i8,i8,char),mut counter:i32) -> bool{
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

    let mut eliminated:char = '-';
    for received in rx{
        println!("Received: {}",received.2);
        if grid[received.0 as usize][received.1 as usize] == '-'
        {
            grid[received.0 as usize][received.1 as usize] = received.2;
        }
        else if received.2 == 'M'
        {
            eliminated = grid[received.0 as usize][received.1 as usize];
            grid[received.0 as usize][received.1 as usize] = 'M';
        }
        else
        {
            println!("collision at {} {}",received.0,received.1);
        }
    }

    if eliminated != '-'
    {
        println!("elimnated: {}", eliminated);
        match eliminated {
            'B' => bugs.2='-',
            'D' => taz.2='-',
            'T' => tweety.2='-',
            _ => println!("Who got shot?"),
        }
    }

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
