use sessionless::hex::IntoHex;
use sessionless::Sessionless;
use reqwest;
use crate::requests::WelcomeResponse;
use crate::utils::Color;

pub fn register(color: Color) -> (Sessionless, WelcomeResponse) {
    let sessionless = Sessionless::new();

    let base_url = color.get_url();
    let placement = color.get_signature_placement();

    let public_key = sessionless.public_key().to_hex();
    let entered_text = "Foo";
    let timestamp = "right now";

    let message = format!(r#"{{"pubKey":"{0}","enteredText":"{1}","timestamp":"{2}"}}"#, public_key, entered_text, timestamp);   
    println!("Signing {0}", message); 
    let signature = sessionless.sign(message.as_bytes()).into_hex();

    let client = reqwest::blocking::Client::new();
    let url = format!("{}/register", base_url);
    println!("{}", url);
    let mut post = client.post(url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json");

    if placement == "payload" {
        let payload = format!(r#"{{"pubKey":"{0}","enteredText":"{1}","timestamp":"{2}","signature":"{3}"}}"#, public_key, entered_text, timestamp, signature);
        post = post.body(payload)
    } else {
        post = post.header("signature", signature)
        .body(message);
    }
    let result = post.send(); 
    let response = match result {
        Ok(resp) => resp,
        Err(_) => panic!("Something went awry!")
    };
    let json_result = response.json::<WelcomeResponse>();
    let welcome_response: WelcomeResponse = match json_result {
        Ok(wr) => wr,
        Err(_) => panic!("Error serializing JSON")
    };
//    let welcome_response: WelcomeResponse = result.expect("response should be json").json::<WelcomeResponse>();
    return (sessionless, welcome_response);
}
