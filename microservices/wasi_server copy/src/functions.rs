use hyper::{ Body, Response };

/* ############## HTTP RESPONSE HEADER ############## */
fn get_attributs_header(page_id: u16) -> Vec<(String, String)> { {
    let mut attributs: Vec<(String, String)> = Vec::new();

    if page_id == 1 || page_id == 6  || page_id == 13{
        attributs.push(("Cache-Control".to_string(), "no-cache, must-revalidate, no-transform, no-store".to_string()));
    }
    
    if page_id == 0 || page_id == 1 || page_id == 2 || page_id == 3 || page_id == 4 || page_id == 5 || page_id == 6 || page_id == 7 || page_id == 8 || page_id == 9 || page_id == 11 || page_id == 12 || page_id == 13{
        attributs.push(("Referrer-Policy".to_string(), "no-referrer".to_string()));
        attributs.push(("Strict-Transport-Security".to_string(), "max-age=31536000; includeSubDomains".to_string()));
        attributs.push(("X-Content-Type-Options".to_string(), "nosniff".to_string()));
        attributs.push(("X-XSS-Protection".to_string(), "1; mode=block".to_string()));
    }

    if page_id == 0 || page_id == 1 || page_id == 3 || page_id == 7 || page_id == 8 || page_id == 11{
        attributs.push(("Content-Security-Policy".to_string(), "frame-src 'self'; frame-ancestors 'self'; object-src 'none';".to_string()));
    }

    if page_id == 0 || page_id == 1 || page_id == 3 || page_id == 5 || page_id == 6 || page_id == 7 || page_id == 8 || page_id == 11{
        attributs.push(("Content-Type".to_string(), "text/html; charset=utf-8".to_string()));
        attributs.push(("X-Robots-Tag".to_string(), "none".to_string()));
    }

    if page_id == 0 || page_id == 1 || page_id == 3 || page_id == 4 || page_id == 7 || page_id == 8 || page_id == 11 || page_id == 13{
        attributs.push(("X-Frame-Options".to_string(), "SAMEORIGIN".to_string()));
    }

    if page_id == 3 || page_id == 7 || page_id == 8 || page_id == 11{
        attributs.push(("Content-Language".to_string(), "en".to_string()));
    }

    if page_id == 3 || page_id == 4 || page_id == 5{
        attributs.push(("Cache-Control".to_string(), "no-cache".to_string()));
    }

    if page_id == 5 || page_id == 6{
        attributs.push(("Content-Security-Policy".to_string(), "frame-src 'self'; object-src 'none'".to_string()));
    }

    if page_id == 7 || page_id == 8 || page_id == 9{
        attributs.push(("Cache-Control".to_string(), "no-store, must-revalidate, max-age=0".to_string()));
    }

    if page_id == 4 || page_id == 13{
        attributs.push(("Content-Type".to_string(), "application/json".to_string()));
    }

    return attributs;
} }

pub fn set_attributs_header(_response: &mut Response<Body>, page_id: u16) {
    let attributs_headers = get_attributs_header(page_id);

    for atrb in attributs_headers.iter() {
        let header_name = atrb.0.clone();
        let header_value = atrb.1.clone().parse().unwrap();

        let header_name_static: &'static str = Box::leak(header_name.into_boxed_str());
        _response.headers_mut().insert(header_name_static, header_value);
    }
}

pub fn get_page_id( uri: String ) -> u16{
    match uri.as_str() {
        "/" => 1,
        "/admin/" => 2,
        "/admin/master/console/" => 3,
        "/admin/master/console/config" => 4,
        "/realms/master/protocol/openid-connect/3p-cookies/step1.html" => 5,
        "/realms/master/protocol/openid-connect/login-status-iframe.html" => 5,
        "/realms/master/protocol/openid-connect/3p-cookies/step2.html" => 6,
        "/realms/master/protocol/openid-connect/auth" => 7,
        "/realms/master/login-actions/authenticate" => 8,
        "/err/login/exec" => 8,
        "/err/login" => 8,
        "/err/login/tabid" => 9,
        "/err/auth/ccmethod_invalid" => 10,
        "/err/auth/ccmethod_nfound" => 10,
        "/err/auth/cc_invalid" => 10,
        "/err/auth/cc_nfound" => 10,
        "/err/bad_client" => 11,
        "/err/auth/rm_invalid" => 12,
        "/err/auth/rt_invalid" => 12,
        "/err/auth/rt_nfound" => 12,
        "/err/auth/scope_invalid" => 12,
        "/realms/master/.well-known/openid-configuration" => 13,
        _ => 0
    }
}