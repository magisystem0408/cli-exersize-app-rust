use std::io;
use cli_exer_app::services;

fn main() {
    let mut service_type: String = String::new();
    println!("実行したい内容を入力してください。(0: register, 1: search)");
    io::stdin().read_line(&mut service_type).unwrap();

    let service_type: u8 = service_type.trim().parse().expect("please input number");
    services::validate::InputValidator::validate_service_type(service_type);

    if service_type == 0 {
        println!("登録サービス");
    } else if service_type == 1 {
        println!("集計サービス");
    }
}
