use console::Term;

struct BigThing<'a>{
    little_thing: &'a LittleThing
}

impl<'a> BigThing<'a> {
    fn new(little_thing: &LittleThing) -> BigThing{
        return BigThing {
            little_thing: little_thing
        };
    }

    fn do_something(&self){
        if let Some(x) = self.little_thing.thing.first(){
            println!("{}", x);
        }
    }
}

struct LittleThing {
    thing: Vec<i32>
}

impl LittleThing {
    fn new(thing: Vec<i32>) -> LittleThing{
        return LittleThing {
            thing: thing
        };
    }

    fn clear_and_push(&mut self, number: i32){
        self.clear();
        self.thing.push(number);
    }

    fn clear(&mut self){
        self.thing.clear();
    }
}

fn main() {
    let stdout = Term::buffered_stdout();

    let mut little_thing = LittleThing::new(Vec::with_capacity(10));

    'program_loop:  loop {

        if let Ok(input) = stdout.read_char() {
            match input {
                '1' => little_thing.clear_and_push(1),
                _ => { break 'program_loop;}
            }
        }

        let big_thing = BigThing::new(&little_thing);

        // do stuff with big_thing that reads from thing but doesn't modify any of it
        big_thing.do_something();
    }
}