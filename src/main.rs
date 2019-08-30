mod riichi;

use riichi::hand::Hand;

fn main() {
    let hand = Hand::random_hand(13);

    println!("hand: {}", hand);

    let hand2 = Hand::from_text("s4m7p3m1m4s8s1s3p9z1z7p2p5").unwrap();

    println!("hand2: {}", hand2);
}
