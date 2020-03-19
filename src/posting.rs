use egg_mode::{tweet::DraftTweet, Token, media::{media_types, UploadBuilder}};
use chrono::{NaiveDate, Utc};
use tokio::join;
use std::future::Future;

pub mod data;
pub mod image_editing;

pub(crate) use data::load_data;

async fn upload(token: &Token, data: Vec<Vec<u8>>) -> u64 {
    let image = image_editing::add_background(data).await;

    join!(
        UploadBuilder::new(
            &image,
            media_types::image_png()
        ).call(&token),
        tokio::fs::write("next_image.png", &image)
    ).0.unwrap().id
}

async fn upload_all(t: &Token, mut futures: Vec<impl Future<Output = Vec<u8>>>) -> Option<u64> {
    Some(match futures.len() {
        0 => return None,
        1 => upload(t, vec![futures.pop().unwrap().await]).await,
        2 => {
            let (b, a) = join!(futures.pop().unwrap(), futures.pop().unwrap());
            upload(t, vec![a, b]).await
        }
        3 => {
            let (c, b, a) = join!(futures.pop().unwrap(), futures.pop().unwrap(), futures.pop().unwrap());
            upload(t, vec![a, b, c]).await
        }
        4 => {
            let (d, c, b, a) = join!(
                futures.pop().unwrap(),
                futures.pop().unwrap(),
                futures.pop().unwrap(),
                futures.pop().unwrap()
            );
            upload(t, vec![a, b, c, d]).await
        }
        _ => unreachable!()
    })
}

pub async fn get_tweet(token: &Token, date: NaiveDate) -> Option<DraftTweet<'_>> {
    let birthdays = data::get_birthdays().await;

    let todays_birthdays = birthdays.query_by_date(date);

    let message = match &todays_birthdays[..] {
        [] => {
            println!("No birthdays today :(");
            return None
        },
        [ref bday] => {
            let star_sign = bday.star_sign();
            match data::get_villager(&bday.name).await {
                Some(villager) => {
                    format!(
                        "Happy birthday to {}! {} {} a {} {} and {} star sign is {}.",
                        villager.name,
                        villager.gender.pronouns().0,
                        villager.gender.is_or_are(),
                        villager.personality,
                        villager.species,
                        villager.gender.pronouns().2.to_lowercase(),
                        star_sign
                    )
                }
                None => format!("Happy birthday to {}! They are a {}.", bday.name, star_sign)
            }
        },
        [ref bday1, ref bday2] => format!("Happy birthday to {} and {}! Two in one day!", bday1.name, bday2.name),
        [bdays @ .., ref last_bday] => {
            let name_list: Vec<_> = bdays.iter().map(|bday| &bday.name[..]).collect();
            format!(
                "Happy birthday to {}, and {}! Wow! That's a lot of birthdays!",
                name_list.join(", "),
                last_bday.name
            )
        }
    };

    println!("{}", message);

    let image_futures: Vec<_> = 
        todays_birthdays
            .iter()
            .map(data::Birthday::image)
            .collect();

    let media_id = upload_all(token, image_futures).await.unwrap();

    if !todays_birthdays.is_empty() {
        Some(
            DraftTweet::new(message)
                .media_ids(&[media_id])
        )
    } else {
        None
    }

}

pub(crate) async fn make_post(token: &Token) -> Option<()> {
    get_tweet(token, Utc::now().date().naive_local()).await?.send(token).await.ok()?;
    Some(())
}
