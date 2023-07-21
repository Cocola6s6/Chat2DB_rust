use anyhow::Result;
use std::collections::HashMap;
use tracing::info;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestInit, RequestMode, Response};

pub struct HttpUtils();

impl HttpUtils {
    pub async fn get(url: String, extra_headers: Option<HashMap<String, String>>) -> Result<JsValue> {
        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::Cors);

        let headers = Headers::new().unwrap();
        headers.set("Accept", "application/json");
        headers.set("Content-Type", "application/json");
        for (key, value) in extra_headers.unwrap_or_default().iter() {
            headers.set(key, value);
        }
        opts.headers(&headers);

        opts.body(None);

        let request = Request::new_with_str_and_init(&url, &opts).unwrap();

        let window = web_sys::window()
            .ok_or("no windows exists".to_string())
            .unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .unwrap();

        assert!(resp_value.is_instance_of::<Response>());
        let resp: Response = resp_value.dyn_into().unwrap();
        let resp = JsFuture::from(resp.json().unwrap()).await.unwrap();
        info!("resp: {:?}", resp);

        Ok(resp)
    }

    pub async fn post(
        url: String,
        extra_headers: Option<HashMap<String, String>>,
        body: String,
    ) -> Result<JsValue> {
        let mut opts = RequestInit::new();
        opts.method("POST");
        opts.mode(RequestMode::Cors);

        let headers = Headers::new().unwrap();
        headers.set("Accept", "application/json");
        headers.set("Content-Type", "application/json");
        for (key, value) in extra_headers.unwrap_or_default().iter() {
            headers.set(key, value);
        }
        opts.headers(&headers);

        opts.body(Some(&JsValue::from_str(body.as_str())));

        let request = Request::new_with_str_and_init(&url, &opts).unwrap();

        let window = web_sys::window()
            .ok_or("no windows exists".to_string())
            .unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .unwrap();

        assert!(resp_value.is_instance_of::<Response>());
        let resp: Response = resp_value.dyn_into().unwrap();
        let resp = JsFuture::from(resp.json().unwrap()).await.unwrap();
        info!("resp: {:?}", resp);

        Ok(resp)
    }
}
