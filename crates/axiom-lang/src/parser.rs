use std::collections::HashMap;

use pest::iterators::Pair;
use pest::Parser;

use crate::ast::*;
use crate::units::{parse_unit, Quantity};

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct AxiomParser;

/// All errors that can occur during parsing.
#[derive(Debug)]
pub enum ParseError {
    Pest(Box<pest::error::Error<Rule>>),
    UnknownUnit(String),
    InvalidNumber(String),
    UnexpectedRule(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Pest(e) => write!(f, "Parse error: {}", e),
            ParseError::UnknownUnit(u) => write!(f, "Unknown unit: '{}'", u),
            ParseError::InvalidNumber(s) => write!(f, "Invalid number: '{}'", s),
            ParseError::UnexpectedRule(r) => write!(f, "Unexpected rule: {}", r),
        }
    }
}

impl std::error::Error for ParseError {}

impl From<pest::error::Error<Rule>> for ParseError {
    fn from(e: pest::error::Error<Rule>) -> Self {
        ParseError::Pest(Box::new(e))
    }
}

/// Parse an AxiomLang source string into an `AxiomFile`.
pub fn parse_file(input: &str) -> Result<AxiomFile, ParseError> {
    let pairs = AxiomParser::parse(Rule::file, input)?;
    let mut file = AxiomFile::default();

    for pair in pairs {
        match pair.as_rule() {
            Rule::file => {
                for stmt in pair.into_inner() {
                    parse_statement(stmt, &mut file)?;
                }
            }
            Rule::EOI => {}
            _ => {}
        }
    }

    Ok(file)
}

fn parse_statement(pair: Pair<Rule>, file: &mut AxiomFile) -> Result<(), ParseError> {
    match pair.as_rule() {
        Rule::statement => {
            for inner in pair.into_inner() {
                parse_statement(inner, file)?;
            }
        }
        Rule::world_block => {
            file.world = Some(parse_world_block(pair)?);
        }
        Rule::material_block => {
            file.materials.push(parse_material_block(pair)?);
        }
        Rule::entity_block => {
            file.entities.push(parse_entity_block(pair)?);
        }
        Rule::terrain_block => {
            file.terrains.push(parse_terrain_block(pair)?);
        }
        Rule::structure_block => {
            file.structures.push(parse_structure_block(pair)?);
        }
        Rule::weather_block => {
            file.weathers.push(parse_weather_block(pair)?);
        }
        Rule::spawn_cmd => {
            file.spawn_cmds.push(parse_spawn_cmd(pair)?);
        }
        Rule::place_cmd => {
            file.place_cmds.push(parse_place_cmd(pair)?);
        }
        Rule::EOI => {}
        _ => {}
    }
    Ok(())
}

// ─── Block parsers ───────────────────────────────────────────────────────────

fn parse_world_block(pair: Pair<Rule>) -> Result<WorldConfig, ParseError> {
    let mut inner = pair.into_inner();
    let name_pair = inner.next().unwrap();
    let name = unquote(name_pair.as_str());
    let props = parse_properties(inner)?;
    Ok(WorldConfig { name, props })
}

fn parse_material_block(pair: Pair<Rule>) -> Result<MaterialDef, ParseError> {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();

    let mut parent = None;
    let mut props_pairs = Vec::new();

    for p in inner {
        match p.as_rule() {
            Rule::extends_clause => {
                parent = Some(p.into_inner().next().unwrap().as_str().to_string());
            }
            Rule::property => {
                props_pairs.push(p);
            }
            _ => {}
        }
    }

    let props = parse_property_list(props_pairs)?;
    Ok(MaterialDef {
        name,
        parent,
        props,
    })
}

fn parse_entity_block(pair: Pair<Rule>) -> Result<EntityDef, ParseError> {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let props = parse_properties(inner)?;
    let entity_type = props
        .get("type")
        .and_then(|v| v.as_str())
        .map(EntityType::from_str)
        .unwrap_or(EntityType::Generic);
    Ok(EntityDef {
        name,
        entity_type,
        props,
    })
}

fn parse_terrain_block(pair: Pair<Rule>) -> Result<TerrainDef, ParseError> {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let props = parse_properties(inner)?;
    Ok(TerrainDef { name, props })
}

