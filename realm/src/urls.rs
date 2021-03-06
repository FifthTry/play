pub fn is_realm_url(p: (&str, &http::Method)) -> bool {
    match p {
        ("/storybook/", &http::Method::GET) => true,
        ("/storybook/poll/", &http::Method::GET) => true,
        ("/iframe/", &http::Method::GET) => true,
        ("/favicon.ico", &http::Method::GET) => true,
        ("/robots.txt", &http::Method::GET) => true,
        (t, _) if t.starts_with("/test/") => true,
        (t, &http::Method::GET) if t.starts_with("/static/") => true,
        _ => false,
    }
}

#[observed(with_result, namespace = "realm")]
pub fn handle<UD>(
    in_: &crate::base::In<UD>,
    p: (&str, &http::Method),
    input: &mut crate::request_config::RequestConfig,
) -> Result<crate::Response, failure::Error>
where
    UD: crate::UserData,
{
    match p {
        ("/storybook/", &http::Method::GET) => crate::storybook::get(in_).map_err(Into::into),
        ("/storybook/poll/", &http::Method::GET) => {
            let hash = input.required("hash")?;
            crate::watcher::poll(in_.ctx, hash).map_err(Into::into)
        }
        ("/test/legacy/", &http::Method::GET) => crate::test::get(in_).map_err(Into::into),
        ("/test/self/", &http::Method::GET) => crate::test::realm(in_).map_err(Into::into),
        ("/test/reset-db/", &http::Method::GET) => crate::test::reset_db(in_).map_err(Into::into),
        ("/test/reset-db/", &http::Method::POST) => crate::test::reset_db(in_).map_err(Into::into),
        ("/iframe/", &http::Method::GET) => crate::iframe::get(in_).map_err(Into::into),

        (crate::rr::RECORD_URL, &http::Method::GET) => crate::rr::get(in_).map_err(Into::into),
        (crate::rr::RECORD_URL, &http::Method::POST) => {
            let (id, title, description) = input.required3("id", "title", "description")?;
            let base = input.optional("base")?;
            crate::rr::post(in_, id, title, description, base).map_err(Into::into)
        }
        ("/test/stop-recording/", _) => crate::rr::stop(in_).map_err(Into::into),

        ("/favicon.ico", &http::Method::GET) => {
            crate::serve_static::serve_static(in_.ctx).map_err(Into::into)
        }
        ("/robots.txt", &http::Method::GET) => {
            crate::serve_static::serve_static(in_.ctx).map_err(Into::into)
        }
        (t, &http::Method::GET) if t.starts_with("/static/") => {
            crate::serve_static::serve_static(in_.ctx).map_err(Into::into)
        }
        _ => Err(format_err!("{:?}", p)),
    }
}
