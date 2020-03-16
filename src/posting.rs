use egg_mode::{tweet::DraftTweet, Token, media::{media_types, UploadBuilder}};
use chrono::{NaiveDate, Utc};
use tokio::join;
use std::future::Future;

mod data;
mod image_editing;

pub(crate) use data::load_data;

async fn upload(token: &Token, data: Option<impl Future<Output = Vec<u8>>>) -> u64 {
    let image = image_editing::add_background(data.unwrap().await).await;

    join!(
        UploadBuilder::new(
            &image,
            media_types::image_png()
        ).call(&token),
        tokio::fs::write("next_image.png", &image)
    ).0.unwrap().id
}

async fn upload_all(t: &Token, mut futures: Vec<impl Future<Output = Vec<u8>>>) -> Vec<u64> {
    match futures.len() {
        0 => vec![],
        1 => vec![upload(t, futures.pop()).await],
        2 => {
            let (b, a) = (futures.pop(), futures.pop());
            let (a, b) = join!(upload(t, a), upload(t, b));
            vec![a, b]
        }
        3 => {
            let (c, b, a) = (futures.pop(), futures.pop(), futures.pop());
            let (a, b, c) = join!(
                upload(t, a),
                upload(t, b),
                upload(t, c)
            );
            vec![a, b, c]
        }
        4 => {
            let (d, c, b, a) = (futures.pop(), futures.pop(), futures.pop(), futures.pop());
            let (a, b, c, d) = join!(
                upload(t, a),
                upload(t, b),
                upload(t, c),
                upload(t, d),
            );
            vec![a, b, c, d]
        }
        _ => unreachable!()
    }
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

    let media_ids = upload_all(token, image_futures).await;

    if !todays_birthdays.is_empty() {
        Some(
            DraftTweet::new(message)
                .media_ids(&media_ids)
        )
    } else {
        None
    }

}

pub(crate) async fn make_post(token: &Token) -> Option<()> {
    get_tweet(token, Utc::now().date().naive_local()).await?.send(token).await.ok()?;
    Some(())
}
