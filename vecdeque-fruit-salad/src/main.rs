/*
This code starts with an initial VecDeque,
converts it to a Vec for shuffling, and then converts it back to a VecDeque.
After that, it pushes "Pomegranate" to the front of the deque, and "Fig" and "Cherry"
to the back of the deque. Finally, it prints out the final fruit salad.

A VecDeque is a double-ended queue, which means that you can push and pop from both ends
of the queue.
*/

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;

fn main() {
    let mut fruit: VecDeque<String> = VecDeque::new();
    fruit.push_back("Arbutus".to_string());
    fruit.push_back("Loquat".to_string());
    fruit.push_back("Strawberry Tree Berry".to_string());

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    let mut fruit_vec: Vec<String> = fruit.into_iter().collect();
    fruit_vec.shuffle(&mut rng);

    // Convert it back to VecDeque<String>
    let mut fruit: VecDeque<String> = fruit_vec.into_iter().collect();

    use std::io::{self, Write};
    loop {
        println!("Enter a fruit name to add (or press Enter to finish):");
        let mut fruit_name = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut fruit_name).unwrap();
        let trimmed = fruit_name.trim();
        if trimmed.is_empty() {
            break;
        }
        let new_fruit = trimmed.to_string();
        println!("Add to front or back? (f/b):");
        let mut end_choice = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut end_choice).unwrap();
        let end_choice = end_choice.trim().to_lowercase();
        if end_choice == "f" {
            fruit.push_front(new_fruit);
        } else {
            fruit.push_back(new_fruit);
        }
    }

    // Optionally remove a fruit from either end
    println!("Remove a fruit from front or back? (f/b, or Enter to skip):");
    let mut remove_choice = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut remove_choice).unwrap();
    let remove_choice = remove_choice.trim().to_lowercase();
    let removed = if remove_choice == "f" {
        fruit.pop_front()
    } else if remove_choice == "b" {
        fruit.pop_back()
    } else {
        None
    };
    if let Some(name) = removed {
        println!("Removed fruit: {}", name);
    } else if !remove_choice.is_empty() {
        println!("No fruit was removed.");
    }

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }

    // Select a random fruit from the salad
    let mut rng = thread_rng();
    let fruit_vec: Vec<_> = fruit.iter().collect();
    if let Some(random_fruit) = fruit_vec.choose(&mut rng) {
        println!("Randomly selected fruit: {}", random_fruit);
    } else {
        println!("No fruit in the salad to select.");
    }
}