fn parse_structure_block(pair: Pair<Rule>) -> Result<StructureDef, ParseError> {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let props = parse_properties(inner)?;
    Ok(StructureDef { name, props })
}

fn parse_weather_block(pair: Pair<Rule>) -> Result<WeatherDef, ParseError> {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let props = parse_properties(inner)?;
    Ok(WeatherDef { name, props })
}

fn parse_spawn_cmd(pair: Pair<Rule>) -> Result<SpawnCmd, ParseError> {
    let mut inner = pair.into_inner();
    let entity_def = inner.next().unwrap().as_str().to_string();
    let instance_name = unquote(inner.next().unwrap().as_str());

    let mut position = None;
    let mut overrides = HashMap::new();

    for p in inner {
        match p.as_rule() {
            Rule::spawn_at => {
                let pair_val = p.into_inner().next().unwrap();
                position = Some(parse_pair_val(pair_val)?);
            }
            Rule::property => {
                let (k, v) = parse_property(p)?;
                overrides.insert(k, v);
            }
            _ => {}
        }
    }

    Ok(SpawnCmd {
        entity_def,
        instance_name,
        position,
        overrides,
    })
}

fn parse_place_cmd(pair: Pair<Rule>) -> Result<PlaceCmd, ParseError> {
    let mut inner = pair.into_inner();
    let terrain_def = inner.next().unwrap().as_str().to_string();
    let range_pair = inner.next().unwrap(); // place_range
    let mut range_inner = range_pair.into_inner();
    let from = parse_pair_val(range_inner.next().unwrap())?;
    let to = parse_pair_val(range_inner.next().unwrap())?;
    Ok(PlaceCmd {
        terrain_def,
        from,
        to,
    })
}

// ─── Property helpers ────────────────────────────────────────────────────────

fn parse_properties<'a>(
    pairs: impl Iterator<Item = Pair<'a, Rule>>,
) -> Result<HashMap<String, Value>, ParseError> {
    let mut map = HashMap::new();
    for p in pairs {
        if p.as_rule() == Rule::property {
            let (k, v) = parse_property(p)?;
            map.insert(k, v);
        }
    }
    Ok(map)
}

fn parse_property_list(props: Vec<Pair<Rule>>) -> Result<HashMap<String, Value>, ParseError> {
    let mut map = HashMap::new();
    for p in props {
        let (k, v) = parse_property(p)?;
        map.insert(k, v);
    }
    Ok(map)
}

fn parse_property(pair: Pair<Rule>) -> Result<(String, Value), ParseError> {
    let mut inner = pair.into_inner();
    let key = inner.next().unwrap().as_str().to_string();
    let value_pair = inner.next().unwrap();
    let value = parse_value(value_pair)?;
    Ok((key, value))
}

fn parse_value(pair: Pair<Rule>) -> Result<Value, ParseError> {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::range_val => {
            let mut parts = inner.into_inner();
            let lo: f64 = parts
                .next()
                .unwrap()
                .as_str()
                .parse()
                .map_err(|_| ParseError::InvalidNumber("range lo".into()))?;
            let hi: f64 = parts
                .next()
                .unwrap()
                .as_str()
                .parse()
                .map_err(|_| ParseError::InvalidNumber("range hi".into()))?;
            Ok(Value::Range(lo, hi))
        }
        Rule::triple_val => {
            let mut parts = inner.into_inner();
            let x: f64 = parts
                .next()
                .unwrap()
                .as_str()
                .parse()
                .map_err(|_| ParseError::InvalidNumber("triple x".into()))?;
            let y: f64 = parts
                .next()
                .unwrap()
                .as_str()
                .parse()
                .map_err(|_| ParseError::InvalidNumber("triple y".into()))?;
            let z: f64 = parts
                .next()
                .unwrap()
                .as_str()
                .parse()
                .map_err(|_| ParseError::InvalidNumber("triple z".into()))?;
            Ok(Value::Triple(x, y, z))
        }
        Rule::pair_val => {
            let (x, y) = parse_pair_val(inner)?;
            Ok(Value::Pair(x, y))
        }
        Rule::quantity => parse_quantity(inner),
        Rule::boolean => {
            let b = match inner.into_inner().next().unwrap().as_rule() {
                Rule::bool_true => true,
                _ => false,
            };
            Ok(Value::Boolean(b))
        }
        Rule::string => Ok(Value::Text(unquote(inner.as_str()))),
        Rule::identifier => Ok(Value::Identifier(inner.as_str().to_string())),
        r => Err(ParseError::UnexpectedRule(format!("{:?}", r))),
    }
}

