use rocket::get;
use rocket::http::Cookie;
use rocket::http::CookieJar;
use rocket::http::SameSite;
use rocket::http::Status;
use rocket::request;
use rocket::request::FromRequest;
use rocket::response::Redirect;
use rocket::Request;
use rocket_oauth2::OAuth2;
use rocket_oauth2::TokenResponse;
use tracing::debug;

pub struct Kanidm {
    pub name: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Kanidm {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Kanidm, ()> {
        let cookies = request
            .guard::<&CookieJar<'_>>()
            .await
            .expect("request cookies");
        if let Some(cookie) = cookies.get("scope") {
            return request::Outcome::Success(Kanidm {
                name: cookie.value().to_string(),
            });
        }

        request::Outcome::Forward(Status::Unauthorized)
    }
}

#[get("/login/kanidm")]
pub fn kanidm_login(oauth2: OAuth2<Kanidm>, cookies: &CookieJar<'_>) -> Redirect {
    oauth2.get_redirect(cookies, &["profile"]).unwrap()
}

#[get("/auth/kanidm")]
pub fn kanidm_callback(token: TokenResponse<Kanidm>, cookies: &CookieJar<'_>) -> Redirect {
    debug!("Scope: {:#?}", token.scope());
    cookies.add(Cookie::build(("scope", format!("{:#?}", token.scope()))).build());
    cookies.add(Cookie::build(("token", token.access_token().to_string())).build());
    Redirect::to("/")
}
