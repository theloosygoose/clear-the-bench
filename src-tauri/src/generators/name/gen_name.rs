use super::country::Country;
use super::names::*;

pub fn gen_name() -> (String, Country) {
    let mut name = String::new();
    
    let birthplace:Country = rand::random();

    let rand_name = match birthplace {
       Country::Spain => (FirstNameAfrican::generate_random().to_string(), LastNameFrench::generate_random().to_string()), 
       Country::France => (FirstNameFrench::generate_random().to_string(), LastNameFrench::generate_random().to_string()), 
       Country::UnitedStates => (FirstNameEnglish::generate_random().to_string(), LastNameEnglish::generate_random().to_string()), 
       Country::Canada => (FirstNameEnglish::generate_random().to_string(), LastNameEnglish::generate_random().to_string()), 
       Country::Jamacia => (FirstNameAfrican::generate_random().to_string(), LastNameEnglish::generate_random().to_string()), 
       Country::PuertoRico => (FirstNameAfrican::generate_random().to_string(), LastNameEnglish::generate_random().to_string()), 
       Country::Serbia => (FirstNameSlav::generate_random().to_string(), LastNameSlav::generate_random().to_string()), 
       Country::Latvia => (FirstNameSlav::generate_random().to_string(), LastNameSlav::generate_random().to_string()), 
       Country::Lithuania=> (FirstNameSlav::generate_random().to_string(), LastNameSlav::generate_random().to_string()), 
       Country::Germany => (FirstNameGerman::generate_random().to_string(), LastNameGerman::generate_random().to_string()), 
       Country::Croatia => (FirstNameSlav::generate_random().to_string().to_string(), LastNameSlav::generate_random().to_string()), 
       Country::Nigeria => (FirstNameAfrican::generate_random().to_string(), LastNameAfrican::generate_random().to_string()), 
       Country::Cameroon => (FirstNameAfrican::generate_random().to_string(), LastNameAfrican::generate_random().to_string()), 
       Country::Senegal => (FirstNameAfrican::generate_random().to_string(), LastNameAfrican::generate_random().to_string()), 
       Country::SouthSudan => (FirstNameAfrican::generate_random().to_string(), LastNameAfrican::generate_random().to_string()), 
       Country::Australia => (FirstNameEnglish::generate_random().to_string(), LastNameEnglish::generate_random().to_string()), 
       Country::Brazil => (FirstNameEnglish::generate_random().to_string(), LastNameEnglish::generate_random().to_string()), 
       Country::Argentina => (FirstNameEnglish::generate_random().to_string(), LastNameEnglish::generate_random().to_string()), 
       Country::UnitedKingdom => (FirstNameEnglish::generate_random().to_string(), LastNameEnglish::generate_random().to_string()), 
       Country::Turkey => (FirstNameSlav::generate_random().to_string(), LastNameSlav::generate_random().to_string()), 
       Country::China => (FirstNameEnglish::generate_random().to_string(), FirstNameEnglish::generate_random().to_string()), 
    };

    name.push_str(&rand_name.0);
    name.push(' ');
    name.push_str(&rand_name.1);
    

    return (name, birthplace)
}
