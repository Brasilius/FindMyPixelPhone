use phonenumber::PhoneNumber;
use phonenumber::country;
use phonenumber::Carrier;
use phonenumber::parse;

fn main() {
    println!("Greetings user!");
    println!("This is a simple program that will read out the location of a phonenumber when inputted.");
    println!("Please input a phone number in the following format example: +111-222-3333");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let phone_number = PhoneNumber::parse("&input", country::US).unwrap().expect("Failed to connect to API");
    let location = phone_number.location().unwrap().expect("failed to get information from phone number");
    println!("The location of the phone number is: {}", location);


}
