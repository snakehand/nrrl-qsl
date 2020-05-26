use csv::StringRecord;
use serde::Deserialize;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct Member {
    pub customer_number: String,
    pub supplier_number: String,
    pub name: String,
    pub organisation_number: String,
    pub telephone_number: String,
    pub mobile_number: String,
    pub fax_number: String,
    pub email: String,
    pub postal_adress1: String,
    pub postal_adress2: String,
    pub postal_code: String,
    pub city: String,
    pub country: String,
    pub business_adress1: String,
    pub business_adress2: String,
    pub business_postal_code: String,
    pub business_city: String,
    pub business_country: String,
    pub category_number1: String,
    pub category_name1: String,
    pub category_number2: String,
    pub category_name2: String,
    pub category_number3: String,
    pub category_name3: String,
}

pub fn read_members(filename: &OsStr) -> Result<Vec<Member>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = csv::ReaderBuilder::new().delimiter(b';').from_reader(file);
    rdr.set_headers(StringRecord::from(vec![
        "customer_number",
        "supplier_number",
        "name",
        "organisation_number",
        "telephone_number",
        "mobile_number",
        "fax_number",
        "email",
        "postal_adress1",
        "postal_adress2",
        "postal_code",
        "city",
        "country",
        "business_adress1",
        "business_adress2",
        "business_postal_code",
        "business_city",
        "business_country",
        "category_number1",
        "category_name1",
        "category_number2",
        "category_name2",
        "category_number3",
        "category_name3",
    ]));
    let mut contacts = Vec::new();
    for result in rdr.deserialize() {
        let record: Member = result?;
        contacts.push(record)
    }
    Ok(contacts)
}
