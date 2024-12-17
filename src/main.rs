use oauth2::Kanidm;
use rocket::get;
use rocket::{launch, routes};
use rocket_oauth2::OAuth2;

mod oauth2;

#[get("/")]
fn index(id: Kanidm) -> String {
    format!("Hello, {}!", id.name)
}

#[get("/", rank = 2)]
fn index_2() -> String {
    format!("Not logged in.")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                index,
                index_2,
                oauth2::kanidm_login,
                oauth2::kanidm_callback
            ],
        )
        .attach(OAuth2::<self::oauth2::Kanidm>::fairing("kanidm"))
}
