use std::fmt;
 // use std::io;
 /*
    use std::fmt::Result;
    use std::io::Result as IoResult;
    fn f1() -> Result {
        Ok(())
    }
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

use rand::Rng;
use std::collections::HashMap;
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
 // use std::cmp::Ordering;
use std::cmp::Ordering::*; // load all of public's items from Ordering
 // use std::{cmp::Ordering, io};
use std::io::{self, Write};
fn main() {

}
