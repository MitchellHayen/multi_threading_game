use rand::Rng;
use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};


fn main()
{
    let mut character_array = vec![];
    //Tuple value descriptions (current_x,current_y,name,previous_x,previous_y,has flag?)
    let bugs :(i8,i8,char,i8,i8,bool) = (3,2,'B',0,0,false);
    let taz:(i8,i8,char,i8,i8,bool) = (2,0,'D',0,0,false);
    let tweety:(i8,i8,char,i8,i8,bool) = (1,0,'T',0,0,false);
    let marvin:(i8,i8,char,i8,i8,bool) = (2,2,'M',0,0,false);
    let mut mountain:(i8,i8,char,i8,i8,bool) = (1,3,'F',0,0,false);
    character_array.push(bugs);
    character_array.push(taz);
    character_array.push(tweety);
    character_array.push(marvin);
    const M: usize = 5;
    const N: usize = 5;
    let eliminated = Arc::new(Mutex::new('0'));
    let end = Arc::new(Mutex::new(false));
    let grid = Arc::new(Mutex::new([['-' as char; N]; M]));
    {
        //entering new scope so that the mutex locks after leaving this scope
        //cloning reference to mutex
        let temp_grid = Arc::clone(&grid);
        //unlocking mutex
        let mut grid = temp_grid.lock().unwrap();
        grid[3][3] = 'C';
        grid[3][1] = 'C';
        grid[mountain.0 as usize][mountain.1 as usize] = mountain.2;
    }
    //mutex is not locked again
    let mut counter = 3;
//main game loop
    loop
    {
        println!("Round {}",counter-3);
        counter = counter + 1;
        //Space X Multi-Dimensional Time Travel Machine
        let mut SXMDTTM = false;
        //Every 3 turns
        if counter%3 == 0 {SXMDTTM = true}

        //if it's time to move the mountain
        if SXMDTTM
        {
            let temp_grid = Arc::clone(&grid);
            let mut rng = rand::thread_rng();
            let mut x = rng.gen_range(0, 4);
            let mut y = rng.gen_range(0, 4);
            //caching current coords to set to empty after mountain moves
            mountain.3 = mountain.0;
            mountain.4 = mountain.1;
            mountain.0 = x;
            mountain.1 = y;
            //mountain position is the same as previous position
            if mountain.3 == x && mountain.4 == y
            {
                let mut grid = temp_grid.lock().unwrap();
                grid[mountain.0 as usize][mountain.1 as usize] = 'F';
                println!("New mountain position at {} {}",mountain.0,mountain.1);
            }
            else {
                let mut grid = temp_grid.lock().unwrap();
                //while current position is not mountain
                while grid[mountain.0 as usize][mountain.1 as usize] != 'F'
                {
                    //if new position is blank, then move mountain and change old position to blank '-'
                    if grid[mountain.0 as usize][mountain.1 as usize] == '-'
                    {
                        grid[mountain.3 as usize][mountain.4 as usize] = '-';
                        grid[mountain.0 as usize][mountain.1 as usize] = 'F';
                        println!("New mountain position at {} {}",mountain.0,mountain.1);
                    } else {
                        //otherwise generate new position
                        x = rng.gen_range(0, 4);
                        y = rng.gen_range(0, 4);
                        mountain.0 = x;
                        mountain.1 = y;
                    }
                }
            }
        }

        //need channels in order to send information out of threads to main thread
        let (tx,rx) = mpsc::channel();
        let mut handles = vec![];
        let mut temp_character_array = vec![];
        for mut character in character_array.clone()
        {
            let prev_x = character.0 as usize;
            let prev_y = character.1 as usize;
            let mut temp_character = character;
            let temp_grid = Arc::clone(&grid);
            let temp_elim = Arc::clone(&eliminated);
            let temp_end = Arc::clone(&end);
            //check if eliminated - break if true
            if *temp_elim.lock().unwrap() == character.2 {continue;}
            else {
                //need to clone sending channel for each new thread created
                let tx_clone = tx.clone();
                //create new thread
                let handle = thread::spawn(move ||
                    {
                        loop
                        {
                            let mut grid = temp_grid.lock().unwrap();
                            temp_character = character_move(character);
                            if grid[temp_character.0 as usize][temp_character.1 as usize] == '-' //if character tries to move to empty game board spot
                            {
                                grid[prev_x][prev_y] = '-'; //previous position is empty
                                grid[temp_character.0 as usize][temp_character.1 as usize] = character.2; //change game board spot to character marker
                                break;
                            } else if grid[temp_character.0 as usize][temp_character.1 as usize] == 'C' && character.2 != '-' //character steps on flag square and is not eliminated
                            {
                                grid[prev_x][prev_y] = '-';
                                grid[temp_character.0 as usize][temp_character.1 as usize] = character.2;
                                temp_character.5 = true;
                                println!("{} got the flag!", character.2);
                                break;
                            } else if grid[temp_character.0 as usize][temp_character.1 as usize] == 'F' && character.5 == true
                            {
                                println!("{} moved to {},{} and won!", character.2, temp_character.0,temp_character.1);
                                grid[prev_x][prev_y] = '-';
                                grid[temp_character.0 as usize][temp_character.1 as usize] = 'V';
                                let mut end = temp_end.lock().unwrap();
                                *end = true;
                                break;
                            } else if character.2 == 'M' //if the character is Marvin and is moving on to a space that is not blank
                            {
                                if grid[temp_character.0 as usize][temp_character.1 as usize] != 'F'
                                {
                                    let mut eliminated = temp_elim.lock().unwrap();
                                    *eliminated = grid[temp_character.0 as usize][temp_character.1 as usize]; //Character which was eliminated is marked
                                    println!("Marvin eliminated {}!", eliminated);
                                    grid[prev_x][prev_y] = '-';
                                    grid[temp_character.0 as usize][temp_character.1 as usize] = 'M'; // Marvin takes position
                                    break;
                                }
                            }
                            // no else statement, this loop should repeat until character finds a space to move to
                        }
                        //TODO have infinite loop in movement when a character is trapped
                        character = temp_character;
                        //send character outside of this thread to main thread (or wherever rx calls it)
                        tx_clone.send(character).unwrap();
                        if *temp_end.lock().unwrap() == false
                        {
                            //print game board
                            let grid = temp_grid.lock().unwrap();
                            for (_i, row) in grid.iter().enumerate()
                            {
                                for (_j, col) in row.iter().enumerate()
                                {
                                    print!("{}  ", col);
                                }
                                println!()
                            }
                            println!();
                        }
                    });
                //add handle to handles list to be joined later
                handles.push(handle);
            }
        }
        for handle in handles
        {
            //wait for all threads to finish before moving on
            handle.join().unwrap();
        }
        //tx was created in order to clone, but never used.  Need to drop for program to continue
        drop(tx);
        //receiving characters from each thread
        for received in rx
        {
            temp_character_array.push(received);
        }
        let temp_end = Arc::clone(&end);
        let end = temp_end.lock().unwrap();
        if *end {break}
        if (counter - 3) > 1000
        {
            println!("Greater than 1000 rounds have been played.  Let's call it a draw!");
            break;
        }
        character_array = temp_character_array;
    }
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
