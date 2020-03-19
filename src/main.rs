use egg_mode as twitter;
use std::env;

mod posting;
mod waiting;

use posting::get_tweet;
use waiting::wait_till_next_day;

const BIRTHDAYS_PATH: &str = "./birthdays.json";
const VILLAGERS_PATH: &str = "./data.json";

#[tokio::main]
async fn main() {
    /*let con_token = twitter::KeyPair::new(
        "E9ffwAWkg3MGHYglRqkeePJu9",
        // secret
    );
    
    // "oob" is needed for PIN-based auth; see docs for `request_token` for more info
    let request_token = join!(
        twitter::request_token(&con_token, "oob"),
        posting::load_data(BIRTHDAYS_PATH, VILLAGERS_PATH)
    ).0.unwrap();
    
    let auth_url = twitter::authorize_url(&request_token);

    // give auth_url to the user, they can sign in to Twitter and accept your app's permissions.
    // they'll receive a PIN in return, they need to give this to your application

    println!("Authorize here: {}", auth_url);
    print!("Enter pin: ");
    std::io::stdout().flush().unwrap();
    let mut verifier = String::new();
    std::io::stdin().read_line(&mut verifier).unwrap();

    // note this consumes con_token; if you want to sign in multiple accounts, clone it here
    let (token, _, screen_name) =
        twitter::access_token(con_token, &request_token, verifier).await.unwrap();*/

    posting::load_data(BIRTHDAYS_PATH, VILLAGERS_PATH).await;

    /*if screen_name != "ACBirthdayBot" {
        panic!("Not signed into the right account!");
    }*/

    /*let date = waiting::time_to_wait_for().date().naive_local();
    
    let birthdays = posting::data::get_birthdays().await;

    let todays_birthdays = birthdays.query_by_date(date);

    let mut image_futures: Vec<_> = 
        todays_birthdays
            .iter()
            .map(posting::data::Birthday::image)
            .collect();

    let img2 = image_futures.pop().unwrap().await;
    let img1 = image_futures.pop().unwrap().await;

    std::fs::write("test_out.png", posting::image_editing::add_background(vec![img1, img2]).await);*/

    let token = twitter::Token::Access {
        consumer: twitter::KeyPair::new(
            env::var("API_KEY").unwrap(),
            env::var("API_SECRET_KEY").unwrap(),
        ),
        access: twitter::KeyPair::new(
            env::var("ACCESS_TOKEN").unwrap(),
            env::var("ACCESS_TOKEN_SECRET").unwrap(),
        ),
    };

    loop {
        let date = waiting::time_to_wait_for().date().naive_local();
        let tweet = get_tweet(&token, date).await;
        wait_till_next_day().await;
        if let Some(tweet) = tweet {
            tweet.send(&token).await.unwrap();
            println!("Finished making post!");
        } else {
            println!("No post today");
        }
    }
}
