use std::collections::BTreeMap;

impl Solution {
    pub fn car_pooling(mut trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        trips.sort_unstable_by(|a, b| a[1].cmp(&b[1]));
        let mut offs = BTreeMap::new();
        let mut count = 0;
        for trip in trips.iter() {
            let cur_start = trip[1];
            while let Some((&leave, &c)) = offs.iter().next() {
                if leave <= cur_start {
                    count -= c;
                    offs.remove(&leave);
                } else {
                    break;
                }
            }
            count += trip[0];
            if count > capacity {
                return false;
            }
            *offs.entry(trip[2]).or_insert(0) += trip[0];
        }
        true
    }
}
