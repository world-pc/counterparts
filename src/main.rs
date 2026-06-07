use rodio::{OutputStream, Sink, source::SineWave, Source};
use std::time::Duration;

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
        Note {letter: '_',
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

    fn from_semitones(given_st: &i8) -> Note {
        let octave = given_st / 12;

        let mut letter = '_';
        let mut accidental = 0;

        match given_st % 12 {
            0 => { letter = 'C'; accidental = 0; },
            1 => { letter = 'C'; accidental = 1; },
            2 => { letter = 'D'; accidental = 0; },
            3 => { letter = 'D'; accidental = 1; },
            4 => { letter = 'E'; accidental = 0; },
            5 => { letter = 'F'; accidental = 0; },
            6 => { letter = 'F'; accidental = 1; },
            7 => { letter = 'G'; accidental = 0; },
            8 => { letter = 'G'; accidental = 1; },
            9 => { letter = 'A'; accidental = 0; },
            10 => { letter = 'A'; accidental = 1; },
            11 => { letter = 'B'; accidental = 0; },
            _ => { letter = '_'; accidental = 0; }
        }

        Note {letter: letter,
              accidental: accidental,
              octave: octave,
              rest: false}
    }

    fn get_freq(&self) -> f32 {
        440.0 * 2f32.powf((self.get_semitones() as f32 - 69.0) / 12.0)
    }

    fn play(&self) {

        let note_freq = self.get_freq(); 

        let (stream, stream_handle) = OutputStream::try_default().expect("failed to get audio stream.");
        let sink = Sink::try_new(&stream_handle).unwrap();
        
        let source = SineWave::new(note_freq)
            .take_duration(Duration::from_secs_f32(0.5))
            .amplify(0.70);

        sink.append(source);
        println!("playing...");
        sink.sleep_until_end();
        println!("done.");
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
        
        for _ in 0..n {
            line.notes.push(Note::rest());
        }

        line
    }

    fn print(&self) {
        for note in &self.notes {
            print!("{}\t", note.letter);
        }
        println!("");
    }

    fn play(&self) {
        for note in &self.notes {
            note.play();
        }
    }
}

struct Score {
    melodies: Vec<Melody>
}

impl Score {

    fn play(&self) { /* we're assuming all 1st species */
        let (_stream, stream_handle) = OutputStream::try_default().expect("failed to get audio stream.");

        for note_index in 0..self.melodies[0].notes.len() {
            let mut sinks = vec![];
            
            for melody in &self.melodies {
                sinks.push(Sink::try_new(&stream_handle).unwrap());
                let source = SineWave::new(melody.notes[note_index].get_freq())
                    .take_duration(Duration::from_secs_f32(0.5))
                    .amplify(0.70);
                sinks.last().unwrap().append(source);
            }

            sinks[0].sleep_until_end();
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

fn harmonic_check() {
}

fn _motion_check() {
}

/* we're doing a lot of back and forth conversions from Note to semitones.. 
we'll make it neater / efficient later. */

fn melodic_check(first: &Note, second: &Note) -> bool{
    //returns true if these two consecutive notes are 
    //fine, melodically.
    
    let first_st = first.get_semitones();
    let second_st = second.get_semitones();

    //avoid tritone leaps
    if (first_st - second_st).abs() == 6 {
        return false;
    }

    true
}

fn gen_counterpoint(gm: &Melody) -> Melody { /* only implementing 1st species at the moment.. */
    //return a countermelody for a given melody (gm)
    
    let mut cmelody = Melody::empty();

    for note in &gm.notes {

        //semitone value of note in given melody
        let note_st = note.get_semitones();

        //generate consonant notes..
        let mut consonants = vec![note_st + 0,
                                  note_st + 7,
                                  note_st + 12,
                                  note_st + 4,
                                  note_st + 3,
                                  note_st + 9,
                                  note_st + 8];
        
        //remove the unison consonant if we're at the first/last in gm
        if !(cmelody.notes.is_empty()) && !(cmelody.notes.len() == gm.notes.len()-1) {
            consonants.retain(|x| *x != note_st);
        }

        for consonant in &consonants {
            if let Some(last_note) = cmelody.notes.last() {
                if melodic_check(last_note, &Note::from_semitones(consonant)) {
                    cmelody.notes.push(Note::from_semitones(consonant));
                    break;
                }
            }
            else {
                cmelody.notes.push(Note::from_semitones(consonant));
                break;
            }
        }
    }

    cmelody
}

fn main() {

    let mut cantus_firmus = Melody::rests(5);
    cantus_firmus.notes[0] = Note::new('C', 0, 4);
    cantus_firmus.notes[1] = Note::new('D', 0, 4);
    cantus_firmus.notes[2] = Note::new('E', 0, 4);
    cantus_firmus.notes[3] = Note::new('F', 0, 4);
    cantus_firmus.notes[4] = Note::new('G', 0, 4);

    let first_species = gen_counterpoint(&cantus_firmus);

    cantus_firmus.print();
    first_species.print();

    cantus_firmus.play();

    let foo = Score {melodies: vec![cantus_firmus, first_species]};
    foo.play();
}
