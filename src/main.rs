enum IntervalQuality { Perfect, Imperfect, Dissonant }

struct Note {
    letter: char,
    accidental: i8, //-1 flat, 0 natural, 1 sharp
    octave: i8
}

impl Note {
    fn new(given_letter: char, given_accidental: i8, given_octave: i8) -> Note {
        Note {letter: given_letter,
              accidental: given_accidental,
              octave: given_octave}
    }

    fn get_semitones(&self) -> i8 {
        let st = match self.letter {
            'C' => 0,
            'D' => 2,
            'E' => 4,
            'F' => 5,
            'G' => 7,
            'A' => 9,
            'B' => 11,
             _  => 0
        };

        st + (self.octave * 12) + self.accidental
    }
}

/* fn getIntervalQuality(a: Note, b: Note) -> IntervalQuality {
} */

fn main() {
    let foo = Note::new('C', 0, 1);
    println!("{}", foo.get_semitones());
}
