extern "C" {
    fn NimMain();
    fn nimFunc();
}

#[no_mangle]
pub unsafe extern "C" fn rustFunc() {
    println!("rustFunc() called");
}

fn main() 
{
    unsafe {
        NimMain();
        nimFunc();
    }
}
