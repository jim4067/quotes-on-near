//imports
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::{env, near_bindgen};
use std::fmt;

//create a struct AniQuotes to hold the vec for the quotes and maybe the user who created them
//create a struct Quotes to hold info about the quote like, anime_show, character, and the quote itself

//NEAR INIT FUNCTION ISN'T A MUST

//main struct
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize /* Debug */)]
pub struct AniQuotes {
    pub quotes: Vector<Quote>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Quote {
    quote: String,
    character: Option<String>,
    show: Option<String>,
}

//default implementation for main struct
impl Default for AniQuotes {
    fn default() -> Self {
        Self {
            quotes: Vector::new(b"m"),
        }
    }
}

impl fmt::Display for Quote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //
        write!(
            f,
            "\nquote: {}\nshow: {:?}\ncharacter: {:?} \n",
            self.quote, self.show, self.character
        )
    }
}

//default implementation for the quotes struct
impl Default for Quote {
    fn default() -> Self {
        Self {
            quote: "".to_string(),
            character: "".to_string().into(),
            show: "".to_string().into(),
        }
    }
}

// //core implementation for the Quote
// #[near_bindgen]
// impl Quote {
//     pub fn new_quote(quote: String, show: String, character: String) -> Self {
//         // env::log_str(format!("added new quote -> {:?}", quote).as_str());

//         let quote = quote;
//         let show = show.into();
//         let character = character.into();

//         Self {
//             quote,
//             character,
//             show,
//         }
//     }
// }

//core implementation of the aniQuotes Struct
#[near_bindgen]
impl AniQuotes {
    pub fn new_quote(&mut self, #[serializer(borsh)] quote: Quote) {
        self.quotes.push(&quote);
        env::log_str(format!("NEW QUOTE ADDED{}", quote).as_str());
    }

    pub fn add_many_quotes(&mut self, #[serializer(borsh)] quotes: Vector<Quote>) {
        //check if the slice is empty
        if !quotes.is_empty() {
            // let quotes = quotes.iter();
            self.quotes.extend(quotes.iter());
            env::log_str(format!("ADDED {} QUOTES", quotes.len()).as_str());
        }

        env::log_str("NO QUOTES FOUND, EXITING...");
        return;
        //add values to the quotes vector
    }

    pub fn view_quotes(&self) {
        env::log_str("these are the saved quotes \n");
        for (index, quote) in self.quotes.iter().enumerate() {
            println!("index {}. -> {:?}", index, quote);
        }

        //TODO
        //implement display for quote
    }

    pub fn delete_quote(&mut self, index: usize) {
        env::log_str(format!("\nDELETING QUOTE\n {:?}", self.quotes.get(index as u64)).as_str());
        // self.quotes.to_vec().remove(index);
        self.quotes.swap_remove(index as u64);
    }

    //clears the vector but the allocated memory remains.
    //try using retain instead
    pub fn delete_all_quotes(&mut self) {
        env::log_str("\nDELETING EVERYTHING!!!\n");
        self.quotes.clear();
    }
}

// //core logic / struct implementation
// #[near_bindgen]
// impl AniQuotes {
//     pub fn add_quote(&mut self, quote: String) {
//         //
//         env::log_str("added quote to chain");
//         self.quote.push(&quote);
//     }

//     // pub fn view_quote() -> Option<Vec<String>> {
//     // //for testing using is_some or is_none to check if the option contains something

//     // }

//     // pub fn delete_quote() {
//     //     //
//     // }
// }

