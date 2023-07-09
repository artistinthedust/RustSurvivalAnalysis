use rand::Rng;
use rand::distributions::{Distribution, Uniform};
use rand_distr::{Normal, NormalError};
use postgres::{Client, NoTls, Error};
use std::collections::HashMap;


struct Author {
    _id: i32,
    name: String,
    country: String
}

fn main() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
    println!("Integer: {}", rng.gen_range(0..10));
    println!("Float: {}", rng.gen_range(0.0..10.0));

    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }

    generate();
    connectDB();
}

fn generate() -> Result<(), NormalError> {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(2.0, 3.0)?;
    let v = normal.sample(&mut rng);
    println!("{} is from a N(2, 9) distribution", v);
    Ok(())
}


fn connectDB() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:1234@localhost/library", NoTls)?;
    client.batch_execute("
    CREATE TABLE IF NOT EXISTS author (
        id              SERIAL PRIMARY KEY,
        name            VARCHAR NOT NULL,
        country         VARCHAR NOT NULL
        )
")?;

println!("connect data");
client.batch_execute("
    CREATE TABLE IF NOT EXISTS book  (
        id              SERIAL PRIMARY KEY,
        title           VARCHAR NOT NULL,
        author_id       INTEGER NOT NULL REFERENCES author
        )
")?;

    
let mut authors = HashMap::new();
authors.insert(String::from("Chinua Achebe"), "Nigeria");
authors.insert(String::from("Rabindranath Tagore"), "India");
authors.insert(String::from("Anita Nair"), "India");

for (key, value) in &authors {
    let author = Author {
        _id: 0,
        name: key.to_string(),
        country: value.to_string()
    };

    client.execute(
            "INSERT INTO author (name, country) VALUES ($1, $2)",
            &[&author.name, &author.country],
    )?;
}

for row in client.query("SELECT id, name, country FROM author", &[])? {
    let author = Author {
        _id: row.get(0),
        name: row.get(1),
        country: row.get(2),
    };
    println!("Author {} is from {}", author.name, author.country);
}
    Ok(())
}