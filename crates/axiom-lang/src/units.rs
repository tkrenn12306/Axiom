/// A physical quantity: a numeric value paired with its unit.
#[derive(Debug, Clone, PartialEq)]
pub struct Quantity {
    pub value: f64,
    pub unit: Unit,
}

impl Quantity {
    pub fn new(value: f64, unit: Unit) -> Self {
        Self { value, unit }
    }

    /// Convert this quantity to its SI base unit value.
    /// Returns `None` if the unit is dimensionless.
    pub fn to_si(&self) -> f64 {
        to_si_value(self.value, &self.unit)
    }

    /// Returns the SI base unit for this quantity's dimension.
    pub fn si_unit(&self) -> Unit {
        self.unit.si_base()
    }

    /// Returns the physical dimension of this unit.
    pub fn dimension(&self) -> Dimension {
        self.unit.dimension()
    }

    /// Returns true if this quantity is compatible (same dimension) with another.
    pub fn is_compatible_with(&self, other: &Quantity) -> bool {
        self.unit.dimension() == other.unit.dimension()
    }
}

impl std::fmt::Display for Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.unit)
    }
}

/// Physical dimensions — used for compatibility checking.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dimension {
    Dimensionless,
    Mass,                // kg
    Length,              // m
    Time,                // s
    Temperature,         // K (Kelvin)
    Velocity,            // m/s
    Acceleration,        // m/s²
    Force,               // N = kg·m/s²
    Pressure,            // Pa = N/m²
    Energy,              // J = N·m
    Power,               // W = J/s
    Area,                // m²
    Volume,              // m³
    Density,             // kg/m³
    ThermalConductivity, // W/(m·K)
    SpecificHeat,        // J/(kg·K)
    Angle,               // rad
    AngularVelocity,     // rad/s
    Frequency,           // Hz
    ElasticModulus,      // Pa (same as pressure dimensionally)
}

/// All supported units in AxiomLang.
#[derive(Debug, Clone, PartialEq)]
pub enum Unit {
    // ─── Mass ───────────────────────────────────
    Kilogram,
    Gram,
    Milligram,
    Pound,

    // ─── Length ─────────────────────────────────
    Meter,
    Centimeter,
    Millimeter,
    Kilometer,
    Foot,
    Yard,
    Mile,

    // ─── Time ───────────────────────────────────
    Second,
    Minute,
    Hour,

    // ─── Temperature ────────────────────────────
    Kelvin,
    Celsius,
    Fahrenheit,

    // ─── Velocity ───────────────────────────────
    MetersPerSecond,
    KilometersPerHour,
    MilesPerHour,

    // ─── Acceleration ───────────────────────────
    MetersPerSecondSquared,

    // ─── Force ──────────────────────────────────
    Newton,
    Kilonewton,

    // ─── Pressure / Elastic Modulus ─────────────
    Pascal,
    Hectopascal,
    Kilopascal,
    Megapascal,
    Gigapascal,
    Atmosphere,

    // ─── Energy ─────────────────────────────────
    Joule,
    Kilojoule,
    Megajoule,
    Gigajoule,
    Kilocalorie,
    Calorie,

    // ─── Power ──────────────────────────────────
    Watt,
    Kilowatt,

    // ─── Area ───────────────────────────────────
    SquareMeter,
    SquareCentimeter,
    SquareKilometer,

    // ─── Volume ─────────────────────────────────
    CubicMeter,
    Liter,
    Milliliter,

    // ─── Density ────────────────────────────────
    KilogramPerCubicMeter,

    // ─── Thermal Conductivity ───────────────────
    WattPerMeterKelvin,

    // ─── Specific Heat ──────────────────────────
    JoulePerKilogramKelvin,

    // ─── Angle ──────────────────────────────────
    Radian,
    Degree,

    // ─── Angular Velocity ───────────────────────
    RadianPerSecond,

    // ─── Frequency ──────────────────────────────
    Hertz,

    // ─── Dimensionless ──────────────────────────
    Dimensionless,

    // ─── CLO (insulation) ───────────────────────
    Clo,
}

