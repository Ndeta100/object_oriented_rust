pub struct AverageCollection {
    list: Vec<i32>,
    average: f64,
}
impl AverageCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    pub fn remove(&mut self) -> Option<i32> {
        let results = self.list.pop();
        match results {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }
    pub fn average(&self) -> f64 {
        self.average
    }
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

pub struct Screen {
    pub component: Vec<Box<dyn Draw>>,
}
impl Draw for Screen {
    fn draw(&self) {
        println!("Screen ",)
    }
}
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub option: Vec<String>,
}
impl Draw for Button {
    fn draw(&self) {
        println!("((Sreeen)) height: {}, width: {}", self.height, self.width);
    }
}
impl Draw for SelectBox {
    fn draw(&self) {
        println!("select box height: {}, width: {}", self.height, self.width);
    }
}
impl Screen {
    pub fn run(&self) {
        for components in self.component.iter() {
            components.draw();
        }
    }
}
pub trait Draw {
    fn draw(&self);
}
