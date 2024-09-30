impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        gain.into_iter()
            .fold((0, 0), |(max_altitude, current_altitude), change| {
                let new_atitude: i32 = current_altitude + change;
                (max_altitude.max(new_atitude), new_atitude)
            })
            .0
    }
}
