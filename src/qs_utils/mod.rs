use serde::Deserialize;

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
