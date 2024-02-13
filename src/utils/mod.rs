use serde::Deserialize;
use std::cmp::Ordering;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Macros
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
macro_rules! gen_handler_fn {
    ($name:literal,  $key_field:literal, $swapi_type:ty, $swapi_data:expr, $path:expr) => {
        ::paste::paste! {
            #[get($path)]
            async fn [<get_ $name>]<'a> (
                config: web::Data<Config<'a>>,
                web::Query(qs): web::Query<QueryString>,
            ) -> impl Responder {
                let mut swapi_resp: SwapiResponse<$swapi_type> = serde_json::from_str($swapi_data).unwrap();
                let qsv = get_query_string_vals(&qs, Some($key_field));
                let key = format!("{}.{}", qsv.sort_by, qsv.sort_dir);
                let sort_fn: Sorter<$swapi_type> = *config.[<$name>].get(&*key).unwrap();

                logger($path, &qsv);
                swapi_resp.results.sort_by(sort_fn);
                web::Json(swapi_resp.results)
            }
        }
    };
}

macro_rules! gen_str_sort_fn {
    ($key:expr, $swapi_type:ty) => {
        ::paste::paste! {
            pub fn [<sort_by_ $key>](dir: &str) -> fn(&$swapi_type, &$swapi_type) -> Ordering {
                if dir == "desc" {
                    |v1: &$swapi_type, v2: &$swapi_type| compare_strs(&v1.$key, &v2.$key).reverse()
                } else {
                    |v1: &$swapi_type, v2: &$swapi_type| compare_strs(&v1.$key, &v2.$key)
                }
            }
        }
    }
}

macro_rules! gen_num_str_sort_fn {
    ($key:expr, $swapi_type:ty) => {
        ::paste::paste! {
            pub fn [<sort_by_ $key>](dir: &str) -> fn(&$swapi_type, &$swapi_type) -> Ordering {
                if dir == "desc" {
                    |v1: &$swapi_type, v2: &$swapi_type| {
                        compare_strs_as_f64s(&v1.$key, &v2.$key).reverse()
                    }
                } else {
                    |v1: &$swapi_type, v2: &$swapi_type| {
                        compare_strs_as_f64s(&v1.$key, &v2.$key)
                    }
                }
            }
        }
    }
}

macro_rules! gen_num_sort_fn {
    ($key:expr, $swapi_type:ty) => {
        ::paste::paste! {
            pub fn [<sort_by_ $key>](dir: &str) -> fn(&$swapi_type, &$swapi_type) -> Ordering {
                if dir == "desc" {
                    |v1: &$swapi_type, v2: &$swapi_type| compare_nums(v1.$key, v2.$key).reverse()
                } else {
                    |v1: &$swapi_type, v2: &$swapi_type| compare_nums(v1.$key, v2.$key)
                }
            }
        }
    }
}

pub(crate) use {
    gen_handler_fn,
    gen_str_sort_fn,
    gen_num_str_sort_fn,
    gen_num_sort_fn,
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Query string utilities
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Deserialize)]
pub struct QueryString {
    #[serde(rename = "sortBy")]
    pub sort_by: Option<String>,

    #[serde(rename = "sortDir")]
    pub sort_dir: Option<String>,
}

pub struct QueryStringVals {
    pub sort_by: String,
    pub sort_dir: String,
}

pub fn get_query_string_vals(qs: &QueryString, default_key: Option<&str>) -> QueryStringVals {
    // If query string arguments for sorting are absent, the default is to sort by name ascending
    QueryStringVals {
        sort_by: match qs.sort_by.as_ref() {
            None => match default_key {
                Some(key_name) => String::from(key_name),
                None => String::from("name"),
            },
            Some(prop_name) => prop_name.clone(),
        },

        sort_dir: match qs.sort_dir.as_ref() {
            None => String::from("asc"),
            Some(dir) => dir.clone(),
        },
    }
}

pub fn logger(swapi_type_name: &str, qsv: &QueryStringVals) {
    println!(
        "Sorting {} by {}, {}ending",
        swapi_type_name, qsv.sort_by, qsv.sort_dir
    );
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Sort utilities
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub type Sorter<T> = fn(s1: &T, s2: &T) -> std::cmp::Ordering;

fn str_to_f64(s: &str) -> f64 {
    // Check for embedded thousands separator
    s.replace(&[',', ' '], "").parse::<f64>().unwrap_or_else(|_| std::f64::INFINITY)
}

pub fn compare_nums<N>(n1: N, n2: N) -> Ordering
where
    N: PartialOrd,
{
    if n1 < n2 {
        Ordering::Less
    } else if n1 > n2 {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

pub fn compare_strs(s1: &str, s2: &str) -> Ordering {
    s1.to_ascii_lowercase().cmp(&s2.to_ascii_lowercase())
}

pub fn compare_strs_as_f64s(s1: &str, s2: &str) -> Ordering {
    compare_nums(str_to_f64(s1), str_to_f64(s2))
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
mod sort_tests;
