mod token;

use jsonwebtoken::errors::Error;
use jsonwebtoken::TokenData;
use warp::Filter;

#[tokio::main]
async fn main() {
    
    let login = warp::path!("login" / String / String)
        .map(|uname: String, _pass: String|
            // TODO: Validate against username and password
            token::make_token(b"secret_key", uname.as_str(), "user")
        ) 
        //.and_then(|res: Result<String, Error>| async move {
        .and_then(|res| async move {
            match res {
                Ok(t) => Ok(format!("{}", t)),
                Err(_) => Err(warp::reject::not_found())
            }
        });

    let validate = warp::path!("validate" / String)
        .map(|token: String|
            token::decode_token(token, b"secret_key")
        )
        .and_then(|res: Result<TokenData<token::Claims>, Error>| async move {
            match res {
                Ok(c) => Ok(format!("{:?}", c.claims)),
                Err(_) => Err(warp::reject::not_found())
            }
        });

    let routes = warp::get().and(login.or(validate));

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
