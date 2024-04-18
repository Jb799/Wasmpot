use regex::Regex;

#[derive(Debug, Clone)]
pub enum Method {
    GET,
    POST,
}

#[derive(Debug, Clone)]
pub enum ParamValue {
    Regex(Regex),
    Exact(String),
}

#[derive(Debug, Clone)]
pub struct Param {
    pub id: u32,
    pub name: String,
    pub value: ParamValue,
    pub flag: u8,
    pub redirect: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub url: String,
    pub method: Method,
    pub query_params: Vec<Param>,
    pub data_params: Vec<Param>,
}

pub fn get_rules() -> Vec<Rule> {
    vec![
        Rule {
            url: String::from("https://example.com/api/data"),
            method: Method::GET,
            query_params: vec![
                Param {
                    id: 1,
                    name: String::from("user_id"),
                    value: ParamValue::Regex(Regex::new(r"^\d+$").unwrap()),
                    flag: 5,
                    redirect: Some(String::from("https://example.com/error")),
                },
                Param {
                    id: 2,
                    name: String::from("token"),
                    value: ParamValue::Exact(String::from("xyz")),
                    flag: 3,
                    redirect: None,
                },
            ],
            data_params: vec![],
        },
        Rule {
            url: String::from("https://example.com/api/submit"),
            method: Method::POST,
            query_params: vec![],
            data_params: vec![
                Param {
                    id: 3,
                    name: String::from("content"),
                    value: ParamValue::Regex(Regex::new(r"^[\w\s]+$").unwrap()),
                    flag: 7,
                    redirect: Some(String::from("https://example.com/content_error")),
                },
            ],
        },
    ]
}