impl Unit {
    /// Returns the physical dimension of this unit.
    pub fn dimension(&self) -> Dimension {
        match self {
            Unit::Kilogram | Unit::Gram | Unit::Milligram | Unit::Pound => Dimension::Mass,

            Unit::Meter
            | Unit::Centimeter
            | Unit::Millimeter
            | Unit::Kilometer
            | Unit::Foot
            | Unit::Yard
            | Unit::Mile => Dimension::Length,

            Unit::Second | Unit::Minute | Unit::Hour => Dimension::Time,

            Unit::Kelvin | Unit::Celsius | Unit::Fahrenheit => Dimension::Temperature,

            Unit::MetersPerSecond | Unit::KilometersPerHour | Unit::MilesPerHour => {
                Dimension::Velocity
            }

            Unit::MetersPerSecondSquared => Dimension::Acceleration,

            Unit::Newton | Unit::Kilonewton => Dimension::Force,

            Unit::Pascal
            | Unit::Hectopascal
            | Unit::Kilopascal
            | Unit::Megapascal
            | Unit::Gigapascal
            | Unit::Atmosphere => Dimension::Pressure,

            Unit::Joule
            | Unit::Kilojoule
            | Unit::Megajoule
            | Unit::Gigajoule
            | Unit::Kilocalorie
            | Unit::Calorie => Dimension::Energy,

            Unit::Watt | Unit::Kilowatt => Dimension::Power,

            Unit::SquareMeter | Unit::SquareCentimeter | Unit::SquareKilometer => Dimension::Area,

            Unit::CubicMeter | Unit::Liter | Unit::Milliliter => Dimension::Volume,

            Unit::KilogramPerCubicMeter => Dimension::Density,

            Unit::WattPerMeterKelvin => Dimension::ThermalConductivity,

            Unit::JoulePerKilogramKelvin => Dimension::SpecificHeat,

            Unit::Radian | Unit::Degree => Dimension::Angle,

            Unit::RadianPerSecond => Dimension::AngularVelocity,

            Unit::Hertz => Dimension::Frequency,

            Unit::Dimensionless | Unit::Clo => Dimension::Dimensionless,
        }
    }

    /// Returns the SI base unit for this unit's dimension.
    pub fn si_base(&self) -> Unit {
        match self.dimension() {
            Dimension::Mass => Unit::Kilogram,
            Dimension::Length => Unit::Meter,
            Dimension::Time => Unit::Second,
            Dimension::Temperature => Unit::Kelvin,
            Dimension::Velocity => Unit::MetersPerSecond,
            Dimension::Acceleration => Unit::MetersPerSecondSquared,
            Dimension::Force => Unit::Newton,
            Dimension::Pressure | Dimension::ElasticModulus => Unit::Pascal,
            Dimension::Energy => Unit::Joule,
            Dimension::Power => Unit::Watt,
            Dimension::Area => Unit::SquareMeter,
            Dimension::Volume => Unit::CubicMeter,
            Dimension::Density => Unit::KilogramPerCubicMeter,
            Dimension::ThermalConductivity => Unit::WattPerMeterKelvin,
            Dimension::SpecificHeat => Unit::JoulePerKilogramKelvin,
            Dimension::Angle => Unit::Radian,
            Dimension::AngularVelocity => Unit::RadianPerSecond,
            Dimension::Frequency => Unit::Hertz,
            Dimension::Dimensionless => Unit::Dimensionless,
        }
    }
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Unit::Kilogram => "kg",
            Unit::Gram => "g",
            Unit::Milligram => "mg",
            Unit::Pound => "lb",
            Unit::Meter => "m",
            Unit::Centimeter => "cm",
            Unit::Millimeter => "mm",
            Unit::Kilometer => "km",
            Unit::Foot => "ft",
            Unit::Yard => "yd",
            Unit::Mile => "mi",
            Unit::Second => "s",
            Unit::Minute => "min",
            Unit::Hour => "h",
            Unit::Kelvin => "K",
            Unit::Celsius => "°C",
            Unit::Fahrenheit => "°F",
            Unit::MetersPerSecond => "m/s",
            Unit::KilometersPerHour => "km/h",
            Unit::MilesPerHour => "mph",
            Unit::MetersPerSecondSquared => "m/s²",
            Unit::Newton => "N",
            Unit::Kilonewton => "kN",
            Unit::Pascal => "Pa",
            Unit::Hectopascal => "hPa",
            Unit::Kilopascal => "kPa",
            Unit::Megapascal => "MPa",
            Unit::Gigapascal => "GPa",
            Unit::Atmosphere => "atm",
            Unit::Joule => "J",
            Unit::Kilojoule => "kJ",
            Unit::Megajoule => "MJ",
            Unit::Gigajoule => "GJ",
            Unit::Kilocalorie => "kcal",
            Unit::Calorie => "cal",
            Unit::Watt => "W",
            Unit::Kilowatt => "kW",
            Unit::SquareMeter => "m²",
            Unit::SquareCentimeter => "cm²",
            Unit::SquareKilometer => "km²",
            Unit::CubicMeter => "m³",
            Unit::Liter => "L",
            Unit::Milliliter => "mL",
            Unit::KilogramPerCubicMeter => "kg/m³",
            Unit::WattPerMeterKelvin => "W/(m·K)",
            Unit::JoulePerKilogramKelvin => "J/(kg·K)",
            Unit::Radian => "rad",
            Unit::Degree => "deg",
            Unit::RadianPerSecond => "rad/s",
            Unit::Hertz => "Hz",
            Unit::Dimensionless => "",
            Unit::Clo => "clo",
        };
        write!(f, "{}", s)
    }
}

