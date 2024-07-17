use std::collections::HashMap;

pub fn show_people_in_dept(txt_arr: &mut Vec<String>) -> HashMap<&str, Vec<&str>> {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for i in txt_arr {
        let mut v: Vec<&str> = i.split_whitespace().collect();
        let mut people_in_dept: Option<&Vec<&str>> = map.get(v[3]);
        match people_in_dept {
            Some(people_in_dept) => { 
                let mut p: Vec<&str> = people_in_dept.to_vec();
                p.push(&v[1]);
                p.sort();
                map.insert(&v[3], p);
            },
            None => {
                let mut p_dept: Vec<&str> = Vec::new();
                p_dept.push(&v[1]);
                map.insert(&v[3], p_dept);
            }
        }
    }
    map
}
 