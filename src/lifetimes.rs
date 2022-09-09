use std::cell::Ref;

struct ReferenceStruct<'a> {
    pub string1: &'a str,
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn lifetimes() {
    let string1 = String::from("ahdskd");
    {
        let string2: &'static str = "odsjdk";

        let new_string = longest(&string1, &string2);
        println!("the longest string is : {}", string2)
    }
}

pub fn lifetime_struct() {
    {
        let some_string = String::from("helllo");
        let reference_struct = ReferenceStruct {
            string1: &some_string,
        };
    }
}
