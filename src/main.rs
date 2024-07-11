extern crate serde_derive;

use std::{io, process};
use std::io::Write;

mod blockchain;
mod transaction;
mod block;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("input a miner address: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut miner_addr).unwrap();

    print!("difficulty: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut difficulty).unwrap();
    let diff = difficulty.trim().parse::<u32>().expect("integer required");

    println!("generating genesis block");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("menu");
        println!("1. New transaction \n2. Mine block \n3. Change difficulty \n4. Change reward \n0. Exit");
        print!("value: ");
        io::stdout().flush().unwrap();
        choice.clear();
        io::stdin().read_line(&mut choice).unwrap();
        println!();

        match choice.trim().parse().unwrap() {
            0 => {
                println!("exiting");
                process::exit(0);
            }
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("sender addr: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut sender).unwrap();

                print!("receiver addr: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut receiver).unwrap();

                print!("amount: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut amount).unwrap();

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap()
                );

                match res {
                    true => println!("transaction completed"),
                    false => println!("transaction failed")
                }
            }
            2 => {
                println!("generating new block");

                let res = chain.generate_new_block();

                match res {
                    true => println!("block was generated"),
                    false => println!("block generation failed")
                }
            }
            3 => {
                let mut new_diff = String::new();

                print!("new difficulty: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut new_diff).unwrap();

                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("difficulty was changed"),
                    false => println!("failed to change difficulty")
                }
            }
            4 => {
                let mut new_reward = String::new();

                print!("new reward: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut new_reward).unwrap();

                let res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("reward was changed"),
                    false => println!("failed to change reward")
                }
            }

            _ => {
                println!("invalid input")
            }
        }
    }
}