fn parse_quantity(pair: Pair<Rule>) -> Result<Value, ParseError> {
    let mut inner = pair.into_inner();
    let num_str = inner.next().unwrap().as_str();
    let value: f64 = num_str
        .parse()
        .map_err(|_| ParseError::InvalidNumber(num_str.to_string()))?;

    if let Some(unit_pair) = inner.next() {
        let unit_str = unit_pair.as_str();
        match parse_unit(unit_str) {
            Some(unit) => Ok(Value::Quantity(Quantity::new(value, unit))),
            None => Err(ParseError::UnknownUnit(unit_str.to_string())),
        }
    } else {
        Ok(Value::Number(value))
    }
}

fn parse_pair_val(pair: Pair<Rule>) -> Result<(f64, f64), ParseError> {
    let mut inner = pair.into_inner();
    let x: f64 = inner
        .next()
        .unwrap()
        .as_str()
        .parse()
        .map_err(|_| ParseError::InvalidNumber("pair x".into()))?;
    let y: f64 = inner
        .next()
        .unwrap()
        .as_str()
        .parse()
        .map_err(|_| ParseError::InvalidNumber("pair y".into()))?;
    Ok((x, y))
}

fn unquote(s: &str) -> String {
    s.trim_matches('"').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_world_block() {
        let src = r#"
world "test_world" {
    gravity: 9.81
    tick_rate: 60
}
"#;
        let file = parse_file(src).unwrap();
        let world = file.world.unwrap();
        assert_eq!(world.name, "test_world");
        assert!((world.gravity() - 9.81).abs() < 1e-10);
        assert!((world.tick_rate() - 60.0).abs() < 1e-10);
    }

    #[test]
    fn test_parse_material_block() {
        let src = r#"
material granite {
    density: 2750
    static_friction: 0.65
}
"#;
        let file = parse_file(src).unwrap();
        assert_eq!(file.materials.len(), 1);
        let mat = &file.materials[0];
        assert_eq!(mat.name, "granite");
        assert!(mat.parent.is_none());
    }

    #[test]
    fn test_parse_material_extends() {
        let src = r#"
material packed_snow extends fresh_snow {
    density: 400
}
"#;
        let file = parse_file(src).unwrap();
        assert_eq!(file.materials[0].parent, Some("fresh_snow".to_string()));
    }

    #[test]
    fn test_parse_entity_block() {
        let src = r#"
entity person {
    type: humanoid
    mass: 75
}
"#;
        let file = parse_file(src).unwrap();
        assert_eq!(file.entities.len(), 1);
        assert_eq!(file.entities[0].name, "person");
    }

    #[test]
    fn test_parse_spawn_cmd() {
        let src = r#"
spawn person "Elena" at (120, 200) {
    mass: 62
}
"#;
        let file = parse_file(src).unwrap();
        assert_eq!(file.spawn_cmds.len(), 1);
        let cmd = &file.spawn_cmds[0];
        assert_eq!(cmd.entity_def, "person");
        assert_eq!(cmd.instance_name, "Elena");
        assert_eq!(cmd.position, Some((120.0, 200.0)));
    }

    #[test]
    fn test_parse_quantity_with_unit() {
        let src = r#"
material ice {
    density: 917
    thermal_conductivity: 2.18
    melting_point: 0
}
"#;
        let file = parse_file(src).unwrap();
        assert!(file.materials[0].props.contains_key("density"));
    }

    #[test]
    fn test_parse_place_cmd() {
        let src = r#"
place meadow at (0,0)..(500,500)
"#;
        let file = parse_file(src).unwrap();
        assert_eq!(file.place_cmds.len(), 1);
        let cmd = &file.place_cmds[0];
        assert_eq!(cmd.terrain_def, "meadow");
        assert_eq!(cmd.from, (0.0, 0.0));
        assert_eq!(cmd.to, (500.0, 500.0));
    }
}
