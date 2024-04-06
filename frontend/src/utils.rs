use web_sys;

pub fn base_url() -> String {
    let window = web_sys::window().unwrap();
    let location = window.location();
    location.origin().unwrap()
}
