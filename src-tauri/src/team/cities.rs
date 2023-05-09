use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct City {
    pub city_name: String,
    pub longitude: f32,
    pub latitude: f32,
}

impl City {
    pub fn gen() -> Vec<City> {
        let cities_csv = "city_name,longitude,latitude
Boston,-71.0597700,42.3584300
Philadelphia,-75.1637900,39.9523300
New York City,-74.0059700,40.7142700
Toronto,-79.4163000,43.7001100
Chicago,-87.6500500,41.8500300
Cleveland,-81.6954100,41.4995000
Detroit,-83.0457500,42.3314300
Indianapolis,-86.1580400,39.7683800";

        let mut cities = vec![];

        let mut reader = csv::Reader::from_reader(cities_csv.as_bytes());

        for city in reader.deserialize() {
            let city: City = match city {
                Ok(val) => val,
                Err(err) => panic!("Unable to Read CSV:: {}", err),
            };

            cities.push(city);
        }

        return cities;
    }
}
