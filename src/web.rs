use warp;
use warp::Filter;

pub async fn run() {
    let ping = warp::path("ping").and(warp::path::end()).map(|| "pong");
    let routes = ping.or(filters::oauth());

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await
}

mod handlers {
    use warp::http::Uri;
    use warp::{reject, Rejection, Reply};

    #[derive(Debug)]
    pub enum Error {
        ServerConfigError,
    }

    impl reject::Reject for Error {}

    pub async fn oauth() -> Result<impl Reply, Rejection> {
        //let redirect_uri = format!(
        //    "https://auth.atlassian.com/authorize? \
        //        audience=api.atlassian.com& \
        //        client_id={client_id}& \
        //        scope=read:jira-work%20read:jira-user& \
        //        redirect_uri={callback_url}& \
        //        state=login&response_type=code&prompt=consent",
        //    client_id = "",
        //);

        let redirect_uri = format!("http://naver.com?callbackurl={client_id}", client_id = "");
        match redirect_uri.parse::<Uri>() {
            Ok(u) => Ok(warp::redirect(u)),
            Err(_) => Err(warp::reject::custom(Error::ServerConfigError)),
        }
    }
}

mod filters {
    use super::handlers;

    use serde::Serialize;
    use warp::http::StatusCode;
    use warp::{Filter, Rejection, Reply};

    #[derive(Serialize)]
    struct ErrorMessage<'a> {
        code: u16,
        message: &'a str,
    }

    async fn error_handler(err: warp::Rejection) -> Result<impl Reply, Rejection> {
        if let Some(err) = err.find::<handlers::Error>() {
            let (code, msg) = match err {
                handlers::Error::ServerConfigError => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Server configuration error",
                ),
            };
            let json = warp::reply::json(&ErrorMessage {
                code: code.as_u16(),
                message: msg,
            });

            Ok(warp::reply::with_status(json, code))
        } else {
            Err(err)
        }
    }

    pub fn oauth() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("oauth")
            .and(warp::path::end())
            .and(warp::get())
            .and_then(handlers::oauth)
            .recover(error_handler)
    }
}
