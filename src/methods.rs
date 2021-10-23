use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub(crate) struct UserAuthInfo {
    pub(crate) login: String,
    pub(crate) password: String
}

pub(crate) fn login(real_data: UserAuthInfo, user_input: HashMap<String, String>) -> bool {
    return match Some(&real_data.login) == user_input.get("login") {
        true => {
            match Some(&real_data.password) == user_input.get("password") {
                true => true,
                _ => false
            }
        },
        _ => false
    }
}