
pub fn solution(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() { return false; }

    let mut char_set_1 = [0; 26];
    let mut char_set_2 = [0; 26];

    for c in s1.chars() {
        let pos = (c as usize) - 97;
        char_set_1[pos]+=1;
    }

    for c in s2.chars() {
        let pos = (c as usize) - 97;
        char_set_2[pos]+=1;
    }

    let mut pos = 0;
    while pos < 25 {
        if char_set_1[pos] != char_set_2[pos] { return false;}
        pos+=1;
    }

    return true;
}

pub fn exec_anagram() {
    let s1 = "earth";
    let s2 = "heart";

    let result = solution(s1, s2);
    println!("{}", result)
}