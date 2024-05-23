use hyper::{Body, Request, Response, StatusCode};
use hyper::header::HeaderValue;
use std::collections::HashMap;
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
    pub url: String,
    pub method: MethodType,
    pub query_params: Vec<Param>,
    pub data_params: Vec<Param>,
}

pub fn get_rules() -> Vec<Rule> {
    vec![
        Rule {
            url: String::from("/realms/master/login-actions/authenticate"),
            method: MethodType::POST,
            query_params: vec![
                Param {
                    id: 1,
                    name: String::from("username"),
                    value: ParamValue::AlwaysFail,
                    flag: 10,
                    redirect: None,
                    required: true,
                },
                Param {
                    id: 2,
                    name: String::from("password"),
                    value: ParamValue::AlwaysFail,
                    flag: 10,
                    redirect: None,
                    required: true,
                },
                Param {
                    id: 3,
                    name: String::from("credentialId"),
                    value: ParamValue::AlwaysFail,
                    flag: 10,
                    redirect: None,
                    required: false,
                },
                Param {
                    id: 6,
                    name: String::from("client_id"),
                    value: ParamValue::Exact("security-admin-console".to_string()),
                    flag: 3,
                    redirect: Some(String::from("/auth/err-client-id2")),
                    required: true,
                },
            ],
            data_params: vec![],
        },
        Rule {
            url: String::from("/realms/master/protocol/openid-connect/auth"),
            method: MethodType::GET,
            query_params: vec![
                Param {
                    id: 4,
                    name: String::from("client_id"),
                    value: ParamValue::Exact("security-admin-console".to_string()),
                    flag: 3,
                    redirect: Some(String::from("/auth/err-client-not-found")),
                    required: true,
                },
                Param {
                    id: 5,
                    name: String::from("redirect_uri"),
                    value: ParamValue::Regex(Regex::new(r".*%2Fadmin%2Fmaster%2Fconsole%2F.*").unwrap()),
                    flag: 10,
                    redirect: Some(String::from("/auth/bad-uri")),
                    required: true,
                },
            ],
            data_params: vec![],
        },
    ]
}

lazy_static! {
    static ref XSS_FILTER_REGEX: Regex = Regex::new(r#"[<>&"']"#).unwrap();
}

pub fn get_response_rules() -> Vec<ResponseRule> {
    vec![
        ResponseRule {
            url: "/realms/master/login-actions/authenticate".to_string(),
            method: MethodType::POST,
            modifications: vec![
                ResponseModification::Sanitize {
                    param_name: "username".to_string(),
                    regex: XSS_FILTER_REGEX.clone(),
                },
                ResponseModification::Replace {
                    placeholder: "{USERNAME}".to_string(),
                    param_name: "username".to_string(),
                },
            ],
        },
    ]
}