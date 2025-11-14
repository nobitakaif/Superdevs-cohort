use std::fmt::Error;
#[derive(Debug)]
struct User{
    id : u32,
    age : u32
}

trait Serialize{
    fn serialize(&self)-> Vec<u8>;
}
trait Deserialize{ 
    fn deserialize(v: &[u8])->Result<User,Error>; // this needs to be static function that's why we're not using self agrs
}

impl Serialize for User{
    fn serialize(&self)-> Vec<u8> {
        let mut v = vec![];
        // v.extend_from_slice() this will appending the new number like [12,34,5]-> [12,34,5,6,7,7,5]
        // to_be_bytes will convert in number 0->255 into 8 bits
        print!("printing the vec extend_from_slice{:?}",v.extend_from_slice(&self.id.to_be_bytes()));
        print!("printing the vec extend_from_slice{:?}",v.extend_from_slice(&self.age.to_be_bytes()));
        println!("printing whole vector {:?}", v);

        return v;
    }
}

impl Deserialize for User{
    fn deserialize(v: &[u8])->Result<Self,Error> {
        
        if v.len() < 8 {
            println!("vector length -> {}", v.len());
            return Err(Error);
        }
        // we use diff way to do this, right now doing very ugly way 
        let id = u32::from_be_bytes([v[0],v[1],v[2],v[3]]); // we know array's length is 8 and 0->3 for id and 4->7 for age;
        let age = u32::from_be_bytes([v[4],v[5],v[6],v[7]]);
        
        return Ok(User{
            id,
            age,
        });
    }
}


fn main() {
    let user = User{
        id : 255,
        age : 6
    };
    let arrary = user.serialize();
    // array will looks like [0, 0, 0, 255, 0, 0, 0, 6]

    let vec_to_User = User::deserialize(&arrary).unwrap();
    print!("deseializing -> {:?}",vec_to_User);
    // println!("{}",user);
}
