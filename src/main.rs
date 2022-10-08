use rand::Rng;
use std::thread;
use std::sync::mpsc;


fn main()
{
    //Tuple value descriptions (current_x,current_y,name,previous_x,previous_y,has flag?)
    let bugs :(i8,i8,char,i8,i8,bool) = (0,0,'B',0,0,false);
    let taz:(i8,i8,char,i8,i8,bool) = (1,0,'D',1,0,false);
    let tweety:(i8,i8,char,i8,i8,bool) = (1,0,'T',1,0,false);
    let marvin:(i8,i8,char,i8,i8,bool) = (4,4,'M',4,4,false);
    let mountain:(i8,i8,char,i8,i8,bool) = (1,3,'F',4,4,false);
    const M: usize = 5;
    const N: usize = 5;
    let mut grid = [['-' as char; N]; M];
    grid[2][2] = 'C';
    grid[3][1] = 'C';
    grid[mountain.0 as usize][mountain.1 as usize] = mountain.2;
    game_board(bugs, taz, tweety, marvin,mountain, grid,0);
}

fn game_board(mut bugs:(i8,i8,char,i8,i8,bool),mut taz:(i8,i8,char,i8,i8,bool), mut tweety:(i8,i8,char,i8,i8,bool),mut marvin:(i8,i8,char,i8,i8,bool), mut mountain:(i8,i8,char,i8,i8,bool),mut grid:[[char;5];5],mut counter:i32) -> bool{
    counter = counter + 1;
    //Space X Multi-Dimensional Time Travel Machine
    let mut SXMDTTM = false;
    //Every 3 turns
    if counter%3 == 0 {SXMDTTM = true}
    let mut end = false;
    //if counter > 20 { end = true;} //used for testing to keep board reprints low


    //creating channels so that we can communicate outside the spawned thread to the main thread
    let (bugs_tx, rx) = mpsc::channel();
    let taz_tx = bugs_tx.clone();
    let tweety_tx = bugs_tx.clone();
    let marvin_tx = bugs_tx.clone();

    //if it's time to move the mountain
    if SXMDTTM
    {
        let mut rng = rand::thread_rng();
        let mut x = rng.gen_range(0, 4);
        let mut y = rng.gen_range(0, 4);
        //caching current coords to set to empty after mountain moves
        mountain.3 = mountain.0;
        mountain.4 = mountain.1;
        mountain.0 = x;
        mountain.1 = y;
        while grid[mountain.0 as usize][mountain.1 as usize] != 'F'
        {
            if grid[mountain.0 as usize][mountain.1 as usize] == '-'
            {
                grid[mountain.0 as usize][mountain.1 as usize] = 'F';
                grid[mountain.3 as usize][mountain.4 as usize] = '-';
            }
            else
            {
                x = rng.gen_range(0, 4);
                y = rng.gen_range(0, 4);
                mountain.0 = x;
                mountain.1 = y;
            }
        }
    }

    let bugs_handle = thread::spawn(move || {
        bugs = character_move(bugs);
        bugs_tx.send(bugs).unwrap();
    });
    let taz_handle = thread::spawn(move || {
        taz = character_move(taz);
        taz_tx.send(taz).unwrap();
    });
    let tweety_handle = thread::spawn(move || {
        tweety = character_move(tweety);
        tweety_tx.send(tweety).unwrap();
    });
    let marvin_handle = thread::spawn(move || {
        marvin = character_move(marvin);
        marvin_tx.send(marvin).unwrap();
    });

    let mut eliminated:char = '-';
    //receiving information from threads when they finish executing
    for mut received in rx{
        println!("Received: {}",received.2);
        if grid[received.0 as usize][received.1 as usize] == '-' //if character tries to move to empty game board spot
        {
            grid[received.0 as usize][received.1 as usize] = received.2; //change game board spot to character marker
            grid[received.3 as usize][received.4 as usize] = '-'; //previous position is empty
        }
        else if grid[received.0 as usize][received.1 as usize] == 'C' && received.2 != '-' //character steps on flag square and is not eliminated
        {
            grid[received.0 as usize][received.1 as usize] = received.2;
            grid[received.3 as usize][received.4 as usize] = '-';
            received.5 = true;
            println!("{} got the flag!",received.2);
        }
        else if grid[received.0 as usize][received.1 as usize] == 'F' && received.5 == true
        {
            println!("{} won!",received.2);
            end = true;
        }
        else if received.2 == 'M' //if the character is Marvin and is moving on to a space that is not blank
        {
            if grid[received.0 as usize][received.1 as usize] != 'F'
            {
                eliminated = grid[received.0 as usize][received.1 as usize]; //Character which was eliminated is marked
                grid[received.0 as usize][received.1 as usize] = 'M'; // Marvin takes position
                grid[received.3 as usize][received.4 as usize] = '-';
            }
        }
        else // characters collide
        {
            println!("collision at {} {}",received.0,received.1);
            grid[received.3 as usize][received.4 as usize] = received.2; //character does not move from previous position
            received.0 = received.3;
            received.1 = received.4;
        }
        match received.2 {
            'B' => bugs = received,
            'D' => taz = received,
            'T' => tweety = received,
            'M' => marvin = received,
            '-' => println!("eliminated character"),
            _ => println!("error in assigning tuple received to character"),
        }
    }

    if eliminated != '-' //if a character was eliminated this round
    {
        println!("eliminated: {}", eliminated);
        match eliminated { //changing character marker to blank game board spot so they are essentially invisible
            'B' =>
                {
                    bugs.2='-';
                    if bugs.5 == true {marvin.5 = true; println!("Marvin now has the flag!")}
                },
            'D' =>
                {
                    taz.2='-';
                    if taz.5 == true {marvin.5 = true; println!("Marvin now has the flag!")}
                },
            'T' =>
                {
                    tweety.2='-';
                    if tweety.5 == true {marvin.5 = true; println!("Marvin now has the flag!")}
                },
            _ => println!("Who got shot?"),
        }
    }

    //wait for all threads to finish
    bugs_handle.join().unwrap();
    taz_handle.join().unwrap();
    tweety_handle.join().unwrap();
    marvin_handle.join().unwrap();



    //printing game board
    for (_i, row) in grid.iter().enumerate() {
        for (_j, col) in row.iter().enumerate() {
            print!("{}  ", col);
        }
        println!()
    }
    println!();

    if !end {
        game_board(bugs, taz, tweety, marvin,mountain,grid,counter);
    }
    true

}


fn character_move(mut character:(i8,i8,char,i8,i8,bool)) -> (i8,i8,char,i8,i8,bool) {
    let mut rng = rand::thread_rng();
    let mut row = rng.gen_range(0, 9);
    let mut column = rng.gen_range(0, 10);
    // setting current position for later reference
    character.3 = character.0;
    character.4 = character.1;
    //arbitrary -- just a way to randomize movement
    if row > column
    {
        //arbitrary -- just a way to randomize movement
        if row > 4 { row = 1;}
        else { row = -1;}
        if character.1 + row < 0 || character.1 + row > 4
        {
            character.1 = character.1 - row;
        }
        else
        {
            character.1 = character.1 + row;
        }
    }
    else
    {
        if column > 4 { column = 1;}
        else { column = -1;}
        if character.0 + column < 0 || character.0 + column > 4
        {
            character.0 = character.0 - column;
        }
        else
        {
            character.0 = character.0 + column;
        }
    }
    character
}
