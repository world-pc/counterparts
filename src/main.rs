enum IntervalQuality { Perfect, Imperfect, Dissonant }

struct Note {
    letter: char,
    accidental: i8, //-1 flat, 0 natural, 1 sharp
    octave: i8,
    rest: bool
}

impl Note {
    fn new(given_letter: char, given_accidental: i8, given_octave: i8) -> Note {
        Note {letter: given_letter,
              accidental: given_accidental,
              octave: given_octave,
              rest: false}
    }

    fn rest() -> Note {
        Note {letter: 'C',
              accidental: 0,
              octave: 0,
              rest: true}
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

struct Melody {
    notes: Vec<Note>
}

impl Melody {
    fn empty() -> Melody {
        Melody { notes: vec![] }
    }

    fn rests(n: usize) -> Melody {
        //creates a Melody object with n rests

        let mut line = Melody::empty();
        
        for i in 0..n {
            line.notes.push(Note::rest());
        }

        line
    }

    fn _print(&self) {
        for note in self.notes {
        }
    }
}

fn get_interval_quality(a: Note, b: Note) -> IntervalQuality {
    //accepts two notes and returns their IntervalQuality
    
    let diff = a.get_semitones() - b.get_semitones();
    let adiff = diff.abs() % 12;

    match adiff {
        //unison, fifth, octave
        0 | 7 | 12 => IntervalQuality::Perfect,

        //major 3rd, minor 3rd, major 6th, minor 6th
        4 | 3 | 9 | 8 => IntervalQuality::Imperfect,

        //minor 2nd, major 2nd, tritone, minor 7th, major 7th
        1 | 2 | 6 | 10 | 11 => IntervalQuality::Dissonant,

        _ => IntervalQuality::Dissonant
    }
}

fn _interval_check() {
}

fn _motion_check() {
}

fn _melodic_check() {
}


fn main() {
    let foo = Note::new('C', 0, 1);
    let bar = Note::new('D', 0, 1);
    get_interval_quality(foo, bar);

    let cantus_firmus = Melody::rests(4);
    println!("{}", cantus_firmus.notes.len());
}
