use base64::prelude::*;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
struct User {
    name: String,
    gender: Gender,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
enum Gender {
    Male,
    Female,
}

fn main() {
    let user = User {
        name: "Ritzier".to_string(),
        gender: Gender::Male,
    };

    // Serialize to bytes and encode to base64
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&user).unwrap();
    let base64_string = BASE64_STANDARD.encode(&bytes);
    println!("Base64 encoded: {}", base64_string);

    // Decode base64 back to bytes
    let decoded_bytes = BASE64_STANDARD.decode(base64_string).unwrap();

    // Deserialize from decoded bytes
    let archived = rkyv::access::<ArchivedUser, rkyv::rancor::Error>(&decoded_bytes).unwrap();
    println!("Archived user: {:?}", archived);

    let deserialized: User = rkyv::deserialize::<User, rkyv::rancor::Error>(archived).unwrap();
    hello(&deserialized);
}

fn hello(user: &User) {
    println!("Hi, {}. You are a {:?}", user.name, user.gender);
}
