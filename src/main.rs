use rodio::{OutputStream, Sink, source::SineWave, Source};
use std::time::Duration;

use pancurses::{initscr, noecho, endwin, Window, Input};
use std::thread::{sleep};

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
    notes: Vec<Note>,
    name: String
}

impl Melody {

    fn empty(given_name: String) -> Melody {
        Melody { notes: vec![] , name: given_name}
    }

    fn rests(n: usize, given_name: String) -> Melody {
        //creates a Melody object with n rests

        let mut line = Melody::empty(given_name);
        
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

    fn draw(&self, window: &Window) {
        
        let mut xpos = 10;
        let ypos = 10;

        for note in &self.notes {
            window.mvprintw(ypos, xpos, note.letter.to_string());
            xpos += 5;
        }
    }

    fn play(&self) {
        for note in &self.notes {
            note.play();
        }
    }
}

struct Cursor {
    /* this tracks where the cursors is, 
     * let's the user move it, and how it interacts with 
     * it's highlighted position */

    selected_melody: String,
}

impl Cursor {
    fn new(given_melody_name: String) -> Cursor {
        Cursor { selected_melody: given_melody_name }
    }
}

struct Score {
    melodies: Vec<Melody>,
    cursor: Cursor
}

impl Score {

    fn draw(&self, window: &Window) {
        
        let mut ypos = 10;
        let mut xpos = 10;

        for melody in &self.melodies {

            /* print the melody's name */
            if melody.name == self.cursor.selected_melody {
                window.mvprintw(ypos, xpos-1, "*");
            }

            window.mvprintw(ypos, xpos, melody.name.clone());
            ypos += 2;

            /* print the melody's notes */
        }

    }

    fn move_cursor_up(&mut self) {
        let mut index = 0;
        for i in 0..self.melodies.len() {
            if self.melodies[i].name == self.cursor.selected_melody {
                index = i;
                break;
            }
        }

        if index > 0 {
            self.cursor.selected_melody = self.melodies[index-1].name.clone();
        }
    }

    fn move_cursor_down(&mut self) {
        //find a more idiomatic way to do this later
        let mut index = 0;
        for i in 0..self.melodies.len() {
            if self.melodies[i].name == self.cursor.selected_melody {
                index = i;
                break;
            }
        }

        if index < self.melodies.len()-1 {
            self.cursor.selected_melody = self.melodies[index+1].name.clone();
        }
    }

    fn play(&self) { /* we're assuming all 1st species */
        let (_stream, stream_handle) = OutputStream::try_default().expect("failed to get audio stream.");

        for note_index in 0..self.melodies[0].notes.len() {
            let mut sinks = vec![];
            
            for melody in &self.melodies {
                sinks.push(Sink::try_new(&stream_handle).unwrap());
                let source = SineWave::new(melody.notes[note_index].get_freq())
                    .take_duration(Duration::from_secs_f32(0.5))
                    .amplify(0.20);
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

fn gen_counterpoint(gm: &Melody, given_name: String) -> Melody { /* only implementing 1st species at the moment.. */
    //return a countermelody for a given melody (gm)
    
    let mut cmelody = Melody::empty(given_name);

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

fn draw_bounds(window: &Window) {

    let height = window.get_max_y();
    let width = window.get_max_x();

    window.mvprintw(0, 0, "#".repeat(width as usize));
    window.mvprintw(height-1, 0, "#".repeat(width as usize));

    for i in 0..height-1 {
        window.mvprintw(i, 0, "#");
        window.mvprintw(i, width-1, "#");
    }
}

fn main() {

    /* create the window */
    let window = initscr();
    pancurses::curs_set(0);
    window.keypad(true);
    noecho();

    let mut cantus_firmus = Melody::rests(5, String::from("cantus firmus"));
    cantus_firmus.notes[0] = Note::new('C', 0, 4);
    cantus_firmus.notes[1] = Note::new('D', 0, 4);
    cantus_firmus.notes[2] = Note::new('E', 0, 4);
    cantus_firmus.notes[3] = Note::new('F', 0, 4);
    cantus_firmus.notes[4] = Note::new('G', 0, 4);

    let first_species = gen_counterpoint(&cantus_firmus, String::from("first species"));

    let mut score = Score{melodies: vec![cantus_firmus, first_species],
                      cursor: Cursor::new(String::from("cantus firmus"))};

    loop {
        window.clear();

        /* draw the GUI */
        draw_bounds(&window);
        score.draw(&window);
        window.refresh();

        /* handle user input */
        match window.getch() {
            Some(Input::KeyLeft) => {},
            Some(Input::KeyRight) => {},
            Some(Input::KeyUp) => { score.move_cursor_up(); },
            Some(Input::KeyDown) => { score.move_cursor_down(); },
            _ => {}
        }
        
    }

    endwin();

    /*let first_species = gen_counterpoint(&cantus_firmus);

    cantus_firmus.print();
    first_species.print();

    cantus_firmus.play();

    let foo = Score {melodies: vec![cantus_firmus, first_species], cursor: Cursor::new(String::from("cantus firmus"))};
    foo.play(); */
}
