// use actix_web::{Error, HttpMessage, HttpRequest, web};
// use jsonwebtoken::{decode, DecodingKey, Validation, TokenData};

// pub struct AuthenticatedUser(pub i32);

// pub async fn auth_middleware(req: HttpRequest, payload: web::Payload) -> Result<web::Payload, Error> {
//     let headers = req.headers();
//     if let Some(auth_header) = headers.get("Authorization") {
//         let token = auth_header.to_str().unwrap_or("").replace("Bearer ", "");
//         let key = DecodingKey::from_secret(secret_key.as_ref());
//         let token_data: TokenData<AuthenticatedUser> = decode(&token, &key, &Validation::default())
//             .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;
        
//         req.extensions_mut().insert(token_data.claims.0); // Insert user_id
//         Ok(payload)
//     } else {
//         Err(actix_web::error::ErrorUnauthorized("Missing Authorization header"))
//     }
// }
