#[macro_export]
macro_rules! blocking_request {
    ($pool_name:ident, $blocked_func:path [ $($args:expr),+]) => {
        {
            let $pool_name = $pool_name.get().expect("Expected connections");
            let res =
                unpack_result(web::block(move || $blocked_func($($args),+)).await);

            match res {
                Ok(val) => {HttpResponse::Ok().body(serde_json::to_string(&val).unwrap())},
                Err(e) => {e},
            }
        }
    }
}