/// Parse a unit string into a `Unit` enum variant.
pub fn parse_unit(s: &str) -> Option<Unit> {
    match s {
        "kg" => Some(Unit::Kilogram),
        "g" => Some(Unit::Gram),
        "mg" => Some(Unit::Milligram),
        "lb" => Some(Unit::Pound),
        "m" => Some(Unit::Meter),
        "cm" => Some(Unit::Centimeter),
        "mm" => Some(Unit::Millimeter),
        "km" => Some(Unit::Kilometer),
        "ft" => Some(Unit::Foot),
        "yd" => Some(Unit::Yard),
        "mi" => Some(Unit::Mile),
        "s" => Some(Unit::Second),
        "min" => Some(Unit::Minute),
        "h" => Some(Unit::Hour),
        "K" => Some(Unit::Kelvin),
        "°C" | "C" => Some(Unit::Celsius),
        "°F" | "F" => Some(Unit::Fahrenheit),
        "m/s" => Some(Unit::MetersPerSecond),
        "km/h" => Some(Unit::KilometersPerHour),
        "mph" => Some(Unit::MilesPerHour),
        "m/s²" | "m/s2" => Some(Unit::MetersPerSecondSquared),
        "N" => Some(Unit::Newton),
        "kN" => Some(Unit::Kilonewton),
        "Pa" => Some(Unit::Pascal),
        "hPa" => Some(Unit::Hectopascal),
        "kPa" => Some(Unit::Kilopascal),
        "MPa" => Some(Unit::Megapascal),
        "GPa" => Some(Unit::Gigapascal),
        "atm" => Some(Unit::Atmosphere),
        "J" => Some(Unit::Joule),
        "kJ" => Some(Unit::Kilojoule),
        "MJ" => Some(Unit::Megajoule),
        "GJ" => Some(Unit::Gigajoule),
        "kcal" => Some(Unit::Kilocalorie),
        "cal" => Some(Unit::Calorie),
        "W" => Some(Unit::Watt),
        "kW" => Some(Unit::Kilowatt),
        "m²" | "m2" => Some(Unit::SquareMeter),
        "cm²" | "cm2" => Some(Unit::SquareCentimeter),
        "km²" | "km2" => Some(Unit::SquareKilometer),
        "m³" | "m3" => Some(Unit::CubicMeter),
        "L" | "l" => Some(Unit::Liter),
        "mL" | "ml" => Some(Unit::Milliliter),
        "kg/m³" | "kg/m3" => Some(Unit::KilogramPerCubicMeter),
        "W/(m·K)" | "W/(m*K)" | "W/mK" => Some(Unit::WattPerMeterKelvin),
        "J/(kg·K)" | "J/(kg*K)" | "J/kgK" => Some(Unit::JoulePerKilogramKelvin),
        "rad" => Some(Unit::Radian),
        "deg" => Some(Unit::Degree),
        "rad/s" => Some(Unit::RadianPerSecond),
        "Hz" | "hz" => Some(Unit::Hertz),
        "clo" | "CLO" => Some(Unit::Clo),
        "" => Some(Unit::Dimensionless),
        _ => None,
    }
}

