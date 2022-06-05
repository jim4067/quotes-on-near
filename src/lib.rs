//imports
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::{env, near_bindgen};
use std::fmt;

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
#[near_bindgen]
impl Quote {
    //
    pub fn is_empty(&self) -> bool {
        self.quote.is_empty() && self.character.is_none() && self.show.is_none()
    }
}

//core implementation of the aniQuotes Struct
#[near_bindgen]
impl AniQuotes {
    pub fn new_quote(&mut self, #[serializer(borsh)] quote: Quote) {
        if quote.is_empty() {
            env::log_str("EMPTY QUOTE!!! NOTHING ADDED... RETURNING...");
            return;
        }

        self.quotes.push(&quote);
        env::log_str(format!("NEW QUOTE ADDED -> {:?}", quote.quote).as_str());
        return;
    }

    pub fn add_many_quotes(&mut self, #[serializer(borsh)] quotes: Vec<Quote>) {
        //check if the vec is empty
        if !quotes.is_empty() {
            // let quotes = quotes.iter();
            env::log_str(format!("\nADDED {} QUOTES", &quotes.len()).as_str());
            self.quotes.extend(quotes); //value moved here
            return;
        }

        env::log_str("\nNO QUOTES ADDED, EXITING...\n");
        return;
        //add values to the quotes vector
    }

    pub fn view_quotes(&self) {
        // env::log_str("these are the saved quotes \n");
        for quote in self.quotes.iter() {
            println!(" quote: {}", quote.quote);
            println!(" show: {}", quote.show.unwrap_or("".to_string()));
            println!(
                " character: {}\n",
                quote.character.unwrap_or("".to_string())
            );
        }
    }

    pub fn delete_quote(&mut self, index: Option<usize>) {
        //if the index is not supplied delete item at index 0
        if index.is_none() {
            let index = 0;
            env::log_str(
                format!(
                    "\nDELETING QUOTE quotes[{index}]\n{:?}", //PAY ATTENTION TO INDEX HERE
                    self.quotes.get(index as u64).unwrap().quote
                )
                .as_str(),
            );
            // self.quotes.to_vec().remove(index);
            self.quotes.swap_remove(index as u64);
            return;
        }

        let index = index.unwrap();
        env::log_str(
            format!(
                "\nDELETING QUOTE quotes[{}]\n{:?}",
                index,
                self.quotes.get(index as u64).unwrap().quote
            )
            .as_str(),
        );
        // self.quotes.to_vec().remove(index);
        self.quotes.swap_remove(index as u64);
        return;
    }

    //clears the vector but the allocated memory remains.
    //try using retain instead
    pub fn delete_all_quotes(&mut self) {
        env::log_str("\nDELETING EVERYTHING!!!\n");
        self.quotes.clear();
    }
}

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
    fn test_adding_quote() {
        let context = get_context(false);
        testing_env!(context);

        let mut contract = AniQuotes::default();

        let quote = "this is the one".to_string();
        let show = Some("my special creations".to_string());
        let character = Some("the foolish one".to_string());

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

        let quotes = vec![quote_one, quote_two, quote_three];
        contract.add_many_quotes(quotes);

        assert_eq!(contract.quotes.len(), 3);
    }

    #[test]
    fn test_delete() {
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

        let quote_four = Quote {
            show: "Bleach".to_string().into(),
            character: "Kyouya Sata".to_string().into(),
            quote: "It doesn't matter how much trash you pick up. You just have a pile of trash."
                .to_string(),
        };

        //add multiple quotes
        let quotes = vec![quote_one, quote_two, quote_three, quote_four];
        contract.add_many_quotes(quotes);

        //testing deleting without params specified
        contract.delete_quote(None);

        //testing deleting one quote
        contract.delete_quote(2.into());
        assert_eq!(contract.quotes.len(), 2);

        //test deleting everything
        contract.delete_all_quotes();
        assert!(contract.quotes.is_empty());
    }

    #[test]
    fn test_adding_empty() {
        let context = get_context(false);
        testing_env!(context);

        let mut contract = AniQuotes::default();

        //testing empty string
        let empty_quote = Quote {
            quote: "".to_string(),
            show: None,
            character: None,
        };
        contract.new_quote(empty_quote);
        assert!(contract.quotes.is_empty());

        //testing for adding vector
        let empty_vector: Vec<Quote> = Vec::new();
        contract.add_many_quotes(empty_vector);
        assert!(contract.quotes.is_empty());
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

        let quotes = vec![quote_one, quote_two, quote_three];
        contract.add_many_quotes(quotes);

        //
        contract.view_quotes();
    }
}
