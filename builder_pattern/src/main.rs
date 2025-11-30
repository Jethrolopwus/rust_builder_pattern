// builder pattern
#[derive(Debug, Default, Clone)]

struct Customer {
    name: String,
    username: String,
    membership: Membershiptype,
    gender: char,
    Country: String,
    age: u8,
}


#[derive(Debug, Clone)]

enum Membershiptype {
    new,
    casual,
    loyal,
}

impl Default for Membershiptype{
    fn default() -> Self {
        Membershiptype::new
    }
}

impl Customer {
    fn new(name: String) -> CustomerBuilder {
        CustomerBuilder { 
            name: name,

            ..Default::default()

            // username: None,
            // membership: None,
            // gender: None,
            // country: None,
            // age: None,
        }
    }
    // fn new(name: String) -> Self {
    //     Customer {
    //         name: name,
    //         ..Default::default()
    //     }
    // }
    // fn new_2( name: String, username: String) -> Self {
    //     Customer { name: name, username: username, ..Default::default() }
    // }
    // fn new_3(name: String, username: String, membership:Membershiptype) -> Self {
    //     Customer { name: name, username: username, membership, ..Default::default()}
    // }
}

#[derive(Default)]
struct CustomerBuilder {
    name: String,
    username: Option<String>,
    membership: Option<Membershiptype>,
    gender: Option<char>,
    country: Option<String>,
    age: Option<u8>

}
impl CustomerBuilder {
    fn username(&mut self, username: String) -> &mut Self {
        self.username = Some(username);
        self
    }

    fn membership(&mut self, membership: Membershiptype ) -> &mut Self {
        self.membership = Some(membership);
        self
    }
    fn gender(&mut self, gender: char) -> &mut Self {
        self.gender = Some(gender);
        self
    }

    fn country(&mut self, country: String) -> &mut Self {
        self.country = Some(country);
        self
    }

    fn age(&mut self, age:u8) -> &mut Self {
        self.age = Some(age);
        self
    }
    fn builder(&mut self) -> Customer {
        Customer {
            name: self.name.clone(),
            username: self.username.clone().unwrap_or_default(),
            membership: self.membership.clone().unwrap_or_default(),
            Country: self.country.clone().unwrap_or_default(),
            gender: self.gender.unwrap_or_default(),
            age: self.age.unwrap_or_default(),
         }
    }
}

fn main() {


    // let new_user = Customer::new("Jethro".to_string());
    // let usr_with_login = Customer::new_2("Harry".to_string(), "Harry123".to_string());
    // let user_with_membership = Customer::new_3("Jethro".to_string(),  "Jeth123".to_string(), Membershiptype::loyal);

    let new_user = Customer::new(String::from("Jethro"))
    .builder();
    let user_with_login = Customer::new(String::from("James"))
    .username(String::from("James123"))
    .builder();
    let user_with_membership = Customer::new(String::from("Harry"))
    .username(String::from("Harry2000"))
    .membership(Membershiptype::loyal)
    .builder();

}
