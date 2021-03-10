use std::process::Command;

fn main() {
    let output = Command::new("cat")
	.args(&["Main.rs"])
	.output()
	.expect("failed to exec");
    let out = String::from_utf8(output.stdout).expect("couldnt parse utf8");
    print!("{}", out)
}
