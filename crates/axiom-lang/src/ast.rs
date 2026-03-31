use std::collections::HashMap;
use crate::units::Quantity;

/// A complete parsed `.ax` file.
#[derive(Debug, Default, Clone)]
pub struct AxiomFile {
    pub world: Option<WorldConfig>,
    pub materials: Vec<MaterialDef>,
    pub entities: Vec<EntityDef>,
    pub terrains: Vec<TerrainDef>,
    pub structures: Vec<StructureDef>,
    pub weathers: Vec<WeatherDef>,
    pub spawn_cmds: Vec<SpawnCmd>,
    pub place_cmds: Vec<PlaceCmd>,
}

/// A value in the AxiomLang AST.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Quantity(Quantity),
    Boolean(bool),
    Text(String),
    Identifier(String),
    Range(f64, f64),
    Pair(f64, f64),
    Triple(f64, f64, f64),
}

impl Value {
    /// If this value has a numeric component, return it as f64.
    /// For quantities, returns the raw (pre-SI) value.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            Value::Quantity(q) => Some(q.value),
            _ => None,
        }
    }

    /// Returns the SI-converted value if this is a Quantity or Number.
    pub fn as_si(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            Value::Quantity(q) => Some(q.to_si()),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Text(s) => Some(s.as_str()),
            Value::Identifier(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Quantity(q) => write!(f, "{}", q),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Text(s) => write!(f, "\"{}\"", s),
            Value::Identifier(s) => write!(f, "{}", s),
            Value::Range(lo, hi) => write!(f, "{}..{}", lo, hi),
            Value::Pair(x, y) => write!(f, "({}, {})", x, y),
            Value::Triple(x, y, z) => write!(f, "({}, {}, {})", x, y, z),
        }
    }
}

// ─── World ───────────────────────────────────────────────────────────────────

/// Parsed `world "name" { ... }` block.
#[derive(Debug, Clone)]
pub struct WorldConfig {
    pub name: String,
    pub props: HashMap<String, Value>,
}

impl WorldConfig {
    /// Grid size as (width, height), defaults to (256, 256).
    pub fn size(&self) -> (u32, u32) {
        if let Some(Value::Pair(w, h)) = self.props.get("size") {
            (*w as u32, *h as u32)
        } else {
            (256, 256)
        }
    }

    /// Tick rate in Hz, defaults to 60.
    pub fn tick_rate(&self) -> f64 {
        self.props.get("tick_rate")
            .and_then(|v| v.as_si())
            .unwrap_or(60.0)
    }

    /// Surface gravity in m/s², defaults to 9.81.
    pub fn gravity(&self) -> f64 {
        self.props.get("gravity")
            .and_then(|v| v.as_si())
            .unwrap_or(9.81)
    }

    /// Ambient temperature in Kelvin, defaults to 288.15 K (15°C).
    pub fn ambient_temperature_k(&self) -> f64 {
        self.props.get("ambient_temperature")
            .and_then(|v| v.as_si())
            .unwrap_or(288.15)
    }
}

// ─── Material ────────────────────────────────────────────────────────────────

/// Parsed `material name [extends parent] { ... }` block.
#[derive(Debug, Clone)]
pub struct MaterialDef {
    pub name: String,
    pub parent: Option<String>,
    pub props: HashMap<String, Value>,
}

// ─── Entity ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum EntityType {
    Humanoid,
    RigidBody,
    Particle,
    Generic,
}

impl EntityType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "humanoid" => EntityType::Humanoid,
            "rigid_body" => EntityType::RigidBody,
            "particle" => EntityType::Particle,
            _ => EntityType::Generic,
        }
    }
}

/// Parsed `entity name { ... }` block (definition / template).
#[derive(Debug, Clone)]
pub struct EntityDef {
    pub name: String,
    pub entity_type: EntityType,
    pub props: HashMap<String, Value>,
}

// ─── Terrain ─────────────────────────────────────────────────────────────────

/// Parsed `terrain name { ... }` block.
#[derive(Debug, Clone)]
pub struct TerrainDef {
    pub name: String,
    pub props: HashMap<String, Value>,
}

// ─── Structure ───────────────────────────────────────────────────────────────

/// Parsed `structure name { ... }` block.
#[derive(Debug, Clone)]
pub struct StructureDef {
    pub name: String,
    pub props: HashMap<String, Value>,
}

// ─── Weather ─────────────────────────────────────────────────────────────────

/// Parsed `weather name { ... }` block.
#[derive(Debug, Clone)]
pub struct WeatherDef {
    pub name: String,
    pub props: HashMap<String, Value>,
}

// ─── Commands ────────────────────────────────────────────────────────────────

/// `spawn <entity_type> "<instance_name>" at (<x>, <y>) { overrides... }`
#[derive(Debug, Clone)]
pub struct SpawnCmd {
    /// The entity definition template name (e.g. "person").
    pub entity_def: String,
    /// The unique instance name (e.g. "Elena").
    pub instance_name: String,
    /// Position on the world grid.
    pub position: Option<(f64, f64)>,
    /// Property overrides for this specific instance.
    pub overrides: HashMap<String, Value>,
}

/// `place <terrain_type> at (<x1>,<y1>)..(<x2>,<y2>)`
#[derive(Debug, Clone)]
pub struct PlaceCmd {
    pub terrain_def: String,
    pub from: (f64, f64),
    pub to: (f64, f64),
}
