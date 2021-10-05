use realm::base::*;

pub fn handle(ctx: &realm::Context) -> Result<realm::Response> {
    let conn = pg::connection();

    let in_ = In::from(&conn, ctx);

    if *ftdb::env::SENTRY_ENABLED {
        let with_pii = crate::sentry_realm::with_pii();
        let sentry_req = crate::sentry_realm::sentry_request_from_http(ctx, with_pii);

        sentry::configure_scope(|scope| {
            scope.set_transaction(Some(ctx.url.path()));
            scope.set_user(Some(sentry::User {
                id: in_.user_id(),
                ..Default::default()
            }));
            scope.add_event_processor(Box::new(move |mut event| {
                event.request = Some(sentry_req.clone());
                Some(event)
            }))
        });
    }

    let result = play::forward::magic(&in_);
    if let Err(e) = &result {
        observer::observe_string("request_body", &ctx.get_body());
        #[allow(deprecated)]
            sentry::integrations::failure::capture_error(e);
    };

    realm::end_context(&in_, result, |_, m| crate::http404(&cin, m))
}
