fn main() {
    let x = String::from("hello world");
    let (x, len) = take_ownership_and_return(x);
    // along with variable shadowing
    println!("{} {}", x, len);

    println!("len: {}", borrow_ownership(&x));

    let mut arr = [1,2,3,4,5]; 
    arr[2] = 33;

}

fn take_ownership_and_return(s: String) -> (String, usize) {
    let l = s.len();
    (s, l)
}

fn borrow_ownership(s: &String) -> usize {
    s.len()
}

// fn dangle() -> &String {
//     &String::from("hello")
//     /*
//     this function's return type contains a borrowed value,
//     but there is no value for it to be borrowed from
//     try using lifetime specifiers
//     */
// }



/*
this chapter is about memory management

and rusts ownership model
x is a pointer to a heap allocated memory


1. each value in rust has a owner that is its variable
2. there can only be one owner at a time
3. when owner goes out of scope, value will be dropped

a way is to return the ownership of the string
but a better way would be to take in the mutable pointer


slices can be thought of as a lens view into the structure
whether its a slice of String or vec or array of int

ques: is array expandable like vector ?
ans: i dont think so

so if we have multiple slices , it is like having multiple immuatable
borrows, and we cant have a mutable borrow

*/

