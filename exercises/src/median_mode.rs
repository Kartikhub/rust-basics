use std::collections::HashMap;

pub fn median(v: &Vec<i32>) -> i32 {
    let l = v.len();
    v[l/2]
}

pub fn mode(v: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for i in v {
        let c = map.entry(i).or_insert(0);
        *c += 1;
    }

    let mut max = 0;
    let mut max_key = v[0];
    for (&key, &value) in &map {
        if value > max {
            max = value;
            max_key = *key;
        }
    }
    max_key
}