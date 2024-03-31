use std::fmt::Display;

pub mod show;
pub mod watched;
pub mod wl;

pub struct ValidatedTitle(String);

impl From<ValidatedTitle> for String {
    fn from(value: ValidatedTitle) -> Self {
        value.0
    }
}

impl Display for ValidatedTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ValidatedTitle {
    fn from_pattern(titles: Vec<String>, pattern: &str) -> Result<Self, String> {
        let lowercase_pattern = pattern.to_lowercase();

        for key in titles.iter() {
            if key == pattern {
                eprintln!("successful exact case-sensitive match: {}", key);
                return Ok(ValidatedTitle(key.to_owned()));
            }
        }

        for key in titles.iter() {
            if key.to_lowercase() == lowercase_pattern {
                eprintln!("successful exact case-insensitive match: {}", key);
                return Ok(ValidatedTitle(key.to_owned()));
            }
        }

        let mut candidates: Vec<_> = titles
            .iter()
            .filter(|&show| show.to_lowercase().contains(&lowercase_pattern))
            .collect();

        if candidates.is_empty() {
            return Err("unsuccessful case-insensitive substring match".into());
        }

        let insensitive_candidates = candidates.clone();
        let mut retained = false;
        if candidates.len() > 1 {
            candidates.retain(|&show| show.contains(pattern));
            retained = true;
        }

        match candidates.len() {
            0 => Err(format!(
            "case-insensitive substring match (too many): '{}'\nand then, unsuccessful case-sensitive substring match",
                insensitive_candidates
                    .iter()
                    .map(|candidate| *candidate as &str)
                    .collect::<Vec<&str>>()
                    .join("', '")
            )),
            1 => {
                // if we got here without retaining, that means we matched precisely a single show
                // case-insensitively.
                // if we did retain, that means we could only get to precisely 1 show once we
                // searched case-sensitively
                eprintln!("successful case-{}sensitive substring match: '{}'", if retained { "" } else { "in" } , candidates[0]);
                Ok(ValidatedTitle(candidates[0].to_owned()))
            },
            _ => Err(format!(
                "case-insensitive substring match (too many): '{}'\nand then, case-sensitive substring match (too many): '{}'",
                insensitive_candidates
                    .iter()
                    .map(|candidate| *candidate as &str)
                    .collect::<Vec<&str>>()
                    .join("', '"),
                candidates
                    .iter()
                    .map(|candidate| *candidate as &str)
                    .collect::<Vec<&str>>()
                    .join("', '")
            )),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
