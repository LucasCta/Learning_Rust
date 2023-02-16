/* impl std::fmt::Debug for PathedIoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PathedIoError")
            .field("path", &self.path)
            .field("inner", &self.inner)
            .finish()
    }
} */

//#[derive(Debug)]

/* struct PathedIoError {
    path: String,
    inner: std::io::Error,
}

impl std::fmt::Debug for PathedIoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "for file {:?}: {}", self.path, self.inner)
    }
}

fn read_input() -> Result<String, PathedIoError> {
    let path = "src/input2.txt";
    match std::fs::read_to_string(path){ 
        Ok(s) => Ok(s),
        Err(e) => Err(PathedIoError {
            path: path.into(),
            inner: e,
        }),
    }
} */

/*fn read_input() -> Result<String, std::io::Error> {
    fs_err::read_to_string("src/input2.txt")
}*/

fn read_input() -> color_eyre::Result<String> {
    /* let input = std::fs::read_to_string("src/input.txt")?;
    Ok(input) */
    //std::fs::read_to_string("src/input.txt").map_err(|e| e.into())
    std::fs::read_to_string("src/input.txt").map_err(From::from)
}

fn main() {
    
    let input = read_input().unwrap();
    print!("{input}");

}
