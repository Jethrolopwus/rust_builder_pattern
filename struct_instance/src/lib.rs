#[derive(Debug, Default)]

pub struct  Student {
    pub id: u8,
    pub name: String,
    pub age: u8,
}


impl Student {
    pub fn new (std_name: String) -> Result<Self, String> {
        if std_name.chars().all(|x| matches!(x, 'a'..='z')) {
            Ok( Self { id: 0, name: std_name, age: 20 })
        } else {
            Err(String::from("Invalid name: only lowercase letters are allowed"))
        }
       
    }
}

// impl Default for Student {
//     fn default() -> Self {
//         Student {
//             id: 0,
//             name: String::from("default_name that is unknown"),
//             age: 18,
//         }
//     }
// }