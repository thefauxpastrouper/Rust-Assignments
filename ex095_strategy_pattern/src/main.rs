// Strategy Pattern

trait RouteStrategy {
    fn route(&self, from: &str, to: &str);
}

struct WalkingStrategy;
impl RouteStrategy for WalkingStrategy {
    fn route(&self, from: &str, to: &str) {
        println!("Walking route from {} to {}, 4km, 30min", from, to);
    } 
}

struct TrainStrategy;
impl RouteStrategy for TrainStrategy {
    fn route(&self, from: &str, to: &str) {
        println!("Travelling By train from {} to {}", from, to);
    }
}

struct Navigator {
    route_strategy: Box<dyn RouteStrategy>
}

impl Navigator {
    fn new(strategy: Box<dyn RouteStrategy>) -> Self {
        Navigator { route_strategy: strategy }
    }
    fn route(&self, from: &str, to: &str) {
        self.route_strategy.route(from, to);
    }
}

fn main() {
    let navigator = Navigator::new(Box::new(WalkingStrategy));
    
    navigator.route("Home", "ChinaTown");

    let navigator = Navigator::new(Box::new(TrainStrategy));

    navigator.route("Home", "ChinaTown");
}
