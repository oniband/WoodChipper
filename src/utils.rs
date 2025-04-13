pub mod utils {

    ///Returns an array containing:
    ///[0] -> The path to the binary.
    ///[1] -> Directory that the dissasembled binary should be placed.
    ///
    /// #Example
    ///
    ///  ```no_run
    /// use std::env;
    /// use std::fs;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let paths: [String; 2] = parse_args(env::args().collect());
    ///
    ///     let file_handle = fs::File::open(paths[0].clone());
    /// }
    ///
    ///  ```
    pub fn parse_args(args: Vec<String>) -> [String; 2] {
        if args.len() == 1 {
            println!(
                "No Argument(s) given. Usage: woodchipper <Path To File> <Optional Output Directory>"
            );
            panic!("Quitting...");
        } else if args.len() > 3 {
            println!(
                "Too many Argument(s) given. Usage: woodchipper <Path To File> <Optional Output Directory>"
            );
            panic!("Quitting...");
        }
        if args.len() == 3 {
            return [args[1].clone(), args[2].to_owned()];
        }
        [args[1].clone(), "./".to_owned()]
    }
}
