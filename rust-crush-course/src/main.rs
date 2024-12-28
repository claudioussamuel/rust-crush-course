#![deny(clippy::all)]

use std::collections::HashMap;
use std::io::Read;

fn main() {
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: "John".to_string(),
        age: 22,
    };

    let personal_data: (i32, &str) = (22, "John");
    let (age, name) = personal_data;

    let message = say_hello_world("Hello world");

    // In line function

    let say_hello_world = |name: &str| -> String { String::from("Hello,") };

    //vectors are arrays in other library
    let mut value: [&str; 2] = ["foo", "bar"];
    let foo = &value[0];

    let v = vec![1, 2, 3];

    //hashmap  are maps in other language
    let mut val: HashMap<&str, &str> = HashMap::new();
    val.insert("foo", "Bar");

    //Optional this is like null safety in dart
    let optional = Some(10);
    let optional_name: Option<&str> = None;

    // Error handling in Rust
    let err: Result<&str, Box<dyn std::error::Error>> = Ok("Hello");
    let erro: Result<&str, ()> = Ok("Hello");

    let user_name = get_user_name().expect("failed to get user name");
    println!("Hello, {}!", user_name);

    let is_ok = get_user_name().is_ok();

    let perrson = Persoon {
        first_name: "Claudious".to_string(),
        last_name: "Opoku".to_string(),
        age: 23,
    };

    println!("{:?}", perrson);

    //
    let  version = read_version("010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000");
    println!("Version: {}", version);
}

fn read_version(transaction_hex: &str) -> u32 {
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let mut bytes_slice = transaction_bytes.as_slice();
    let mut buffer = [0; 4];
    bytes_slice.read_exact(&mut buffer).unwrap();
    u32::from_le_bytes(buffer)
}

// Traits (They are shared functionality)
#[derive(Debug)]
struct Persoon {
    first_name: String,
    last_name: String,
    age: u8,
}

trait HasFullName {
    fn full_name(&self) -> String;
}

trait CanRun {
    fn run(&self) -> u32;
}

fn print_details<T>(value: &T)
where
    T: HasFullName + CanRun,
{
    println!("{}", value.full_name());
    value.run();
}

impl HasFullName for Persoon {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

// Life time operator
fn get_full_name() -> &'static str {
    "John Doe"
}

fn get_random_number<'l>(a: &'l str, b: &'l str) -> &'l str {
    b
}

struct Person<'a> {
    name: &'a str,
}

impl<'a> Person<'a> {
    fn first_char_of_first_name(&self) -> &str {
        &self.name[0..1]
    }
}

// Error handling in Rust
fn get_user_name() -> Result<String, ()> {
    Err(())
    // Ok("John")
}

fn say_hello_world(name: &str) -> String {
    String::from("Hello, world")
}

fn get_values() -> (String, String, i32) {
    ("Hello".to_string(), "World".to_string(), 30)
}
