
    if let Some(cookie) = req.cookie("token") {
        let token = cookie.value().to_string();
    } else {
        let cookie = Cookie::build("token", generate_random_token())
            .path("/")
            .http_only(true)
            .finish();
        HttpResponse::Ok().cookie(cookie).finish()
    };

