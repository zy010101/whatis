use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Rule {
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub keywords: Option<Vec<String>>,
    pub rule: String,
    pub exception: Option<Vec<String>>,
    pub validation: Option<String>,
    pub keyword_max_distance: u64,
    pub tags: Option<Vec<String>>,
    pub example: Option<Vec<String>>
}


#[cfg(test)]
mod tests{
    use std::{fs::File, io::BufReader};
    use super::Rule;

    #[test]
    fn read_yaml() {
        let file = File::open("demo.yaml").unwrap();
        let reader = BufReader::new(file);
        let rule: Rule = serde_yaml::from_reader(reader).unwrap();

        assert_eq!(rule.description, "Chinese Mainland ID number，excluding Hong Kong, Macao, and Taiwan".to_string());
        assert_eq!(rule.name, "Chinese Mainland ID number".to_string());
        assert_eq!(rule.example.unwrap(), vec![r###"{"id": "372522197003231000"}"###, "身份证号码：372522197003231000"]);
        assert_eq!(rule.exception.unwrap(), vec!["His employee ID is 372522197003231000"]);
        assert_eq!(rule.keyword_max_distance, 30);
        assert_eq!(rule.keywords.unwrap(), vec!["(?i)id", "身份证", "证件", "证明"]);
        assert_eq!(rule.name, "Chinese Mainland ID number");
        assert_eq!(rule.rule, r"(?i)(1[1-5]|2[1-3]|3[1-7]|4[1-6]|5[0-4]|6[1-5]|71|8[1-3])(0[1-9]|[1-6][0-9]|70)\d{2}(19|20)\d{2}(0[1-9]|1[0-2])(0[1-9]|[1-2][0-9]|3[0-1])\d{3}[0-9x]");
        assert_eq!(rule.tags.unwrap(), vec!["identity", "personal information"]);
        assert_eq!(rule.validation.unwrap(), "chinese_mainland_id_number".to_string());
    }
}

