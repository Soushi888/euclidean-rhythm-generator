use rodio::{Decoder, OutputStream, Source};

mod euclidean_rhythm;

use euclidean_rhythm::{e, inverse_matrix};
use std::{env::args, io::Cursor, time::Duration};

const SOUND_DATA: &'static [u8] = include_bytes!("../quick_fart_x.wav");

fn main() {
    let beats = args().nth(1).unwrap_or("3".to_string());

    let mut rhythm = match beats.as_str() {
        "tressio" => tressio(),
        "quintillo" => quintillo(),
        "venda" => venda(),
        "bembe" => bembe(),
        _ => {
            let beats: usize = beats.parse().unwrap_or(3);
            let steps: usize = args().nth(2).unwrap_or("8".to_string()).parse().unwrap();

            e(beats, steps)
        }
    };

    if args().nth(2).unwrap_or_default() == "inverse"
        || args().nth(3).unwrap_or_default() == "inverse"
    {
        inverse_matrix(&mut rhythm);
    }

    loop {
        play_beat(&rhythm);
    }
}

fn play_beat(beats: &Vec<u8>) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    for beat in beats {
        print!("{}", beat);
        if beat == &1 {
            let cursor = Cursor::new(SOUND_DATA);
            let source = Decoder::new(cursor).unwrap().convert_samples();
            stream_handle.play_raw(source).unwrap();
        }
        std::thread::sleep(Duration::from_millis(300));
    }
}

fn tressio() -> Vec<u8> {
    e(3, 8)
}

fn quintillo() -> Vec<u8> {
    e(5, 8)
}

fn venda() -> Vec<u8> {
    e(5, 12)
}
fn bembe() -> Vec<u8> {
    e(7, 12)
}
