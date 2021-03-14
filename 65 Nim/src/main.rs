use std::io;
use rand::Rng;

fn main() -> io::Result<()> {
    let mut rng = rand::thread_rng(); // an rng for random moves
    println!("{}{}", " ".repeat(33), "NIM");
    println!("{}{}", " ".repeat(15), "CREATIVE COMPUTING  MORRISTOWN, NEW JERSEY");
    println!();
    println!();
    println!();
    println!("THIS IS THE GAME OF NIM.");
    println!("DO YOU WANT INSTRUCTIONS");
    // standard input loop, the same construct is repeated everywhere in code where
    // an input of certain format is expected
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim() == "YES" || input.trim() == "yes" {
            println!("THE GAME IS PLAYED WITH A NUMBER OF PILES OF OBJECTS.");
            println!("ANY NUMBER OF OBJECTS ARE REMOVED FROM ONE PILE BY YOU AND");
            println!("THE MACHINE ALTERNATELY.  ON YOUR TURN, YOU MAY TAKE");
            println!("ALL THE OBJECTS THAT REMAIN IN ANY PILE, BUT YOU MUST");
            println!("TAKE AT LEAST ONE OBJECT, AND YOU MAY TAKE OBJECTS FROM");
            println!("ONLY ONE PILE ON A SINGLE TURN.  YOU MUST SPECIFY WHETHER");
            println!("WINNING IS DEFINED AS TAKING OR NOT TAKING THE LAST OBJECT,");
            println!("THE NUMBER OF PILES IN THE GAME, AND HOW MANY OBJECTS ARE");
            println!("ORIGINALLY IN EACH PILE.  EACH PILE MAY CONTAIN A");
            println!("DIFFERENT NUMBER OF OBJECTS.");
            println!("THE MACHINE WILL SHOW ITS MOVE BY LISTING EACH PILE AND THE");
            println!("NUMBER OF OBJECTS REMAINING IN THE PILES AFTER  EACH OF ITS");
            println!("MOVES.");
            break;
        } else if input.trim() == "NO" || input.trim() == "no" {
            break;
        } else {
            println!("PLEASE ANSWER YES OR NO");
        }
    }

    let mut running = true;
    while running {
        let take_last: bool;
        let mut move_first: bool;
        let piles_count: usize;
        let mut pile_sizes: Vec::<i32>;
        println!();
        loop {
            println!("ENTER WIN OPTION - 1 TO TAKE LAST, 2 TO AVOID LAST");
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if input.trim() == "1" || input.trim() == "2" {
                take_last = input.trim() == "1";
                break;
            }
        }
        loop {
            println!("ENTER NUMBER OF PILES");
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if let Ok(n) = input.trim().parse::<usize>() {
                if n > 0 && n < 101 {
                    piles_count = n;
                    break;
                }
            }
        }
        pile_sizes = vec![0; piles_count];
        println!("ENTER PILE SIZES");
        for i in 0..piles_count {
            loop {
                println!("{}", i);
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                if let Ok(n) = input.trim().parse::<i32>() {
                    if n > 0 && n < 2001 {
                        pile_sizes[i] = n;
                        break;
                    }
                }
            }
        }
        println!("DO YOU WANT TO MOVE FIRST");
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if input.trim() == "YES" || input.trim() == "yes" {
                move_first = true;
                break;
            } else if input.trim() == "NO" || input.trim() == "no" {
                move_first = false;
                break;
            } else {
                println!("PLEASE ANSWER YES OR NO.");
            }
        }

        // the game logic itself
        loop {
            if move_first {
                println!("PILE SIZE");
                for i in 0..piles_count {
                    println!("{} {}", i, pile_sizes[i]);
                }
                // player to move
                loop {
                    println!("YOUR MOVE - PILE, NUMBER TO BE REMOVED");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    // parse two numbers separated by a space
                    let mut parts = input.trim().split(' ').map(|s| s.parse::<i32>());
                    match (parts.next(), parts.next()) {
                        (Some(Ok(a)), Some(Ok(b))) => {
                            if a >= 0 && (a as usize) < piles_count && b > 0 {
                                if b <= pile_sizes[a as usize] {
                                    pile_sizes[a as usize] -= b;
                                    break;
                                }
                            }
                        },
                        _ => {},
                    }
                }
            }
            move_first = true; // starting from the next loop player has to move
            println!("PILE SIZE");
            for i in 0..piles_count {
                println!("{} {}", i, pile_sizes[i]);
            }

            // the usual nim strategy is to keep xor of all pile sizes equal to zero
            // however there is one exception - if one is playing 'a misère game',
            // so that one doesn't want to take the last stone; in this case at one point the
            // normal move for winning would leave only the piles of size one; f.e.
            // 3 1 1 1 -> 1 1 1 1
            // and from that point the game progression can't be changed. So instead one has to
            // take all the stones from the pile:
            // 3 1 1 1 -> 0 1 1 1
            // would would force the other player to take the last stone

            // here the code logic slightly differs from that of the original code
            let mut nonempty_piles_count = 0;
            let mut one_stone_piles_count = 0;
            for &x in &pile_sizes {
                if x > 0 {
                    nonempty_piles_count += 1;
                }
                if x == 1 {
                    one_stone_piles_count += 1;
                }
            }
            match nonempty_piles_count - one_stone_piles_count {
                0 => {
                    if (one_stone_piles_count % 2 == 0) != take_last {
                        // this covers even number of piles + machine taking last
                        // and odd number of piles + player taking last
                        println!("MACHINE WINS");
                    } else {
                        println!("MACHINE LOSES");
                    }
                    break;
                },
                1 => {
                    // game is 1 move away from being over, depending on the goal either take the
                    // entire large pile or leave one stone in it
                    for i in 0..piles_count {
                        if pile_sizes[i] > 1 {
                            if (one_stone_piles_count % 2 == 0) == take_last {
                                pile_sizes[i] = 0;
                            } else {
                                pile_sizes[i] = 1;
                            }
                            break;
                        }
                    }
                    println!("PILE SIZE");
                    for i in 0..piles_count {
                        println!("{} {}", i, pile_sizes[i]);
                    }
                    println!("MACHINE WINS");
                    break;
                },
                _ => {
                    // game goes on, play to maintain the invariant
                    let xor = pile_sizes.iter().fold(0i32, |acc, x| acc ^ x);
                    if xor != 0 {
                        // to find the right pile to take from just try them all
                        for i in 0..piles_count {
                            if pile_sizes[i] ^ xor < pile_sizes[i] {
                                pile_sizes[i] ^= xor;
                                break;
                            }
                        }
                    } else {
                        // do a random move
                        loop {
                            let i = rng.gen_range(0..piles_count);
                            if pile_sizes[i] > 0 {
                                let x = rng.gen_range(0..pile_sizes[i])+1;
                                pile_sizes[i] -= x;
                                break;
                            }
                        }
                    }
                },
            }
        }

        println!("do you want to play another game");
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if input.trim() == "YES" || input.trim() == "yes" {
                break;
            } else if input.trim() == "NO" || input.trim() == "no" {
                running = false;
                break;
            } else {
                println!("PLEASE.  YES OR NO.");
            }
        }
    }
    Ok(())
}
