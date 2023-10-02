# Overview

Simple lib to interact with the code challenge scoreboard DB.

# Test

Setup a MySQL test DB called code_challenge.
Located on port 3306

```bash
export DB_PASS=<password>
cargo test
```

# Usage

Your main interface is the scoreboard module and you should ise it thusly

```rust
use scoreboard::ScoreBoard;


let mute scoreboard = Scoreboard::new(scores);

let scoreboard = ScoreBoard::new(scores);
let filters = Builder::new()
    .add_filter(Filter::Player(vec!["foo".to_string()]))
    .add_filters(Filter::Sort())
    .build();
let scores = scoreboard.display(Some(&filters));



```