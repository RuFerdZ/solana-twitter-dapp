use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("AdGdR4tVHcxa36vEvPGh9QHsAMLAD8861puJzr6mntJj");

#[program]
pub mod solana_twitter {
    use super::*;
    // functions are snake_cased
    pub fn send_tweet(ctx: Context<SendTweet>, topic: String, content: String) -> ProgramResult {
        // taken from SendTweet Context
        let tweet: &mut Account<Tweet> = &mut ctx.accounts.tweet;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();

        //TODO: we cannot use 'my_string.len()' because it returns the number of bytes in the string
        // this will throw and exception:kinda thing and stop the program
        // No matter how many instructions and nested instructions exists inside a transaction, it will always be atomic — i.e. it's all or nothing.
        if topic.chars().count() > 50 {
            // Return a error...
            return Err(ErrorCode::TopicTooLong.into())
        }

        if content.chars().count() > 280 {
            // Return a error...
            return Err(ErrorCode::ContentTooLong.into())
        }

        tweet.author = *author.key;                //Let's start with the author's public key. We can access it via author.key but this contains a reference to the public key so we need to dereference it using *
        tweet.timestamp = clock.unix_timestamp;
        tweet.topic = topic;
        tweet.content = content;

        Ok(())
    }
}

// #[derive(Accounts)]
// pub struct Initialize {}
#[derive(Accounts)]
pub struct SendTweet<'info> {
    #[account(init, payer = author, space = Tweet :: LEN)]
    pub tweet : Account<'info, Tweet>,   // this specifies it is an account/program of type Tweet
    #[account(mut)]  // mute account
    pub author : Signer<'info>,  // // the signer is the account that is signing the transaction and somewhat similar to AccountInfo
    #[account(address = system_program::ID)]   // to validate the system program id matches solana system program id
    pub system_program: AccountInfo<'info>,  // AccountInfo represent account info of a program
}

// 1. Define the structure of the Tweet account.
#[account]
pub struct Tweet {
    pub author: Pubkey,
    pub timestamp: i64,
    pub topic: String,
    pub content: String,
}

// 2. Add some useful constants for sizing propeties.

// https://lorisleiva.com/create-a-solana-dapp-from-scratch/structuring-our-tweet-account

// Whenever a new account is created, a discriminator of exactly 8 bytes will be added to the very beginning of the data.
// That discriminator stores the type of the account. This way, if we have multiple types of accounts — say a Tweet account and a UserProfile account — then our program can differentiate them.
// Alright, let’s keep track of that information in our code by adding the following constant at the end of the lib.rs file.
const DISCRIMINATOR_LENGTH: usize = 8; // bytes

// This special looking struct defines an array. The size of each item is given in the first element and the length of the array is given in the second element. Therefore, that struct defines an array of 32 items of type u8. The type u8 means it’s an unsigned integer of 8 bits. Since there are 8 bits in one byte, we end up with a total array length of 32 bytes.
// That means, to store the author property — or any public key — we only need 32 bytes. Let’s also keep track of that information in a constant.
const PUBLIC_KEY_LENGTH: usize = 32; // bytes

// The timestamp property is of type i64. That means it’s an integer of 64 bits or 8 bytes.
// Let’s add a constant, see our updated storage representation and move on to the next property.
const TIMESTAMP_LENGTH: usize = 8; // bytes

// So let’s make a decision that a topic will have a maximum size of 50 characters. That should be enough for most topics out there.
// Now we need to figure out how many bytes are required to store one character.
// It turns out, using UTF-8 encoding, a character can use from 1 to 4 bytes. Since we need the maximum amount of bytes a topic could require, we’ve got to size our characters at 4 bytes each.
// Okay, so far we have figured out that our topic property should at most require 50 x 4 = 200 bytes.

// We’re almost done with our topic property but there’s one last thing to think about when it comes to the String type or vectors in general.
// Before storing the actual content of our string, there will be a 4 bytes prefix whose entire purpose is to store its total length. Not the maximum length that it could be, but the actual length of the string based on its content.
// That prefix is important to know where the next property is located on the array of bytes. Since vectors have no limits, without that prefix we wouldn’t know where it stops.
// Phew! Okay, now that we know how to size String properties, let’s define a few constants that summarise our findings.
const STRING_LENGTH_PREFIX: usize = 4; // Stores the size of the string.
const MAX_TOPIC_LENGTH: usize = 50 * 4; // 50 chars max.

// The only thing that differs from the topic property is the character count. Here, we want the content of our tweets to be a maximum of 280 characters which make the total size of our content 4 + 280 * 4 = 1124 bytes.
// As usual, let’s add a constant for this.
const MAX_CONTENT_LENGTH: usize = 280 * 4; // 280 chars max.

// 3. Add a constant on the Tweet account that provides its total size.
impl Tweet {
    const LEN: usize = DISCRIMINATOR_LENGTH // discriminator - 8 bytes - stores the type of account
        + PUBLIC_KEY_LENGTH // Author.
        + TIMESTAMP_LENGTH // Timestamp.
        + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH // Topic.
        + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH; // Content.
}

#[error]
pub enum ErrorCode {   // our custom error messages
    #[msg("The provided topic should be 50 characters long maximum.")]
    TopicTooLong,

    #[msg("The provided content should be 280 characters long maximum.")]
    ContentTooLong,
}