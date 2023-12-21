use actix_web::{HttpResponseBuilder};

/* ############## HTTP RESPONSE HEADER ############## */
fn get_attributs_header(_http_code: i32) -> Vec<(String, String)> { {
    let mut attributs: Vec<(String, String)> = Vec::new();

    attributs.push(("Referrer-Policy".to_string(), "no-referrer".to_string()));
    attributs.push(("Strict-Transport-Security".to_string(), "max-age=31536000; includeSubDomains".to_string()));
    attributs.push(("X-Content-Type-Options".to_string(), "nosniff".to_string()));
    attributs.push(("X-XSS-Protection".to_string(), "1; mode=block".to_string()));

    if _http_code == 200 || _http_code == 400 {
        attributs.push(("Content-Security-Policy".to_string(), "frame-src 'self'; frame-ancestors 'self'; object-src 'none';".to_string()));
        attributs.push(("X-Frame-Options".to_string(), "SAMEORIGIN".to_string()));
        attributs.push(("X-Robots-Tag".to_string(), "none".to_string()));
        attributs.push(("Content-Type".to_string(), "text/html; charset=utf-8".to_string()));

        if _http_code == 200 {
            attributs.push(("Cache-Control".to_string(), "no-cache, must-revalidate, no-transform, no-store".to_string()));
        }
    }

    return attributs;
} }

pub fn set_attributs_header(_http_object: &mut HttpResponseBuilder, _http_code: i32) {
    let attributs_headers = get_attributs_header(_http_code);
    
    for atrb in attributs_headers.iter() {
        _http_object.insert_header((
            atrb.0.clone(), // Cloner la chaîne pour transférer la propriété
            atrb.1.clone(),
        ));
    }
}