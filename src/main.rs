use clap::Parser;
use parse_display::{Display, FromStr};

fn is_leap_year(year: u64) -> bool {
    // https://www.mathsisfun.com/leap-years.html
    (year % 4) == 0 && (year % 100 != 0 || year % 400 == 0)
}

#[derive(Debug)]
enum Weekday {
    Monday = 0,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl TryFrom<u8> for Weekday {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let result = match value {
            0 => Self::Monday,
            1 => Self::Tuesday,
            2 => Self::Wednesday,
            3 => Self::Thursday,
            4 => Self::Friday,
            5 => Self::Saturday,
            6 => Self::Sunday,
            _ => return Err(()),
        };

        Ok(result)
    }
}

#[derive(Clone, Debug, Display, FromStr)]
#[display("{day}.{month}.{year}")]
struct Date {
    day: u8,
    month: u8,
    year: u64,
}

impl Date {
    fn weekday(&self) -> Weekday {
        // https://www.reddit.com/r/NoStupidQuestions/comments/1hma5x8/i_met_a_guy_at_a_christmas_party_who_could_tell/
        let leap_year: i64 = is_leap_year(self.year).into();
        let century: i64 = match self.year {
            1753..=1799 => 4,
            1800..=1899 => 2,
            1900..=1999 => 0,
            2000..=2099 => -1,
            _ => todo!("Years outside [1753; 2099] have not yet been considered."),
        };
        let year = (self.year % 100) as i64;
        let months = [
            1i64 - leap_year,
            4 - leap_year,
            4,
            0,
            2,
            5,
            0,
            3,
            6,
            1,
            4,
            6,
        ];
        let month = months[(self.month - 1) as usize];

        let weekday = (((year + (year / 4) + month + self.day as i64 + century) + 5) % 7) as u8;

        weekday.try_into().expect("Math failed. :(")
    }
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    date: Date,
}

fn main() {
    let cli = Cli::parse();
    println!("{}: {:?}", cli.date, cli.date.weekday());
}
