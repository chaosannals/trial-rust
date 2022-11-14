use salvo::prelude::*;

mod index;
mod login;

use index::*;
use login::*;

pub fn new_router() -> Router {
    Router::new()
        .get(index)
        .push(
            Router::new()
            .path("login")
            .get(login)
        ).push(
            Router::with_path("logout/<id>")
            .get(logout)
        )
}