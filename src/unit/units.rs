use super::{Unit, UnitNumber};
use paste::paste;

macro_rules! define_unit {
    ($name:ident, $symbol:literal) => {
        paste! {
            #[derive(PartialEq, Eq, Clone, Copy)]
            pub struct [<$name Unit>];

            impl Unit for [<$name Unit>] {
                fn name() -> &'static str {
                    $symbol
                }
            }

            pub type $name = UnitNumber<[<$name Unit>]>;
        }
    };
}

define_unit!(Voltage, "V");
define_unit!(Current, "A");
define_unit!(Resistance, "Ω");
define_unit!(Capacitance, "F");
define_unit!(Inductance, "H");
define_unit!(Charge, "Q");
define_unit!(Power, "W");
define_unit!(Energy, "J");
define_unit!(Time, "s");
define_unit!(Frequency, "Hz");
define_unit!(Length, "m");
define_unit!(Area, "m²");
define_unit!(Force, "N");
define_unit!(Pressure, "Pa");
define_unit!(MagneticFlux, "Wb");
define_unit!(FluxDensity, "T");
define_unit!(Conductance, "S");
define_unit!(Velocity, "m/s");
define_unit!(Accel, "m/s²");
define_unit!(Temperature, "K");
define_unit!(Angle, "rad");

#[macro_export]
macro_rules! define_rule {
    ($out:ty, $lhs:ty, $rhs:ty) => {
        paste! {
            impl $crate::UnitMul<[<$rhs Unit>]> for [<$lhs Unit>] {
                type Output = [<$out Unit>];
            }

            impl $crate::UnitDiv<[<$rhs Unit>]> for [<$out Unit>] {
                type Output = [<$lhs Unit>];
            }

            impl $crate::UnitDiv<[<$lhs Unit>]> for [<$out Unit>] {
                type Output = [<$rhs Unit>];
            }
        }
    };
}

define_rule!(Voltage, Resistance, Current);    // V = R × I
define_rule!(Power, Voltage, Current);         // P = V × I
define_rule!(Energy, Power, Time);             // E = P × t
define_rule!(Charge, Capacitance, Voltage);    // Q = C × V
define_rule!(Charge, Current, Time);           // Q = C × V
define_rule!(Current, Charge, Time);           // I = Q / t

define_rule!(Power, Force, Velocity);          // P = F × v
define_rule!(Energy, Force, Length);           // E = F × d
define_rule!(Force, Pressure, Area);           // F = P × A

define_rule!(MagneticFlux, FluxDensity, Area); // Φ = B × A
define_rule!(MagneticFlux, Voltage, Time);     // Φ = V × t

impl Frequency {
    pub fn to_period(&self) -> Time {
        Time::new(1. / self.number)
    }
}

impl Time {
    pub fn to_frquency(&self) -> Frequency {
        Frequency::new(1. / self.number)
    }
}

#[cfg(test)]
mod tests {
    use crate::{num, u, i, r};

    use super::*;

    #[test]
    fn test_same_unit_add_sub() {
        let q1 = u!(10.0 mQ); 
        let q2 = u!(5.0 mQ);
        let q3 = q1 + q2;
        assert_eq!(format!("{:.1}", q3), "15.0mQ");

        let t1 = u!(2. us); 
        let t2 = u!(3. us);
        let t3 = t2 - t1;
        assert_eq!(format!("{:.0}", t3), "1us");

        let v1 = u!(1.5 V);
        let v2 = u!(0.5 V);
        assert_eq!((v1 + v2).to_string(), "2V");

        let i1 = u!(1.0 A);
        let i2 = u!(0.1 A);
        assert_eq!(format!("{:.2}", i1 - i2), "900.00mA");

        let r1 = u!(100. Ω);
        let r2 = u!(200. Ω);
        assert_eq!((r1 + r2).to_string(), "300Ω");

        let c1 = u!(10.0 F);
        let c2 = u!(5.0 F);
        assert_eq!(format!("{:.0}", c1 - c2), "5F");

        let e1 = u!(1.2 J);
        let e2 = u!(0.8 J);
        assert_eq!((e1 + e2).to_string(), "2J");

        let f1 = u!(9.8 N);
        let f2 = u!(0.2 N);
        assert_eq!(format!("{:.1}", f1 - f2), "9.6N");

        let t1 = u!(300. K);
        let t2 = u!(273. K);
        assert_eq!(format!("{:.0}", t1 - t2), "27K");

        let a1 = u!(1. rad);
        let a2 = u!(2. rad);
        assert_eq!((a1 + a2).to_string(), "3rad");
    }

