use std::{thread, time};

trait State {
    fn update(self: Box<Self>, pawn: &mut Pawn) -> Box<dyn State>;
}

// Standing sta
struct Standing {}

impl State for Standing {
    fn update(self: Box<Self>, pawn: &mut Pawn) -> Box<dyn State> {
        self
    }
}

// Walking state for a cat
struct Walking {
    duration: i32, //seconds
}

impl State for Walking {
    fn update(mut self: Box<Self>, pawn: &mut Pawn) -> Box<dyn State> {
        self.duration -= 1;

        if self.duration < 0 {
            pawn.frame = 0;
            pawn.current_frames = vec![1, 2];
            Box::new(Standing {})
        } else {
            self
        }
    }
}

// Holding state of a cat
struct Pawn {
    frame: usize,
    current_frames: Vec<usize>,
    state: Option<Box<dyn State>>,
}

impl Pawn {
    fn new() -> Pawn {
        Pawn {
            frame: 0,
            state: Some(Box::new(Walking { duration: 5 })),
            current_frames: vec![3, 4],
        }
    }

    fn update(&mut self) {
        // handle animation frames
        self.frame += 1;
        if self.frame >= self.current_frames.len() {
            self.frame = 0;
        }

        // handle states
        if let Some(state) = self.state.take() {
            self.state = Some(state.update(self));
        }
    }

    fn get_current_frame(&self) -> usize {
        self.current_frames[self.frame]
    }
}

fn main() {
    let anim: Vec<Vec<&str>> = vec![
        vec!["  ^___^", "('o' )", "( u u )"],             // Look left
        vec!["  ^___^", "( 'o' )", "( u u )"],            // Standing
        vec!["  ^___^", "( 'o' )  SUBSCRIBE", "( u u )"], // Say Subscribe
        vec!["     ^__^", "   ( 'o')", "@(u  Uu)"],       // Walk frame1
        vec!["     ^_^", "   ( 'o')", "@(U  uj)"],        // Walk frame2
    ];

    let delay = time::Duration::from_secs(1);

    let mut pawn = Pawn::new();

    loop {
        pawn.update();

        let frame = &anim[pawn.get_current_frame()];

        for line in frame {
            println!("{:<25}", line);
        }

        println!("\u{1b}[4A");

        thread::sleep(delay);
    }
}
