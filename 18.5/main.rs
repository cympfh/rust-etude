use std::process::Command;

fn main() {
    match Command::new("rustc").arg("--version").output() {
        Ok(output) => println!("{:?}",output),
        _ => panic!("filed to exe")
    };

    fn test(command: &str)
    {
        let out = Command::new(command).output().unwrap_or_else(|e| panic!("{}", e));
        println!("$ {}", command);
        if out.status.success() {
            println!("  EXIT WITH 0");
            println!("  STDOUT: {}", String::from_utf8_lossy(&out.stdout).trim_right());
            println!("  STDERR: {}", String::from_utf8_lossy(&out.stderr).trim_right());
        } else {
            println!("  EXIT WITH 1");
            println!("  STDERR: {}", String::from_utf8_lossy(&out.stderr).trim_right());
        }
        
    }

    test("pwd");
    test("false");
    test("true");

}
