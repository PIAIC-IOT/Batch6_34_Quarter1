use rand::Rng;
use std::io;
use std::collections::HashMap;

fn main() {
    println!("Welcome to LUDO");    
    //in player variable we save total number of player
    let player = loop {
        println!("Enter Number of Players!");
        let mut player = String::new();
        io::stdin().read_line(&mut player).expect("Only Numbers Allowed");
        //logic to entery only numbers
        let player :u8 = match player.trim().parse() {
            Ok(p)   => p,
            Err(_)  => {
                println!("Only Numbers Allowed, Try Again");
                continue
            },
        };
        break player;
    };
    //in playerlist we will save name of players
    let mut playerlist : Vec<String> = Vec::new();
    //in turnscore we will save turn score with playername, turn number and turn score
    //in turnscore we also save to total of each player and will update on each turn,
    let mut turnscores = HashMap::new();    
    //Palyer Name Entry
    for turn in 1..=player {
        println!("Enter Name of Player {} :",turn);
        let mut player = String::new();
        io::stdin().read_line(&mut player).expect("Only Numbers Allowed");
        playerlist.push(player.trim().to_string());
        turnscores.entry(player.trim().to_string()).or_insert(0u8);
    }
    println!("{:?}",playerlist);
    println!("{:?}",turnscores);
    let mut turn = 1;
    loop {
        for myturn in 1..=player {
            let mut score :u8 = 0;
            let dice = rand::thread_rng().gen_range(1, 7);    
            //logic start to cehck 6 and then three sixer
            match dice {
                6 => {
                    score = score + dice;
                    let dice = rand::thread_rng().gen_range(1, 7);    
                    match dice {
                        6 => {
                            score = score + dice;
                            let dice = rand::thread_rng().gen_range(1, 7);
                            match dice {
                                6 => score = { 
                                    println!("\nAlas! Turn {} Dice Roll of Player {} - {} is 0 due to 3 Sixer and Total {}",turn,myturn,playerlist[(myturn-1)as usize],turnscores.get(&playerlist[(myturn-1)as usize]).unwrap()); 
                                    0
                                },
                                _ => score = score + dice
                            }   
            
                        },
                        _ => score = score + dice
                    }

                },
                _ => score = score + dice
            }
            //logic end to cehck 6 and then three sixer
            let myscore = match turnscores.get(&playerlist[(myturn-1)as usize]) {
                Some(data)  => data,
                None        => &0,
            };
            //Saving score of every turn with name of palyer turn number and turn score
            let playerturn = format!("{}{}",playerlist[(myturn-1)as usize],turn);
            //checking the total should remaing <=100 and 
            //Updating Total  score 
            if myscore + score <= 100 {
                turnscores.entry(playerturn).or_insert(score);
                let t = turnscores.entry(playerlist[(myturn-1)as usize].clone()).or_insert(0);
                * t += score;
            } else {
                turnscores.entry(playerturn).or_insert(0);
            }
         
            let myscore = match turnscores.get(&playerlist[(myturn-1)as usize]) {
                Some(data)  => data,
                None        => &0,
            };
            //Printing turn and total score of every player at every turn
            println!("Turn {} Dice Roll of Player {} - {} is {:?} and Total {}",turn,myturn,playerlist[(myturn-1)as usize],score,turnscores.get(&playerlist[(myturn-1)as usize]).unwrap());
            let nowplayer = playerlist[(myturn-1)as usize].clone();
            let mut playerout = false;
            let mut index = 0usize;
            let mut indexout = 0usize;
            //Checking if any player have the same score 
            for out in playerlist.iter(){
                index = index + 1;
                if out != &nowplayer {
                    if turnscores.get(out) == Some(myscore) {
                        println!("\nAlas! {} has kicked by {} at score of  {:?}\n",out,nowplayer,turnscores.get(out).unwrap());
                        //println!("by {} {:?}",nowplayer, Some(myscore));
                        indexout = index;
                        playerout = true;
                    }

                }
            }
            //updating score to 0 if someone kicked out
            if playerout {
                turnscores.insert(playerlist[indexout-1].clone(),0);
            }

            //checking if someone reach at 100
            if turnscores.get(&playerlist[(myturn-1)as usize]) >=Some(&100)  {
                
                println!("\n\nCongratulations!  Player {} has won on turn {} \n\n",playerlist[(myturn-1)as usize],turn);
                
                turn=0;
                break;
            }
            //println!("{}",myturn);
    
        }   
        if turn==0 {
            //println!("{:?}",turnscores);    
            break
        }
        turn = turn + 1;
        //waiting for user to press enter and then continue to next turn,
        let mut stop = String::new();
        io::stdin().read_line(&mut stop).expect("");
    }   
}