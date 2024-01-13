struct MyStruct {
    a: u8,
    b: u8,
    c: u8
}

impl MyStruct {
    fn add(&self) -> u8 {
        self.a + self.b + self.c
    }

    fn mul(&self) -> u8 {
        self.a * self.b
    }
}

struct MusicPlayer {
    file_path: String,
}

impl MusicPlayer {
    fn play(&self) {
        println!("file path is {}", self.file_path);
        println!("Playing music...");
    }
    fn pause(&self) {
        println!("Paused music...");
    }
    fn stop(&self) {
        println!("Stopped music...");
    }
}

fn main() {
    println!("Welcome to Music Player");
    
    let my_struct = MyStruct{a: 10, b: 10, c: 10};
    
    println!("Value of a is {}", my_struct.a);
    println!("Addition is {}", my_struct.add());
    println!("Multition is {}", my_struct.mul());

    let music_player = MusicPlayer {file_path: String::from("path_to_music")};
    music_player.play();
    music_player.pause();
    music_player.stop();
}
