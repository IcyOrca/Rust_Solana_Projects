u  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Favourites as anchor.Program<Favourites>;
  
import type { Favourites } from "../target/types/favourites";
se anchor_client::solana_sdk::signer::Signer;
use anchor_client::{Program, Provider};
use anchor_lang::prelude::*;
use crate::favourites;
use solana_program::pubkey::Pubkey;

#[tokio::test]
async fn test_set_favourites() {
    let program_id = Pubkey::from_str("6CWFZERMLpKbZJpXYJbrsoW5Bqs8MzTd7cL4ggxUKuz3").unwrap();
    let provider = Provider::default(); // Set up your provider as needed
    let program = Program::new(program_id, provider);

    // Prepare test data
    let user_key = Pubkey::new_unique(); // Create a unique public key for the user
    let number = 42;
    let color = "Blue".to_string();
    let hobbies = vec!["Coding".to_string(), "Gaming".to_string()];

    // Set up your transaction and context
    let context = Context::<SetFavourites>::new(program, user_key);

    // Call the program's set_favourites function
    let result = favourites::set_favourites(context, number, color, hobbies).await;

    // Assert that it was successful
    assert!(result.is_ok());

    // Optionally, check that the data was correctly stored in the account
    let account = program.account::<Favourites>(user_key).await.unwrap();
    assert_eq!(account.number, number);
    assert_eq!(account.color, color);
    assert_eq!(account.hobbies, hobbies);
}
