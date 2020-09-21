enum Emotion{
    Anger,
    Happy,
}

trait Emotional{
    fn get_happy(&mut self) -> String;
    fn get_anger(&mut self) -> String;
    fn tell_state(&self) -> String;
}

struct HappyPerson{
    name: String,
    state: Emotion,
}



fn main(){

}