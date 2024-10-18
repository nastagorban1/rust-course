///  https://www.hackerrank.com/challenges/counting-valleys/problem
fn countingValleys1(steps: i32, path: &str) -> i32 {
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
        fn calc_delta(prev_level: i32, curr_level: i32) -> i32 {
            match curr_level {
                0 if prev_level < 0 => 1,
                _ => 0,
            }
        }
        fn up(&self) -> Self {
            let curr_level = self.level + 1;
            let delta_count = Self::calc_delta(self.level, curr_level);
            State::new(curr_level, self.count + delta_count)
        }
        fn down(&self) -> Self {
            let curr_level = self.level - 1;
            let delta_count = Self::calc_delta(self.level, curr_level);
            State::new(curr_level, self.count + delta_count)
        }
        fn process(&self, c: char) -> State {
            match c {
                'U' => self.up(),
                'D' => self.down(),
                _ => panic!("wrong char"),
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
        let delta = match curr_level {
            0 if prev_level < 0 => 1,
            _ => 0,
        };
        prev_level = curr_level;
        count += delta;
    }
    count
}
