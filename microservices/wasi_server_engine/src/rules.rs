use hyper::{Body, Request, Response, StatusCode};
use hyper::header::HeaderValue;
use std::collections::HashMap;
use regex::Regex;

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
            ],
            data_params: vec![],
        },
        Rule {
            url: String::from("/api/submit"),
            method: MethodType::POST,
            query_params: vec![],
            data_params: vec![
                Param {
                    id: 3,
                    name: String::from("content"),
                    value: ParamValue::Regex(Regex::new(r"^[\w\s]+$").unwrap()),
                    flag: 7,
                    redirect: Some(String::from("/content_error")),
                    required: false,
                },
            ],
        },
    ]
}
