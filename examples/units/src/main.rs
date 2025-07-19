use runit::u;

fn main() {
    let v = u!(12.0 V);

    let r = u!(6.0 Ω);

    let i = v / r;

    println!("Voltage: {}", v);
    println!("Resistance: {}", r);
    println!("Current (I = V / R): {}", i);

    let v2 = u!(3.0 V);
    let v_total = v + v2;
    println!("Total voltage (v + v2): {}", v_total);

    let small_current = u!(2.0 mA);
    println!("Small current: {}", small_current);
}
