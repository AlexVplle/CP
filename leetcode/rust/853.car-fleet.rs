impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        let mut zip: Vec<(i32, i32)> = position
            .into_iter()
            .map(|pos: i32| target - pos)
            .zip(speed)
            .collect();
        zip.sort_unstable();
        let times: Vec<f32> = zip
            .into_iter()
            .map(|(pos, speed): (i32, i32)| pos as f32 / speed as f32)
            .collect();
        times.into_iter().fold(0.0, |max, time| {
            if max < time {
                result += 1;
            }
            max.max(time)
        });
        result
    }
}
