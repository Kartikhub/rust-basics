
pub fn make_pl(s: &String) -> String {
        let first_char = s.chars().next().unwrap();
        let vowels = String::from("aeiou");
        if vowels.contains(first_char) {
            let ans = format!("{s}-hay");
            return ans;
        }
        let mut st = &s[1..];
        format!("{st}-{first_char}ay")
}