use realm::base::*;

pub struct FTDPlay;

impl realm::serve::Middleware for FTDPlay {
    fn handle(&self, ctx: &realm::Context) -> Result<realm::Response> {
        observer::create_context("middleware");
        let path = ctx.url.path();

        observer::observe_string("path", path);
        if ctx.method != http::Method::GET {
            observer::observe_string("method", ctx.method.as_str());
        }

        if let Some(d) = ctx
            .get_header(http::header::HOST)
            .and_then(|v| v.to_str().ok())
        {
            if d.to_lowercase() == "fifthtry.com" {
                observer::log("is naked fifthtry.com, redirecting to www");
                let res = Ok(realm::Response::Http(
                    http::Response::builder()
                        .header("Location", format!("https://www.fifthtry.com{}", path))
                        .status(http::StatusCode::PERMANENT_REDIRECT)
                        .body(vec![])?,
                ));
                observer::end_context();
                return res;
            }
        }

        if path.starts_with("/static/") {
            let res = realm::serve_static::serve_static(ctx);
            observer::end_context();
            return res;
        }
        play::handle(ctx)
    }
}

pub fn main() {
    let logger = observer::backends::logger::Logger::builder()
        .with_path(play::env::OBSERVER_LOG_PATH.as_str())
        .with_stdout()
        .build();

    observer::builder(logger).init();
    encrypted_id::init(realm::env::REALM_SECRET.as_ref());

    let mut _guard = None;
    if *play::env::SENTRY_ENABLED {
        _guard = Some(sentry::init((
           "https://f8acce81f1874ada94b85cac26d78eab@o517176.ingest.sentry.io/5994173",
            sentry::ClientOptions {
                release: sentry::release_name!(),
                send_default_pii: true,
                debug: true,
                ..Default::default()
            }
            .add_integration(ftweb::sentry_realm::ObserverIntegration::new()),
        )));
    }

    let realm_service = realm::serve::RealmService::new(FTDPlay {});
    realm_service.http();
}

