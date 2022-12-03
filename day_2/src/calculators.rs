
pub fn calculate_regular_score(player_choice: &&str, opponent_choice: &&str) -> i32 {
    let mut points: i32 = 0;
    match player_choice {
        &"rock" => {
            points += 1;
            match opponent_choice {
                &"rock" => {
                    points += 3
                }
                &"scissors" => {
                    points += 6
                }
                _ => {}
            }
        },
        &"paper" => {
            points += 2;
            match opponent_choice {
                &"rock" => {
                    points += 6
                }
                &"paper" => {
                    points += 3
                }
                _ => {}
            }
        },
        &"scissors" => {
            points += 3;
            match opponent_choice {
                &"paper" => {
                    points += 6
                }
                &"scissors" => {
                    points += 3
                }
                _ => {}
            }
        },
        _ => {}
    }
    return points;
}
  

// Rock = lose
// Paper = draw
// Scissors = win

pub fn calculate_weird_score(player_choice: &&str, opponent_choice: &&str) -> i32 {
    let mut points: i32 = 0;
    match opponent_choice {
        &"rock" => {
            match player_choice {
                &"rock" => {
                    points += 3;
                },
                &"paper" => {
                    points += 4;
                },
                &"scissors" => {
                    points += 8
                }
                _ => {}
            }
        },
        &"paper" => {
            match player_choice {
                &"rock" => {
                    points += 1;
                },
                &"paper" => {
                    points += 5;
                },
                &"scissors" => {
                    points += 9
                }
                _ => {}
            }
        },
        &"scissors" => {
            match player_choice {
                &"rock" => {
                    points += 2;
                },
                &"paper" => {
                    points += 6;
                },
                &"scissors" => {
                    points += 7
                }
                _ => {}
            }
        },
        _ => {}
    }

    
    return points;
}
  
