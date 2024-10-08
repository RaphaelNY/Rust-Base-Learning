fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    drop(c);// c.drop(); <- error: explicit use of destructor method
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

