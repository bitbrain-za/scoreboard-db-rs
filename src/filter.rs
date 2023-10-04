use std::str::FromStr;

use crate::score::Score;
use log::trace;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortColumn {
    PlayerName,
    Binary,
    Language,
    Time,
}

impl FromStr for SortColumn {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "player" => Ok(SortColumn::PlayerName),
            "players" => Ok(SortColumn::PlayerName),
            "name" => Ok(SortColumn::PlayerName),
            "names" => Ok(SortColumn::PlayerName),
            "binaries" => Ok(SortColumn::Binary),
            "binary" => Ok(SortColumn::Binary),
            "time" => Ok(SortColumn::Time),
            "times" => Ok(SortColumn::Time),
            "language" => Ok(SortColumn::Language),
            "languages" => Ok(SortColumn::Language),
            _ => Err(format!("invalid sort column: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Filter {
    Top(usize),
    Bottom(usize),
    Player(Vec<String>),
    Binary(Vec<String>),
    Language(Vec<String>),
    UniquePlayers,
    UniqueBinaries,
    UniqueLanguages,
    Sort(SortColumn),
}

impl PartialEq for Filter {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Filter::Player(a), Filter::Player(b)) => a == b,
            (Filter::Binary(a), Filter::Binary(b)) => a == b,
            (Filter::UniquePlayers, Filter::UniquePlayers) => true,
            (Filter::UniqueBinaries, Filter::UniqueBinaries) => true,
            (Filter::UniqueLanguages, Filter::UniqueBinaries) => true,
            (Filter::Sort(a), Filter::Sort(b)) => a == b,

            /* only allow one of these */
            (Filter::Top(_), Filter::Top(_)) => true,
            (Filter::Bottom(_), Filter::Bottom(_)) => true,
            (Filter::Top(_), Filter::Bottom(_)) => true,
            (Filter::Bottom(_), Filter::Top(_)) => true,

            _ => false,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Builder {
    filters: Vec<Filter>,
}

impl Builder {
    pub fn new() -> Self {
        Builder {
            filters: Vec::new(),
        }
    }

    pub fn build(self) -> Collection {
        trace!("building collection: {:?}", self.filters);
        Collection {
            filters: self.filters.clone(),
        }
    }

    fn push(filters: &mut Vec<Filter>, filter: Filter) {
        trace!("adding filter: {:?}", filter);
        match filter.clone() {
            Filter::Player(players) => {
                let current = filters.iter().find(|f| matches!(f, Filter::Player(_)));
                if let Some(Filter::Player(current)) = current {
                    let mut players = players;
                    players.extend_from_slice(current);
                    players.dedup();
                    filters.retain(|f| !matches!(f, Filter::Player(_)));
                    filters.push(Filter::Player(players));
                } else {
                    filters.push(filter);
                }
            }
            Filter::Binary(binaries) => {
                let current = filters.iter().find(|f| matches!(f, Filter::Binary(_)));
                if let Some(Filter::Binary(current)) = current {
                    let mut binaries = binaries;
                    binaries.extend_from_slice(current);
                    binaries.dedup();
                    filters.retain(|f| !matches!(f, Filter::Binary(_)));
                    filters.push(Filter::Binary(binaries));
                } else {
                    filters.push(filter);
                }
            }
            _ => {
                filters.retain(|f| *f != filter);
                filters.push(filter);
            }
        }
        trace!("filters: {:?}", filters);
    }

    pub fn add_filter(mut self, filter: Filter) -> Self {
        Self::push(&mut self.filters, filter);
        self
    }

    pub fn append(&mut self, filter: Filter) {
        Self::push(&mut self.filters, filter);
    }

    pub fn remove_filter(mut self, filter: Filter) -> Self {
        self.filters.retain(|f| *f != filter);
        self
    }

    pub fn clear(mut self) -> Self {
        self.filters.clear();
        self
    }
}

pub struct Collection {
    filters: Vec<Filter>,
}

impl Collection {
    pub fn apply(&self, scores: &[Score]) -> Vec<Score> {
        trace!("applying filters: {:?}", self.filters);
        let mut results = scores.to_vec();
        for filter in self.filters.iter() {
            results = match filter {
                Filter::Top(_) | Filter::Bottom(_) => Self::apply_top(&results, filter),
                Filter::UniquePlayers => Self::apply_unique_players(&results),
                Filter::UniqueBinaries => Self::apply_unique_binaries(&results),
                Filter::UniqueLanguages => Self::apply_unique_languages(&results),
                Filter::Player(players) => Self::apply_players(&results, players),
                Filter::Binary(binaries) => Self::apply_binaries(&results, binaries),
                Filter::Language(language) => Self::apply_languages(&results, language),
                Filter::Sort(column) => Self::apply_sort(&results, column),
            }
        }
        results
    }

    fn apply_top(scores: &[Score], top: &Filter) -> Vec<Score> {
        trace!("applying top filter: {:?}", top);
        let mut scores = scores.to_vec();
        match top {
            Filter::Top(n) => scores.truncate(*n),
            Filter::Bottom(n) => {
                scores.reverse();
                scores.truncate(*n);
                scores.reverse();
            }
            _ => unreachable!(),
        }
        scores
    }

    fn apply_unique_players(scores: &[Score]) -> Vec<Score> {
        trace!("applying unique players filter");
        let mut scores = scores.to_vec();
        let mut seen = Vec::new();
        scores.retain(|s| {
            if seen.contains(&s.name) {
                false
            } else {
                seen.push(s.name.clone());
                true
            }
        });
        scores
    }

    fn apply_unique_binaries(scores: &[Score]) -> Vec<Score> {
        trace!("applying unique binaries filter");
        let mut scores = scores.to_vec();
        let mut seen = Vec::new();
        scores.retain(|s| {
            if seen.contains(&s.command) {
                false
            } else {
                seen.push(s.command.clone());
                true
            }
        });
        scores
    }

    fn apply_unique_languages(scores: &[Score]) -> Vec<Score> {
        trace!("applying unique language filter");
        let mut scores = scores.to_vec();
        let mut seen = Vec::new();
        scores.retain(|s| {
            if seen.contains(&s.language) {
                false
            } else {
                seen.push(s.language.clone());
                true
            }
        });
        scores
    }

    fn apply_players(scores: &[Score], player: &[String]) -> Vec<Score> {
        trace!("applying players filter: {:?}", player);
        let mut scores = scores.to_vec();
        scores.retain(|s| player.contains(&s.name));
        scores
    }

    fn apply_binaries(scores: &[Score], binaries: &[String]) -> Vec<Score> {
        trace!("applying binaries filter: {:?}", binaries);
        let mut scores = scores.to_vec();
        scores.retain(|s| binaries.contains(&s.command));
        scores
    }

    fn apply_languages(scores: &[Score], languages: &[String]) -> Vec<Score> {
        trace!("applying language filter: {:?}", languages);
        let mut scores = scores.to_vec();
        scores.retain(|s| languages.contains(&s.language));
        scores
    }

    fn apply_sort(scores: &[Score], column: &SortColumn) -> Vec<Score> {
        trace!("applying sort filter: {:?}", column);
        let mut scores = scores.to_vec();
        match column {
            SortColumn::PlayerName => scores.sort_by(|a, b| a.name.cmp(&b.name)),
            SortColumn::Binary => scores.sort_by(|a, b| a.command.cmp(&b.command)),
            SortColumn::Language => scores.sort_by(|a, b| a.language.cmp(&b.language)),
            SortColumn::Time => scores.sort_by(|a, b| a.time_ns.partial_cmp(&b.time_ns).unwrap()),
        }
        scores
    }
}