/// Convert a value in the given unit to its SI base unit value.
/// For temperature, this is a conversion to Kelvin.
pub fn to_si_value(value: f64, unit: &Unit) -> f64 {
    match unit {
        // Mass → kg
        Unit::Kilogram => value,
        Unit::Gram => value * 1e-3,
        Unit::Milligram => value * 1e-6,
        Unit::Pound => value * 0.453_592_37,

        // Length → m
        Unit::Meter => value,
        Unit::Centimeter => value * 1e-2,
        Unit::Millimeter => value * 1e-3,
        Unit::Kilometer => value * 1e3,
        Unit::Foot => value * 0.3048,
        Unit::Yard => value * 0.9144,
        Unit::Mile => value * 1_609.344,

        // Time → s
        Unit::Second => value,
        Unit::Minute => value * 60.0,
        Unit::Hour => value * 3600.0,

        // Temperature → K
        Unit::Kelvin => value,
        Unit::Celsius => value + 273.15,
        Unit::Fahrenheit => (value - 32.0) * 5.0 / 9.0 + 273.15,

        // Velocity → m/s
        Unit::MetersPerSecond => value,
        Unit::KilometersPerHour => value / 3.6,
        Unit::MilesPerHour => value * 0.44704,

        // Acceleration → m/s²
        Unit::MetersPerSecondSquared => value,

        // Force → N
        Unit::Newton => value,
        Unit::Kilonewton => value * 1e3,

        // Pressure → Pa
        Unit::Pascal => value,
        Unit::Hectopascal => value * 1e2,
        Unit::Kilopascal => value * 1e3,
        Unit::Megapascal => value * 1e6,
        Unit::Gigapascal => value * 1e9,
        Unit::Atmosphere => value * 101_325.0,

        // Energy → J
        Unit::Joule => value,
        Unit::Kilojoule => value * 1e3,
        Unit::Megajoule => value * 1e6,
        Unit::Gigajoule => value * 1e9,
        Unit::Kilocalorie => value * 4_184.0,
        Unit::Calorie => value * 4.184,

        // Power → W
        Unit::Watt => value,
        Unit::Kilowatt => value * 1e3,

        // Area → m²
        Unit::SquareMeter => value,
        Unit::SquareCentimeter => value * 1e-4,
        Unit::SquareKilometer => value * 1e6,

        // Volume → m³
        Unit::CubicMeter => value,
        Unit::Liter => value * 1e-3,
        Unit::Milliliter => value * 1e-6,

        // Density → kg/m³
        Unit::KilogramPerCubicMeter => value,

        // Thermal conductivity → W/(m·K)
        Unit::WattPerMeterKelvin => value,

        // Specific heat → J/(kg·K)
        Unit::JoulePerKilogramKelvin => value,

        // Angle → rad
        Unit::Radian => value,
        Unit::Degree => value.to_radians(),

        // Angular velocity → rad/s
        Unit::RadianPerSecond => value,

        // Frequency → Hz
        Unit::Hertz => value,

        // Dimensionless
        Unit::Dimensionless => value,

        // CLO (not a strict SI unit — keep as-is)
        Unit::Clo => value,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mass_conversions() {
        assert!((to_si_value(1000.0, &Unit::Gram) - 1.0).abs() < 1e-10);
        assert!((to_si_value(1.0, &Unit::Pound) - 0.453_592_37).abs() < 1e-8);
    }

    #[test]
    fn test_length_conversions() {
        assert!((to_si_value(100.0, &Unit::Centimeter) - 1.0).abs() < 1e-10);
        assert!((to_si_value(1.0, &Unit::Kilometer) - 1000.0).abs() < 1e-10);
        assert!((to_si_value(1.0, &Unit::Foot) - 0.3048).abs() < 1e-10);
    }

    #[test]
    fn test_temperature_conversions() {
        // 0°C = 273.15 K
        assert!((to_si_value(0.0, &Unit::Celsius) - 273.15).abs() < 1e-10);
        // 32°F = 273.15 K
        assert!((to_si_value(32.0, &Unit::Fahrenheit) - 273.15).abs() < 1e-10);
        // 100°C = 373.15 K
        assert!((to_si_value(100.0, &Unit::Celsius) - 373.15).abs() < 1e-10);
    }

    #[test]
    fn test_velocity_conversions() {
        // 3.6 km/h = 1 m/s
        assert!((to_si_value(3.6, &Unit::KilometersPerHour) - 1.0).abs() < 1e-10);
        // 100 km/h = 27.777... m/s
        assert!((to_si_value(100.0, &Unit::KilometersPerHour) - 100.0 / 3.6).abs() < 1e-10);
    }

    #[test]
    fn test_pressure_conversions() {
        assert!((to_si_value(1.0, &Unit::Atmosphere) - 101_325.0).abs() < 1e-5);
        assert!((to_si_value(1.0, &Unit::Gigapascal) - 1e9).abs() < 1.0);
    }

    #[test]
    fn test_energy_conversions() {
        assert!((to_si_value(1.0, &Unit::Kilojoule) - 1000.0).abs() < 1e-10);
        assert!((to_si_value(1.0, &Unit::Kilocalorie) - 4184.0).abs() < 1e-10);
    }

    #[test]
    fn test_angle_conversion() {
        use std::f64::consts::PI;
        assert!((to_si_value(180.0, &Unit::Degree) - PI).abs() < 1e-10);
    }

    #[test]
    fn test_dimension_compatibility() {
        let a = Quantity::new(5.0, Unit::Kilogram);
        let b = Quantity::new(1000.0, Unit::Gram);
        let c = Quantity::new(5.0, Unit::Meter);
        assert!(a.is_compatible_with(&b));
        assert!(!a.is_compatible_with(&c));
    }

    #[test]
    fn test_parse_unit() {
        assert_eq!(parse_unit("kg"), Some(Unit::Kilogram));
        assert_eq!(parse_unit("km/h"), Some(Unit::KilometersPerHour));
        assert_eq!(parse_unit("°C"), Some(Unit::Celsius));
        assert_eq!(parse_unit("GPa"), Some(Unit::Gigapascal));
        assert_eq!(parse_unit("unknown_xyz"), None);
    }
}
