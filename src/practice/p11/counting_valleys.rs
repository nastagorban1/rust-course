///  https://www.hackerrank.com/challenges/counting-valleys/problem
fn countingValleys1(steps: i32, path: &str) -> i32 {
    #[derive(Clone)]
    struct State {
        level: i32,
        count: i32,
    }

    impl State {
        fn new(level: i32, count: i32) -> State {
            State { level, count }
        }
        fn fresh() -> Self {
            Self::new(0, 0)
        }
        fn delta(prev_level: i32, curr_level: i32) -> i32 {
            if curr_level == 0 && prev_level < 0 {
                1
            } else {
                0
            }
        }
        fn up(&self) -> Self {
            let curr_level = self.level + 1;
            let delta_count = Self::delta(self.level, curr_level);
            State::new(curr_level, self.count + delta_count)
        }
        fn down(&self) -> Self {
            let curr_level = self.level - 1;
            let delta_count = Self::delta(self.level, curr_level);
            State::new(curr_level, self.count + delta_count)
        }
        fn process(&self, c: char) -> State {
            match c {
                'U' => self.up(),
                'D' => self.down(),
                _ => self.clone(), // or without clone, by moving. remove & in `up` and `down`
            }
        }
    }

    path.chars()
        .fold(State::fresh(), |st, c| st.process(c))
        .count
}

fn countingValleys2(steps: i32, path: &str) -> i32 {
    let mut prev_level = 0;
    let mut count = 0;
    for c in path.chars() {
        let curr_level = match c {
            'U' => prev_level + 1,
            'D' => prev_level - 1,
            _ => prev_level,
        };
        if curr_level == 0 && prev_level < 0 {
            count += 1;
        }
        prev_level = curr_level;
    }
    count
}
