pub fn determinist(u: String, v: String) -> (String, String) {
    if u < v {
        (u.clone(), v.clone())
    } else {
        (v.clone(), u.clone())
    }
}
