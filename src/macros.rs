macro_rules! gen_handler_fn {
    ($name:literal,  $key_field:literal, $swapi_type:ty, $path:expr) => {
        ::paste::paste! {
            #[get($path)]
            async fn [<handle_ $name>](
                app_cache: web::Data<SwapiCache>,
                web::Query(qs): web::Query<QueryString>,
            ) -> impl Responder {
                let mut cached_data = app_cache.[<$name>].lock().unwrap();

                if cached_data.len() == 0 {
                    let url = &format!("{API_URL}{}/", $name);
                    if let Some(new_data) = [<fetch_ $name>](url).await.unwrap() {
                        *cached_data = new_data.results;
                    };
                }

                let sort_by = &*qs.sort_by.unwrap_or($key_field.to_string());
                let sort_order = &*qs.sort_order.unwrap_or("asc".to_string());

                cached_data.sort_by($swapi_type::sort_by(sort_by, sort_order));

                HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .body(serde_json::to_string(&*cached_data).unwrap())
            }
        }
    };
}

pub(crate) use {
    gen_handler_fn
};