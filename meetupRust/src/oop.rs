pub struct BaseUnit{
    hp: f32,
    pos_x: f32,
    pos_y:f32
}

impl Default for BaseUnit{
    fn default() -> BaseUnit {
        BaseUnit {
            hp : 100.0,
            pos_x: 0.0,
            pos_y: 0.0
        }
    }
}

impl BaseUnit{
    fn get_pos(&self) ->(f32, f32){
        (self.pos_x, self.pos_y)
    }
    fn moveto(&mut self, x: f32, y: f32){
        self.pos_x = x;
        self.pos_y = y;
    }
    fn print_unit(&self){
        println!("hp: {}; pos: {}, {}",self.hp, self.pos_x, self.pos_y);
    }
}

struct Worker{
    unit : BaseUnit,
    stamina: f32,
}

impl Default for Worker{
    fn default() -> Worker {
        Worker {
            unit: Default::default(),
            stamina: 100.0,
        }
    }
}

struct Warrior{
    unit : BaseUnit,
    damage: f32,
}

impl Default for Warrior{
    fn default() -> Warrior {
        Warrior {
            unit: Default::default(),
            damage: 10.0,
        }
    }
}

trait Unit{
    fn get_unit(&mut self)->&mut BaseUnit;
    fn do_job(&mut self);
}

impl Unit for Worker{
    fn get_unit(&mut self)->&mut BaseUnit{
        &mut self.unit
    }
    fn do_job(&mut self) {
        println!("Get {} resources", self.stamina);
    }
}

impl Unit for Warrior{
    fn get_unit(&mut self)->&mut BaseUnit{
        &mut self.unit
    }
    fn do_job(&mut self) {
        println!("Strike power {}", self.damage);
    }
}


fn main() {
    let mut worker : Worker = Worker::default();
    let mut warrior: Warrior = Warrior::default();

    let mut units: Vec<&mut dyn Unit> = vec![&mut worker, &mut warrior];

    for unit in units.iter_mut() {
        let base_unit = unit.get_unit();
        let (x, y) = base_unit.get_pos();
        base_unit.moveto(x + 1.0, y + 1.0);
        base_unit.print_unit();
        unit.do_job();
    }
}