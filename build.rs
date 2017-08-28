use std::process::Command;

fn main() {
    println!("Building image tileset from font and tiles.");
    let command = "convert assets/terminal16x16_gs_ro.png assets/graphics16x16.png -append assets/master16x16_ro.png";
    println!("Running Imagemagick command {}", command);
    let status = if cfg!(target_os = "windows") {
        println!("Platform: Windows");
        Command::new("cmd")
            .args(&["/C", command])
            .status()
    } else {
        println!("Platform: Unix");
        Command::new("sh")
            .args(&["-c", command])
            .status()
    };

    if status.is_err() {
        println!("Command exited with failure");
        println!("Please make sure you have ImageMagick installed.");
        println!("You can find ImageMagick at https://www.imagemagick.org/script/binary-releases.php");
    } else {
        println!("Building is a success!");
    }
}
