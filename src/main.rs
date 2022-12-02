use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let filename = "src/les_douze_jours_de_noel.txt";
    let m:usize = 0;
    let cnt = line_counter(filename);

    song_iteration(m, cnt, filename);
}

fn song_iteration(mut m: usize, cnt: usize, filename: &str) {

    while m < cnt + 1 {

        let file = File::open(filename).unwrap();
        let mut reader = BufReader::new(file);
        let reader_ref = &mut reader;

        println!("Le jour {} de Noël", m + 1);
        println!("J'ai reçu de mon ami");
        for (index, line) in reader_ref.lines().enumerate() {
            let line = line.unwrap();
            if index < m {
                println!("{line}");
            };
        }
        if m > 0 {
            println!("Et un moineau tout en haut du pommier\n");
        } else {
            println!("Un moineau tout en haut du pommier\n");
        }
        
        m += 1;
        }
}

fn line_counter(filename: &str) -> usize {

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let cnt  = reader.lines().count();
    cnt
}
