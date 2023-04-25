use crate::generators::name::country::Country;


pub fn generate_person_id(name: &String, country: &Country, age: &u16) -> String{
    let mut id = String::new();

    id.push_str(name.replace(" ", "").as_str());
    id.push_str(country.to_string().as_str());
    id.push_str(age.to_string().as_str());

    return id.to_lowercase();
}