//! valid_palindrome
//! Time: O(?) | Space: O(?)

pub fn valid_palindrome_v1(s:String)->bool {
    let filter_s= s.chars()
        .filter(|s| s.is_alphanumeric())
        .flat_map(|s| s.to_lowercase());
    filter_s.clone().eq(filter_s.rev())
}
pub fn valid_palindrome_v2(s:String)->bool {
    let chars: Vec<char> = s.chars().collect();
    if chars.is_empty(){
        return true;
    }
    let (mut left, mut right) = (0,chars.len() - 1);
    while left<right {
        if !chars[left].is_alphanumeric(){
            left+=1;
            continue;
        }
        if !chars[right].is_alphanumeric(){
            right-=1;
            continue;
        }
        if chars[left].to_ascii_lowercase() != chars[right].to_ascii_lowercase()
        {
            return false;
        }
        left+=1;
        right -=1;
    }
    true
}
pub fn is_palindrome_simple(s: String) -> bool {
    let filtered: String = s.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    let reversed: String = filtered.chars().rev().collect();
    
    filtered == reversed
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_palindrome_v1_basic() {
        assert!(valid_palindrome_v1("A man, a plan, a canal: Panama".to_string()));
        assert!(!valid_palindrome_v1("race a car".to_string()));
        assert!(valid_palindrome_v1("".to_string()));
        assert!(valid_palindrome_v1(" ".to_string()));
    }

    #[test]
    fn valid_palindrome_v2_basic() {
        assert!(valid_palindrome_v2("A man, a plan, a canal: Panama".to_string()));
        assert!(!valid_palindrome_v2("race a car".to_string()));
        assert!(valid_palindrome_v2("".to_string()));
        assert!(valid_palindrome_v2(" ".to_string()));
    }

    #[test]
    fn is_palindrome_simple_basic() {
        assert!(is_palindrome_simple("A man, a plan, a canal: Panama".to_string()));
        assert!(!is_palindrome_simple("race a car".to_string()));
        assert!(is_palindrome_simple("".to_string()));
        assert!(is_palindrome_simple(" ".to_string()));
    }

    #[test]
    fn single_alphanumeric_char() {
        assert!(valid_palindrome_v2("a".to_string()));
        assert!(valid_palindrome_v2("0".to_string()));
    }

    #[test]
    fn three_versions_agree() {
        let cases = vec![
            "A man, a plan, a canal: Panama",
            "race a car",
            "0P",
            ".,",
            "ab_a",
        ];
        for case in cases {
            let r1 = valid_palindrome_v1(case.to_string());
            let r2 = valid_palindrome_v2(case.to_string());
            let r3 = is_palindrome_simple(case.to_string());
            assert_eq!(r1, r2, "v1 vs v2 mismatch on: {}", case);
            assert_eq!(r2, r3, "v2 vs v3 mismatch on: {}", case);
        }
    }
}