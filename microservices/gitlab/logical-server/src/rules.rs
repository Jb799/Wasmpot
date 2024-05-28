use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub enum ResponseModification {
    Replace { placeholder: String, param_name: String },
    Sanitize { param_name: String, regex: Regex },
}

#[derive(Debug, Clone)]
pub struct ResponseRule {
    pub url: String,
    pub method: MethodType,
    pub modifications: Vec<ResponseModification>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MethodType {
    GET,
    POST,
    ANY,
}

#[derive(Debug, Clone)]
pub enum ParamValue {
    Regex(Regex),
    Exact(String),
    AlwaysFail,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub id: u32,
    pub name: String,
    pub value: ParamValue,
    pub flag: u8,
    pub redirect: Option<String>,
    pub required: bool,
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub url_pattern: Regex,
    pub method: MethodType,
    pub query_params: Vec<Param>,
    pub redirect: Option<String>,
    pub flag: Option<u8>,
    pub id: Option<u32>,
}

pub fn get_rules() -> Vec<Rule> {
    vec![
        Rule {
            url_pattern: Regex::new(r"/users/sign_in").unwrap(),
            method: MethodType::POST,
            query_params: vec![
                Param {
                    id: 1,
                    name: String::from("user[login]"),
                    value: ParamValue::AlwaysFail,
                    flag: 10,
                    redirect: Some(String::from("/users/sign_in_err")),
                    required: true,
                },
                Param {
                    id: 1,
                    name: String::from("user[password]"),
                    value: ParamValue::AlwaysFail,
                    flag: 10,
                    redirect: Some(String::from("/users/sign_in_err")),
                    required: true,
                },
            ],
            redirect: None,
            flag: None,
            id: None,
        },
        Rule {
            url_pattern: Regex::new(r"/users/password").unwrap(),
            method: MethodType::POST,
            query_params: vec![],
            redirect: Some(String::from("/users/sign_in/reset_pass")),
            flag: Some(10),
            id: Some(2),
        },
    ]
}

lazy_static! {
    static ref XSS_FILTER_REGEX: Regex = Regex::new(r#"[<>&"']"#).unwrap();
}

pub fn get_response_rules() -> Vec<ResponseRule> {
    vec![]
}