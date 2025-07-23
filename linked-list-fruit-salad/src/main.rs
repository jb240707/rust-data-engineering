/*
As with the VecDeque example, this code starts by creating a LinkedList of fruits,
converts it to a Vec for shuffling, and then converts it back to a LinkedList.
After the shuffling, it adds "Pomegranate", "Fig", and "Cherry" to the end of the list.
Finally, it prints out the final fruit salad.

This example shows how to use a LinkedList, but remember that LinkedList
has a higher memory overhead and worse cache locality than Vec or VecDeque,
so it's typically not the best choice unless you have a specific need for the properties
of a linked list. In Rust, it's usually better to use a Vec or VecDeque.

A LinkedList is a doubly-linked list, which means that each element in the list
has a pointer to the next element and the previous element.
A great example of when to use a LinkedList is when you need to insert or remove elements
from the middle of the list.
*/

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;
use std::collections::LinkedList;
use std::io;

fn main() {
    let mut fruit: LinkedList<&str> = LinkedList::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Convert it back to LinkedList
    let mut fruit: LinkedList<_> = fruit.into_iter().collect();

    // Add fruits to the both ends of the list after shuffling
    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");

    // Ask user to add a fruit at any position
    println!("Enter a fruit to add:");
    let mut fruit_name = String::new();
    io::stdin()
        .read_line(&mut fruit_name)
        .expect("Failed to read line");
    let fruit_name = fruit_name.trim().to_string();

    println!("Enter the position (0-based index) to insert the fruit:");
    let mut pos_str = String::new();
    io::stdin()
        .read_line(&mut pos_str)
        .expect("Failed to read line");
    let pos: usize = pos_str.trim().parse().unwrap_or(0);

    // Insert the fruit at the specified position
    let temp: LinkedList<String> = fruit.into_iter().map(|s| s.to_string()).collect();
    let mut new_list = LinkedList::new();
    let mut inserted = false;
    for (i, item) in temp.into_iter().enumerate() {
        if i == pos {
            new_list.push_back(fruit_name.clone());
            inserted = true;
        }
        new_list.push_back(item);
    }
    if !inserted {
        new_list.push_back(fruit_name);
    }

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in new_list.iter().enumerate() {
        if i != new_list.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }

    // Select a random fruit from the salad
    let fruit_vec: Vec<_> = new_list.iter().collect();
    let mut rng = thread_rng();
    if let Some(random_fruit) = fruit_vec.choose(&mut rng) {
        println!("Randomly selected fruit: {}", random_fruit);
    } else {
        println!("The fruit salad is empty!");
    }

    // Ask user for a position to remove a fruit
    println!("Enter the position (0-based index) of the fruit to remove:");
    let mut remove_str = String::new();
    io::stdin()
        .read_line(&mut remove_str)
        .expect("Failed to read line");
    let remove_pos: usize = remove_str.trim().parse().unwrap_or(usize::MAX);

    // Remove the fruit at the specified position
    let mut temp_vec: Vec<String> = new_list.into_iter().collect();
    if remove_pos < temp_vec.len() {
        let removed = temp_vec.remove(remove_pos);
        println!("Removed fruit: {}", removed);
    } else {
        println!("Invalid position, no fruit removed.");
    }

    // Show the state of the list after removal
    println!("Fruit Salad after removal:");
    for (i, item) in temp_vec.iter().enumerate() {
        if i != temp_vec.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
