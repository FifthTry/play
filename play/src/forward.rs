use crate::prelude::*;
use http::method::Method;

pub fn magic(cin: &Cache) -> realm::Result {
    let mut input = cin.in_.ctx.input()?;
    let pm = cin.in_.ctx.pm();

    match pm {
        t if realm::is_realm_url(t) => match realm::handle(cin.in_, t, &mut input) {
            Ok(t) => Ok(t),
            Err(_) => crate::http404(cin, "page not found"),
        },

        ("/", &Method::GET) => {
            play::route::get(in_)
        }
        _ => unimplemented!("return 404"),
    }
}
