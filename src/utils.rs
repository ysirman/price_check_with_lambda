use reqwest::Client;
use dotenv;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub fn get_client() -> Client {
    let client = Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();

    client
}

use reqwest::StatusCode;
use scraper::{Html, Selector};
#[tokio::main]
pub async fn get_price_and_rates() -> String {
    let default_expect_rate = 90;
    let expect_rate: i32 = match dotenv::var("EXPECT_RATE") {
        Ok(val) => match val.parse::<i32>() { Ok(expect_rate) => expect_rate,
                                              Err(_) => default_expect_rate }
        Err(_) => default_expect_rate };
    let client = get_client();
    let url: String = dotenv::var("TARGET_URL").unwrap();
    let result = client.get(&url).send().await.unwrap();
    let raw_html = match result.status() {
        StatusCode::OK => result.text().await.unwrap(),
        _ => panic!("HTML取得失敗"),
    };
    let document = Html::parse_document(&raw_html);

    // スクレイピング対象
    // <td class="giftList_cell giftList_cell-facevalue giftList_cell-label giftList_cell-labelBold" data-label="額面：">
    //     <span id="facevalue_2642950">¥ 10,000</span>
    //     <span class="giftList_rate giftList_spText">93.0 %</span>
    // </td>
    let wrapper_selector = Selector::parse("td.giftList_cell.giftList_cell-facevalue.giftList_cell-label.giftList_cell-labelBold").unwrap();
    let price_and_rate_select = Selector::parse("span").unwrap();
    let mut price_and_rate_vec:Vec<String> = Vec::new();
    price_and_rate_vec.push(url);
    for (i, element) in document.select(&wrapper_selector).enumerate() {
        let price_and_rate:Vec<String> = element.select(&price_and_rate_select).map(|inner_element| {
            inner_element.inner_html().to_string()
        }).collect();
        let rate: i32 = price_and_rate[1][..2].parse().unwrap();

        // println!("割引率：{}", price_and_rate[1]);
        if i == 0 && rate > expect_rate {
            panic!("[ERROR] 割高のため処理中止（{}）", price_and_rate.join(" / "));
        }

        price_and_rate_vec.push(price_and_rate.join(" / "));
    };
    // println!("{}", price_and_rate_vec.join("\n"));
    price_and_rate_vec.join("<br>")
}

use sendgrid::SGClient;
use sendgrid::{Destination, Mail};
pub fn send_email(mail_body:&str) {
    let api_key: String = dotenv::var("SENDGRID_API_KEY").unwrap();
    let sg = SGClient::new(api_key);
    let mut x_smtpapi = String::new();
    x_smtpapi.push_str(r#"{"unique_args":{"test":7}}"#);
    let to_email: &str = &dotenv::var("EMAIL_SEND_TO").unwrap();
    let from_email: &str = &dotenv::var("EMAIL_SEND_FROM").unwrap();

    let mail_info = Mail::new()
        .add_to(Destination {
            address: to_email,
            name: "通知先",
        })
        .add_from(from_email)
        .add_subject("価格レポート")
        .add_html(mail_body)
        .add_from_name("価格通知マン")
        .add_header("x-cool".to_string(), "indeed")
        .add_x_smtpapi(&x_smtpapi);
    match sg.send(mail_info) {
        Err(err) => println!("Error: {}", err),
        Ok(response_body) => println!("Response: {:?}", response_body),
    };
}