    #[test]
    fn test_display_default() {
        let v = Voltage::new(3.1415926);
        assert_eq!(v.to_string(), "3.1415926V");

        let i = Current::new(0.005);
        assert_eq!(i.to_string(), "0.005A");

        let r = Resistance::new(220.0);
        assert_eq!(r.to_string(), "220Ω");
    }

    #[test]
    fn test_display_precision() {
        let v = Voltage::new(3.1415926);
        assert_eq!(format!("{:.2}", v), "3.14V");
        assert_eq!(format!("{:.4}", v), "3.1416V");

        let i = Current::new(0.0001234);
        assert_eq!(format!("{:.6}", i), "0.000123A");
    }

    #[test]
    fn test_display_zero_and_negative() {
        let v = Voltage::new(0.0);
        assert_eq!(v.to_string(), "0V");

        let i = Current::new(-1.23);
        assert_eq!(i.to_string(), "-1.23A");
    }

    #[test]
    fn test_display_scientific_values() {
        let big = Voltage::new(1e6);
        assert_eq!(format!("{}", big), "1000000V");

        let small = Current::new(1e-9);
        assert_eq!(format!("{:.2}", small), "0.00A"); // 精度控制仍会影响
    }

    #[test]
    fn test_display_various_units() {
        let e = Energy::new(12.5);
        assert_eq!(e.to_string(), "12.5J");

        let t = Time::new(0.001);
        assert_eq!(format!("{:.3}", t), "0.001s");

        let f = Force::new(9.81);
        assert_eq!(f.to_string(), "9.81N");

        let p = Pressure::new(101325.0);
        assert_eq!(format!("{:.0}", p), "101325Pa");
    }
    
    #[test]
    fn test_ohms_law_voltage() {
        let i = Current::new(num!(2.0));
        let r = Resistance::new(num!(5.0));
        let v = r * i;
        assert_eq!(v.value(), num!(10.0));

        let i = u!(2.0 A);
        let r = u!(5.0 Ω);
        let v = r * i;
        assert_eq!(v.value(), num!(10.0));

        let i = i!(2.0);
        let r = r!(5.0);
        let v = r * i;
        assert_eq!(v.value(), num!(10.0));
    }

    #[test]
    fn test_power_from_voltage_current() {
        let v = Voltage::new(num!(3.0));
        let i = Current::new(num!(2.0));
        let p = v * i;
        assert_eq!(p.value(), num!(6.0));

        let v = u!(3.0 v);
        let i = u!(2.0 A);
        let p = v * i;
        assert_eq!(p.value(), num!(6.0));
    }

    #[test]
    fn test_energy_from_power_time() {
        let p = Power::new(num!(5.0));
        let t = Time::new(num!(10.0));
        let e = p * t;
        assert_eq!(e.value(), num!(50.0));

        let p = u!(5.0 mW);
        let t = u!(10. s);
        let e = p * t;
        assert_eq!(e.value(), num!(50.0 m));
    }

    #[test]
    fn test_charge_from_capacitance_voltage() {
        let c = Capacitance::new(num!(1.5));
        let v = Voltage::new(num!(4.0));
        let q = c * v;
        assert_eq!(q.value(), num!(6.0));

        let c = u!(1.5 pF);
        let v = Voltage::new(num!(4.0));
        let q = c * v;
        assert_eq!(q.value(), num!(6.0 p));
    }

    #[test]
    fn test_current_from_charge_time() {
        let q = Charge::new(num!(10.0));
        let t = Time::new(num!(2.0));
        let i = q / t;
        assert_eq!(i.value(), num!(5.0));

        let q = u!(10.0 mQ);
        let t = u!(2. us);
        let i = q / t;
        assert_eq!(i.value(), num!(5.0 k));
    }

    #[test]
    fn test_power_from_force_velocity() {
        let f = Force::new(num!(3.0));
        let v = Velocity::new(num!(4.0));
        let p = f * v;
        assert_eq!(p.value(), num!(12.0));
    }

    #[test]
    fn test_energy_from_force_length() {
        let f = Force::new(num!(10.0));
        let d = Length::new(num!(2.0));
        let e = f * d;
        assert_eq!(e.value(), num!(20.0));
    }

    #[test]
    fn test_pressure_from_force_area() {
        let f = Force::new(num!(100.0));
        let a = Area::new(num!(5.0));
        let p = f / a;
        assert_eq!(p.value(), num!(20.0));
    }

    #[test]
    fn test_flux_from_density_area() {
        let b = FluxDensity::new(num!(2.0));
        let a = Area::new(num!(3.0));
        let phi = b * a;
        assert_eq!(phi.value(), num!(6.0));
    }

}
