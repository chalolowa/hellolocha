fn main() {
    println!("Hello, locha!");

    let name = String::from("Locha");
    let mut age = 30;
    age = 25;
    let is_male = true;
    let info = true;

    if (info == true) {
        println!("Name: {}", name);
        println!("Age: {}", age);
        println!("Is Male: {}", is_male)
    } else if (info == false) {
        println!("False ifo");
    } else {
        println!("Go away!!");
    }
}
