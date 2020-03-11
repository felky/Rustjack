extern crate rand;

use rand::Rng;
use std::io;
use console::style;
use console::Term;

// Card suit + card number
type Card = (i32, &'static str);  

fn main() {
    game_loop();
}

// Primary game loop
fn game_loop() {
    let term = Term::stdout();
    let max_value = 21;

    let mut hand_count = 1;
    let mut hand = vec![generate_card(), generate_card()];

    //Prints hand
    print_hand(hand.as_mut_slice());
    let mut buffer = String::new();

    while hand_value(hand.as_mut_slice()) < max_value {
        println!("Would you like to [h]it or [s]tay? [q] to stop.");
        io::stdin().read_line(&mut buffer).expect("Failed to read line");
    
        let buffer= buffer.trim();
        
        if buffer == "h" {
            hand.push(generate_card());
            hand_count += 1;
            println!("Added {} to hand", format!("{} {}", hand[hand_count].0, hand[hand_count].1));
            print_hand(hand.as_mut_slice());
        } else if buffer == "s" {
            println!("matched s!");
        } else if buffer == "q" {
            println!("You ended the match with {}", hand_value(hand.as_mut_slice()));
        }
    }

    if hand_value(hand.as_mut_slice()) == max_value {
        println!("{}", style("You Win").green());
    } else {
        println!("{}", style("You Lost...").red());
    }
}

fn generate_card() -> Card {
    let suites = ["of Spades", "of Hearts", "of Clubs", "of Diamonds"];
    let mut rng = rand::thread_rng();
    let card: Card = (rng.gen_range(1, 11), suites[rng.gen_range(0,3)]);

    return card
}

fn print_hand(hand_to_print: &mut [Card]) {
    println!("Cards in hand:");

    //Loops through vector to print cards
    for &(number, suit) in hand_to_print.iter() {
        println!("{} {}", number, suit)
    }

    println!("Hand Value. {}", hand_value(hand_to_print));
}

fn hand_value(hand_to_print: &mut [Card]) -> i32 {
    let mut card_values = 0;

    for &(number, _suit) in hand_to_print.iter() {
        card_values += number;
    }
    return card_values
}