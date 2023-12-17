use std::fmt;
use std::io;
 /*
    use std::fmt::Result;
    use std::io::Result as IoResult;
    fn f2() -> IoResult<()> {
        Ok(())
    }
 */
fn  f1() -> fmt::Result {
    Ok(())
}
fn f2() -> io::Result<()> {
    Ok(())
}

fn main() {
}
