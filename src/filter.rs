use crate::score::Score;
use log::trace;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortColumn {
    PlayerName,
    Binary,
    Time,
}

#[derive(Debug, Clone)]
pub enum Filter {
    Top(usize),
    Bottom(usize),
    Player(Vec<String>),
    Binary(Vec<String>),
    UniquePlayers,
    UniqueBinaries,
    Sort(SortColumn),
}

impl PartialEq for Filter {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Filter::Player(a), Filter::Player(b)) => a == b,
            (Filter::Binary(a), Filter::Binary(b)) => a == b,
            (Filter::UniquePlayers, Filter::UniquePlayers) => true,
            (Filter::UniqueBinaries, Filter::UniqueBinaries) => true,
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

    pub fn add_filter(mut self, filter: Filter) -> Self {
        trace!("adding filter: {:?}", filter);
        match filter.clone() {
            Filter::Player(players) => {
                let current = self.filters.iter().find(|f| matches!(f, Filter::Player(_)));
                if let Some(Filter::Player(current)) = current {
                    let mut players = players;
                    players.extend_from_slice(current);
                    players.dedup();
                    self.filters.retain(|f| !matches!(f, Filter::Player(_)));
                    self.filters.push(Filter::Player(players));
                } else {
                    self.filters.push(filter);
                }
            }
            Filter::Binary(binaries) => {
                let current = self.filters.iter().find(|f| matches!(f, Filter::Binary(_)));
                if let Some(Filter::Binary(current)) = current {
                    let mut binaries = binaries;
                    binaries.extend_from_slice(current);
                    binaries.dedup();
                    self.filters.retain(|f| !matches!(f, Filter::Binary(_)));
                    self.filters.push(Filter::Binary(binaries));
                } else {
                    self.filters.push(filter);
                }
            }
            _ => {
                self.filters.retain(|f| *f != filter);
                self.filters.push(filter);
            }
        }
        trace!("filters: {:?}", self.filters);
        self
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
                Filter::Player(players) => Self::apply_players(&results, players),
                Filter::Binary(binaries) => Self::apply_binaries(&results, binaries),
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
        scores.dedup_by(|a, b| a.name == b.name);
        scores
    }

    fn apply_unique_binaries(scores: &[Score]) -> Vec<Score> {
        trace!("applying unique binaries filter");
        let mut scores = scores.to_vec();
        scores.dedup_by(|a, b| a.name == b.name);
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

    fn apply_sort(scores: &[Score], column: &SortColumn) -> Vec<Score> {
        trace!("applying sort filter: {:?}", column);
        let mut scores = scores.to_vec();
        match column {
            SortColumn::PlayerName => scores.sort_by(|a, b| a.name.cmp(&b.name)),
            SortColumn::Binary => scores.sort_by(|a, b| a.command.cmp(&b.command)),
            SortColumn::Time => scores.sort_by(|a, b| a.time_ns.partial_cmp(&b.time_ns).unwrap()),
        }
        scores
    }
}