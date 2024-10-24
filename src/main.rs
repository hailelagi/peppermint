mod morris;

fn main() {
    let mut counter = morris::ApprxCounter::new();
    
    for _ in 0..1_000_000 {
        counter.increment();
    }
    
    println!("Estimated count: {}", counter.estimate());
}
