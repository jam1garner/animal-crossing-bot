# [Animal Crossing Birthday Bot](https://twitter.com/ACBirthdayBot)

A bot to post when all your favorite villager's birthdays are written in Rust.

![](https://cdn.discordapp.com/attachments/376971848555954187/689157291643240492/unknown.png)

## Prerequisites

* Must have `cargo` installed
* Must have villager pictures in `villagers/` folder. These can be found at `https://animal-crossing.com/assets/img/characters/[image path]` (see `birthdays.json` for image path) and should be named the capitalized form of the name (example: `[project]/villagers/Boots.png`)

## Run

1. Configure your environment variables with your twitter API/Access keys/secrets.

The following are required:
```
   API_KEY
   API_SECRET_KEY
   ACCESS_TOKEN
   ACCESS_TOKEN_SECRET
```

2. `cargo run`