//tests
#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext};
    use std::convert::TryInto;

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("bob_near".to_string().try_into().unwrap())
            .is_view(is_view)
            .build()
    }

    #[test]
    fn dummy() {
        println!("this surely should work");
    }

    #[test]
    fn test_adding_quote() {
        let context = get_context(false);
        testing_env!(context);

        let mut contract = AniQuotes::default();

        let quote = "this is the one".to_string();
        let show = Some("my special creations".to_string());
        let character = Some("the foolish one".to_string());

        // let show = "peak shonen".to_string();
        // let character = "peak".to_string();

        // let one = Quote::new_quote(quote, show, character);
        // println!("this is the quote struct -> {:?}", one);

        let two = Quote {
            quote,
            show,
            character,
        };

        contract.new_quote(two);

        assert_eq!(contract.quotes.len(), 1);
    }

    #[test]
    fn test_adding_multiple_quotes() {
        let context = get_context(false);
        testing_env!(context);

        let mut contract = AniQuotes::default();

        let quote_one = Quote {
            quote: "I might seem to be on the losing side to you, but I'm still fighting... Not all battles happen in the brightest, flashiest spots.".to_string(),
            character: "Saeki Ryutaro".to_string().into(),
            show: "Solanin".to_string().into()
        };

        let quote_two = Quote {
            show: "Bleach".to_string().into(),
            character: "Kyouya Sata".to_string().into(),
            quote: "It doesn't matter how much trash you pick up. You just have a pile of trash."
                .to_string(),
        };

        let quote_three = Quote {
            show: None,
            character: None,
            quote: "this is an edge test.".to_string(),
        };

        // println!("\n one -> {quote_one:?}\n two -> {quote_two:?}\n three{quote_three:?}");

        contract.new_quote(quote_one);
        contract.new_quote(quote_two);
        contract.new_quote(quote_three);

        assert_eq!(contract.quotes.len(), 3);
    }

    #[test]
    fn test_delete_everything() {
        let context = get_context(false);
        testing_env!(context);

        let mut contract = AniQuotes::default();

        let quote_one = Quote {
            quote: "I might seem to be on the losing side to you, but I'm still fighting... Not all battles happen in the brightest, flashiest spots.".to_string(),
            character: "Saeki Ryutaro".to_string().into(),
            show: "Solanin".to_string().into()
        };

        contract.new_quote(quote_one);

        contract.delete_all_quotes();

        assert_eq!(contract.quotes.len(), 0);
    }

    #[test]
    fn test_delete_entry() {
        let context = get_context(false);
        testing_env!(context);

        let mut contract = AniQuotes::default();

        let quote_one = Quote {
            quote: "I might seem to be on the losing side to you, but I'm still fighting... Not all battles happen in the brightest, flashiest spots.".to_string(),
            character: "Saeki Ryutaro".to_string().into(),
            show: "Solanin".to_string().into()
        };

        contract.new_quote(quote_one);

        contract.delete_quote(0);

        assert!(contract.quotes.is_empty());
        // assert_eq!(contract.quotes.len(), 0);
    }

    #[test]
    fn test_viewing_quotes() {
        let context = get_context(false);
        testing_env!(context);

        let mut contract = AniQuotes::default();

        let quote_one = Quote {
            quote: "I might seem to be on the losing side to you, but I'm still fighting... Not all battles happen in the brightest, flashiest spots.".to_string(),
            character: "Saeki Ryutaro".to_string().into(),
            show: "Solanin".to_string().into()
        };

        let quote_two = Quote {
            show: "Bleach".to_string().into(),
            character: "Kyouya Sata".to_string().into(),
            quote: "It doesn't matter how much trash you pick up. You just have a pile of trash."
                .to_string(),
        };

        let quote_three = Quote {
            show: None,
            character: None,
            quote: "this is an edge test.".to_string(),
        };

        let one = vec!["one"];

        // let quotes : Vector<Quote> = [quote_one, quote_two, quote_three];
        

        // implement add many for contracts
        // contract.add_many_quotes(quotes);
    }

    //     #[test]
    //     fn create_and_read_quote() {
    //         let context = get_context(false);
    //         testing_env!(context);

    //         let mut contract = AniQuotes::default();

    //         let quote = "A farewell is like the other side of a coin. It will only happen when there is an encounter. Rather than dreading the day we have to part, shouldn't we be happy that we had the chance to meet today instead?";
    //         contract.add_quote(quote.to_string());
    //         contract.add_quote("How does it handle two values".to_string());

    //         //since we only added one quote we expect the first index of
    //         //our vec to be the quote when we read it
    //         println!("the contract length -> {:?}", contract.quote.len());

    //         let mut quotes_iterator = contract.quote.iter();

    //         println!(
    //             "the items in the contract {:?}",
    //             // contract.quote.iter().next().unwrap()
    //             quotes_iterator.next()
    //         );
    //         println!(
    //             "the second one -> {:?}",
    //             // contract.quote.iter().next().unwrap()
    //             quotes_iterator.next()
    //         );
    //         // for item in contract.quote.iter() {
    //         //     println!("the items here -> {:?}", item);
    //         // }
    //         // println!("the items in the contract -> {:?}", contract.quote);
    //     }

    //     //test what happens when more than one quote is added
}
