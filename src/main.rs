extern crate rand;

use rand::Rng;
use std::io;
use console::style;

// tuple type containing card suit + card number
type Card = (i32, &'static str);  

fn main() {
    game_loop();
}

// Primary game loop
fn game_loop() {

    let max_value = 21;
    let dealer_stop_value = 17;

    let mut hand_count = 1;

    //Vector contains type Card which is a tuple i32 and string
    let mut hand = vec![generate_card(), generate_card()];
    let mut dealer_hand = vec![generate_card(), generate_card()];

    //Prints hand
    print_hand(hand.as_mut_slice(), false);

    while hand_value(hand.as_mut_slice()) < max_value {
        
        //Declare empty buffer each iteration
        let mut buffer = String::new();

        //Player turn
        println!("Would you like to [h]it or [s]tay?");
        io::stdin().read_line(&mut buffer).expect("Failed to read line");
    
        let buffer= buffer.trim();
        
        if buffer == "h" {
            hand.push(generate_card());
            hand_count += 1;
            println!("Added {} to hand", format!("{} {}", hand[hand_count].0, hand[hand_count].1));
            print_hand(hand.as_mut_slice(), false);
        } else if buffer == "s" {
            println!("You stayed your turn.");
        } else {
            println!("Incorrect input.");
        }

        //Dealer turn
        if hand_value(dealer_hand.as_mut_slice()) < dealer_stop_value {
            dealer_hand.push(generate_card());
            print_hand(dealer_hand.as_mut_slice(), true);
        } else if hand_value(dealer_hand.as_mut_slice()) == dealer_stop_value {
            println!("Dealer stopped at {}", hand_value(dealer_hand.as_mut_slice()))
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

fn print_hand(hand_to_print: &mut [Card], is_dealer: bool) {
    if is_dealer {
        println!("{}", style("\nCard in dealers hand:").blue());
        for &(number, suit) in hand_to_print.iter() {
            println!("{} {}", number, suit)
        }

        println!("\nHand Value. {}", hand_value(hand_to_print));
    } else {
        println!("{}", style("\nCard in your hand:").green());
        for &(number, suit) in hand_to_print.iter() {
            println!("{} {}", number, suit)
        }

        println!("\nHand Value. {}", hand_value(hand_to_print));
    }
}

fn hand_value(hand_to_print: &mut [Card]) -> i32 {
    let mut card_values = 0;

    for &(number, _suit) in hand_to_print.iter() {
        card_values += number;
    }
    return card_values
}