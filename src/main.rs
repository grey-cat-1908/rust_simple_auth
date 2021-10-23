use std::collections::HashMap;
use std::io;

mod methods;

fn main() {
    let real_data = methods::UserAuthInfo {
        login: "aboba".to_string(),
        password: "aboba_telecom232".to_string()
    };

    println!("[INFO] Здравствуйте! Если Вы хотите войти, то отправьте первой строкой свой логин, а второй свой пароль!");

    let mut user_input = HashMap::new();

    let mut user_login: String = "".to_string();
    let mut user_password: String = "".to_string();

    io::stdin().read_line(&mut user_login).unwrap().to_string();
    io::stdin().read_line(&mut user_password).unwrap().to_string();

    user_input.insert("login".to_string(), user_login.replace("\n", "").to_string());
    user_input.insert("password".to_string(), user_password.replace("\n", "").to_string());

    if methods::login(real_data, user_input) {
        println!("[INFO] Добро Пожаловать! Хотите выйти из аккаунта?");

        let mut user_choice: String = "".to_string();

        io::stdin().read_line(&mut user_choice).unwrap().to_string();

        if user_choice.replace("\n", "").to_string().to_lowercase() == "да" {
            main()
        }
    }
    else {
        println!("[INFO] Аккаунта, соответствующего введенным данным - не существует.")
    }
}
