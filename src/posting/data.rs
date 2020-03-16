use std::collections::HashMap;
use serde::{Deserialize, Deserializer};
use serde_json::from_str;
use chrono::naive::NaiveDate;
use chrono::Datelike;
use tokio::fs;
use std::path::Path;
use tokio::sync::{RwLock, RwLockReadGuard};
use tokio::join;
use std::fmt;

#[derive(Deserialize, Debug, Clone)]
pub struct Birthday {
    pub id: usize,
    pub name: String,
    #[serde(deserialize_with = "deserialize_date", rename="birthday")]
    pub date: NaiveDate,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Villager {
    pub name: String,
    pub species: String,
    pub gender: Gender,
    pub personality: String,
    pub games: Vec<Game>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Game {
    in_game: bool,
    title: Option<String>,
}

impl Birthday {
    pub async fn image(&self) -> Vec<u8> {
        fs::read(format!("{}/{}.png", PICTURES_PATH, self.name)).await.unwrap()
    }

    pub fn star_sign(&self) -> StarSign {
        StarSign::from(self.date)
    }
}

const PICTURES_PATH: &str = "./villagers";
const FORMAT: &str = "%d-%m-%Y";

pub fn deserialize_date<'de, D>(
    deserializer: D,
) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
}

async fn read_birthdays_from_file<P: AsRef<Path>>(path: P) -> Vec<Birthday> {
    let path = path.as_ref();

    from_str(&fs::read_to_string(path).await.unwrap()).unwrap()
}

async fn read_villagers_from_file<P: AsRef<Path>>(path: P) -> HashMap<String, Villager> {
    let path = path.as_ref();

    let villagers: Vec<Villager> = from_str(&fs::read_to_string(path).await.unwrap()).unwrap();
    villagers
        .into_iter()
        .map(|v|{
            (v.name.clone(), v)
        })
        .collect()
}

lazy_static::lazy_static!{
    static ref BIRTHDAYS: RwLock<Option<Vec<Birthday>>> = RwLock::new(None);
    static ref VILLAGERS: RwLock<Option<HashMap<String, Villager>>> = RwLock::new(None);
}

pub async fn load_data<'a, P: AsRef<Path>>(birthdays_path: P, villaers_path: P) {
    let mut birthdays = BIRTHDAYS.write().await;
    let mut villagers = VILLAGERS.write().await;

    let (b, v) = join!(
        read_birthdays_from_file(birthdays_path),
        read_villagers_from_file(villaers_path)
    );
    *birthdays = Some(b);
    *villagers = Some(v);
}

pub async fn get_birthdays<'a>() -> Birthdays<'a> {
    Birthdays(BIRTHDAYS.read().await)
}

pub async fn get_villager<'a, I: Into<String>>(name: I) -> Option<Villager> {
    VILLAGERS.read().await.as_ref().unwrap().get(&name.into()).cloned()
}

pub struct Birthdays<'a>(pub RwLockReadGuard<'a, Option<Vec<Birthday>>>);

impl<'a> Birthdays<'a> {
    pub fn query_by_date(&self, date: NaiveDate) -> Vec<Birthday> {
        let birthdays = &*self.0;

        birthdays
            .as_ref()
            .unwrap()
            .iter()
            .filter(|bday|{
                bday.date.day() == date.day() && bday.date.month() == date.month()
            })
            .cloned()
            .collect()
    }
}

pub enum StarSign {
    Capricorn,
    Aquarius,
    Pisces,
    Aries,
    Taurus,
    Gemini,
    Cancer,
    Leo,
    Virgo,
    Libra,
    Scorpio,
    Sagittarius,
}

impl<D: Datelike> From<D> for StarSign {
    fn from(date: D) -> Self {
        match (date.month(), date.day()) {
            (1, day) if day <= 20 => Self::Capricorn,
            (1, _) => Self::Aquarius,
            (2, day) if day <= 19 => Self::Aquarius,
            (2, _) => Self::Pisces,
            (3, day) if day <= 20 => Self::Pisces,
            (3, _) => Self::Aries,
            (4, day) if day <= 20 => Self::Aries,
            (4, _) => Self::Taurus,
            (5, day) if day <= 21 => Self::Taurus,
            (5, _) => Self::Gemini,
            (6, day) if day <= 21 => Self::Gemini,
            (6, _) => Self::Cancer,
            (7, day) if day <= 22 => Self::Cancer,
            (7, _) => Self::Leo,
            (8, day) if day <= 21 => Self::Leo,
            (8, _) => Self::Virgo,
            (9, day) if day <= 23 => Self::Virgo,
            (9, _) => Self::Libra,
            (10, day) if day <= 23 => Self::Libra,
            (10, _) => Self::Scorpio,
            (11, day) if day <= 22 => Self::Scorpio,
            (11, _) => Self::Sagittarius,
            (12, day) if day <= 22 => Self::Sagittarius,
            _ => Self::Capricorn
        }
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
    #[serde(other)]
    Other
}

impl Gender {
    /// Get the assumed pronouns by gender, may not be reflective of gender used
    pub fn pronouns(&self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::Male => ("He", "Him", "His"),
            Self::Female => ("She", "Her", "Hers"),
            _ => ("They", "Them", "Theirs")
        }
    }

    pub fn is_or_are(&self) -> &'static str {
        match self {
            Self::Other => "are",
            _ => "is"
        }
    }
}

impl fmt::Display for StarSign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Capricorn => "Capricorn",
            Self::Aquarius => "Aquarius",
            Self::Pisces => "Pisces",
            Self::Aries => "Aries",
            Self::Taurus => "Taurus",
            Self::Gemini => "Gemini",
            Self::Cancer => "Cancer",
            Self::Leo => "Leo",
            Self::Virgo => "Virgo",
            Self::Libra => "Libra",
            Self::Scorpio => "Scorpio",
            Self::Sagittarius => "Sagittarius",
        })
    }
}
