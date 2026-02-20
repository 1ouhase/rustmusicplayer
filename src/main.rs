use rustmediaplayer::{MusicPlayer, console_clear, error_helper};

fn main() {
    let mut music_player: MusicPlayer = MusicPlayer::new().expect("falied to create music player");
    let mut current_song:Option<usize> = None;

    loop {
        if let Err(err) = console_clear() {
            error_helper(format!("{err}"));
        }

        let song_list = match music_player.song_list() {
            Err(err) => {
                error_helper(format!("an error accured when getting the song list: {err}"));
                continue;
            }
            Ok(list) => list,
        };

        println!("song list:");
        for (index, song) in song_list.iter().enumerate() {
            if current_song == Some(index) {
                print!("\x1b[1;31m")
            }
            println!(
                "{index}) {}",
                song.file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
            );
            print!("\x1b[0m");
        }

        println!("\nnum) type number of song to play");
        println!("p) to pause song");
        println!("r) to resume playing song");
        println!("q) to exit app");
        println!("i) WARN: pressing i turns off internet :) (experimental)");

        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Err(err) => {
                error_helper(format!("{err}"));
            }
            Ok(_) => {
                match input.trim().to_ascii_lowercase().as_str(){
                    "p" => {
                        music_player.pause();
                        continue;
                    },
                    "r" => {
                        music_player.resume();
                        continue;
                    },
                    "q" => {
                        break;
                    }
                    "i" => {
                        music_player.simulate_network(false);
                        continue;
                    }
                    _ => {}
                }
                match input.trim().parse::<usize>() {
                    Err(_) => {error_helper(format!("please choose a number"));},
                    Ok(num) => {
                        let song = match song_list.get(num) {
                            Some(s) => {s},
                            None => {error_helper(format!("song not in list")); continue;},
                        };
                        current_song = Some(num);
                        if let Err(err) = music_player.play_song(song) {
                            error_helper(format!("{err}"));
                        }
                    }
                }
            }
        }
    }
}