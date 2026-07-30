//! group_anagrams
//! Time: O(?) | Space: O(?)
use std::collections::HashMap;
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for s in strs {
        // Chuẩn hóa từ: chuyển thành Vec<char>, sắp xếp, rồi gom lại thành String làm key
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_unstable();
        let sorted_key: String = chars.into_iter().collect();

        // Gom nhóm dùng entry API và move trực tiếp chuỗi s vào Vec
        map.entry(sorted_key).or_insert(Vec::new()).push(s);
        }

    // Lấy tất cả các giá trị (bỏ key) để trả về mảng 2 chiều
    map.into_values().collect()
}
#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn group_anagrams_test() {
        let input = vec![
            String::from("eat"),
            String::from("tea"),
            String::from("tan"),
            String::from("ate"),
            String::from("nat"),
            String::from("bat"),
        ];

        let mut result = group_anagrams(input);

        // Sắp xếp các phần tử bên trong nhóm và giữa các nhóm để kiểm tra độc lập với thứ tự của HashMap
        for group in &mut result {
            group.sort();
        }
        result.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut expected = vec![
            vec![String::from("bat")],
            vec![String::from("ate"), String::from("eat"), String::from("tea")],
            vec![String::from("nat"), String::from("tan")],
        ];

        for group in &mut expected {
            group.sort();
        }
        expected.sort_by(|a, b| a[0].cmp(&b[0]));

        assert_eq!(result, expected);
    }
}
