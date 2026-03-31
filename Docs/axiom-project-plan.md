# AXIOM — Terminal-Based Open-World Physics Simulation Framework

> *"From first principles, simulate everything."*

**Repository:** `github.com/[tkrenn12306]/axiom`
**Lizenz:** MIT
**Sprache:** Rust
**Status:** Pre-Alpha / Konzeptphase

---

## 1. Vision & Projektbeschreibung

**Axiom** ist ein Open-Source-Framework, das über das Terminal bedient wird und es ermöglicht, physikalisch korrekte Open-World-Simulationen zu erstellen, zu konfigurieren und in Echtzeit zu beobachten. Es bringt eine eigene deklarative Sprache mit (**AxiomLang**, Dateiendung `.ax`), über die Welten, Terrain, Wetter, Personen, Gegenstände und physikalische Gesetze definiert werden.

Im Gegensatz zu grafischen Engines (Unity, Unreal, Godot) oder reinen Physik-Bibliotheken (Box2D, Rapier) bietet Axiom:

- **Kein GUI nötig** — alles läuft im Terminal (ASCII/Unicode-Rendering + TUI)
- **Echte Physik** — keine Approximation, sondern korrekte Newtonian Mechanics, Thermodynamik, Fluiddynamik (vereinfacht), Materialphysik
- **Deklarative Konfiguration** — Welten werden in `.ax`-Dateien beschrieben und sind teilbar
- **Composable** — jedes physikalische System ist ein Modul, das ein- und ausgeschaltet werden kann
- **Headless-fähig** — Simulationen können ohne Rendering laufen (für KI-Training, Batch-Simulation, Tests)

### Zielgruppen

| Zielgruppe | Use Case |
|---|---|
| Physik-Studierende & Lehrende | Physikalische Phänomene simulieren und beobachten |
| Game-Designer | Rapid Prototyping von Spielmechaniken |
| KI-Forscher | Environments für Reinforcement-Learning-Agenten |
| Hobbyisten & Worldbuilder | Welten bauen, Szenarien durchspielen |
| Ingenieure | Schnelle "What-if"-Simulationen für Materialien, Kräfte, Thermik |

---

## 2. Physikalische Systeme — Vollständige Übersicht

Axiom implementiert echte physikalische Gesetze, organisiert in unabhängige, aber miteinander interagierende Systeme.

### 2.1 Klassische Mechanik (Newtonian Mechanics)

Das Fundament aller Bewegung in Axiom.

**Gesetze:**
- Newtons 1. Gesetz (Trägheit): Objekte behalten ihren Bewegungszustand bei, solange keine Kraft wirkt
- Newtons 2. Gesetz: **F = m · a** — Kraft erzeugt Beschleunigung proportional zur Masse
- Newtons 3. Gesetz (Actio = Reactio): Jede Kraft erzeugt eine gleich große Gegenkraft
- Impulserhaltung: **p = m · v**, in geschlossenen Systemen bleibt der Gesamtimpuls erhalten
- Drehimpulserhaltung: **L = I · ω**, Rotationsträgheit bleibt erhalten

**Implementierung:**
```
Für jede Entity pro Tick:
  1. Alle wirkenden Kräfte summieren (Gravitation, Reibung, Wind, Kontakt, ...)
  2. F_net = Σ F_i
  3. a = F_net / m
  4. v_new = v_old + a · Δt
  5. pos_new = pos_old + v_new · Δt
  (Verlet-Integration oder RK4 für höhere Genauigkeit)
```

**Konfigurierbare Parameter pro Entity:**
- `mass` (kg) — Masse
- `velocity` (m/s) — Geschwindigkeitsvektor (x, y, z)
- `acceleration` (m/s²) — Beschleunigungsvektor
- `moment_of_inertia` (kg·m²) — Rotationsträgheit
- `angular_velocity` (rad/s) — Winkelgeschwindigkeit
- `elasticity` (0-1) — Rückprall bei Kollision (0 = plastisch, 1 = perfekt elastisch)
- `static_friction` — Haftreibungskoeffizient
- `kinetic_friction` — Gleitreibungskoeffizient

### 2.2 Gravitation

**Gesetze:**
- Lokale Gravitation: **F_g = m · g** (Standardwert g = 9.81 m/s², konfigurierbar)
- Optionale Newtonian Gravitation zwischen Körpern: **F = G · (m₁ · m₂) / r²**
- Gravitationsfeld-Variationen basierend auf Höhe: **g(h) = g₀ · (R / (R + h))²**

**Konfiguration:**
```axiom
world "testworld" {
  gravity: 9.81           // Oberflächen-Beschleunigung (m/s²)
  gravity_model: local     // "local" (konstant) oder "newtonian" (r²-Abhängigkeit)
  planet_radius: 6371km    // Für höhenabhängige Gravitation
}
```

### 2.3 Reibung & Oberflächenphysik

**Gesetze:**
- Haftreibung: **F_s ≤ μ_s · F_n** (Objekt bewegt sich nicht, solange angelegte Kraft kleiner)
- Gleitreibung: **F_k = μ_k · F_n** (konstante Kraft entgegen der Bewegungsrichtung)
- Rollreibung: **F_r = μ_r · F_n / R** (für rollende Objekte)
- Luftwiderstand: **F_d = ½ · ρ · v² · C_d · A** (quadratisch zur Geschwindigkeit)

**Terrain-Materialien mit physikalischen Eigenschaften:**
```axiom
material ice {
  static_friction: 0.03
  kinetic_friction: 0.01
  rolling_friction: 0.005
  hardness: 3.0          // Mohs-Skala
  density: 917            // kg/m³
  thermal_conductivity: 2.18  // W/(m·K)
  melting_point: 0        // °C
  specific_heat: 2090     // J/(kg·K)
}

material wet_soil {
  static_friction: 0.35
  kinetic_friction: 0.25
  rolling_friction: 0.08
  density: 1600
  moisture_capacity: 0.45   // Maximale Sättigung (0-1)
  permeability: 1e-5        // m/s (Darcy-Durchlässigkeit)
  load_bearing: 150         // kPa (Tragfähigkeit)
}

material sand {
  static_friction: 0.55
  kinetic_friction: 0.40
  angle_of_repose: 34       // Grad — Schüttwinkel
  density: 1600
  is_granular: true          // Aktiviert Granulardynamik
}
```

### 2.4 Thermodynamik

**Gesetze:**
- 1. Hauptsatz: **ΔU = Q - W** (Energieerhaltung)
- Wärmeleitung (Fourier): **q = -k · ∇T** (Wärmefluss proportional zum Temperaturgradienten)
- Wärmekonvektion: **Q = h · A · (T_surface - T_fluid)**
- Wärmestrahlung (Stefan-Boltzmann): **P = ε · σ · A · T⁴**
- Phasenübergänge: Schmelzen, Verdampfen, Gefrieren mit korrekter latenter Wärme

**Implementierung:**
```
Für jede Entity/Terrainzelle pro Tick:
  1. Wärmeleitung zu benachbarten Zellen berechnen
  2. Konvektion (Wind-Kühlung) anwenden
  3. Solare Einstrahlung (Tag/Nacht-Zyklus) addieren
  4. Phasenübergänge prüfen:
     - Wasser bei T < 0°C → Eis (latente Wärme: 334 kJ/kg)
     - Eis bei T > 0°C → Wasser
     - Wasser bei T > 100°C → Dampf (latente Wärme: 2260 kJ/kg)
  5. Neue Temperatur berechnen: ΔT = Q / (m · c)
```

**Konfiguration:**
```axiom
entity campfire {
  temperature: 800          // °C
  heat_output: 5000         // W
  radiation_radius: 10m
  fuel_consumption: 0.5kg/h
  fuel_remaining: 5kg
  ignites_at_distance: 0.5m
}

// Entities reagieren auf Temperatur
entity person {
  body_temperature: 37      // °C
  comfort_range: 18..28     // °C
  hypothermia_at: 35        // °C (Kerntemperatur)
  hyperthermia_at: 40       // °C
  clothing_insulation: 2.5  // CLO-Einheit
}
```

### 2.5 Fluiddynamik (vereinfacht)

Keine volle Navier-Stokes-Lösung (zu rechenintensiv für Echtzeit), sondern ein Shallow-Water-Modell + Partikel-basierte Strömung.

**Modelle:**
- Shallow-Water-Equations für Gewässer, Überflutungen, Regenwasser-Abfluss
- Darcys Gesetz für Grundwasser/Bodenpermeabilität: **Q = -k · A · (Δh / L)**
- Bernoulli-Gleichung für einfache Strömungen: **p + ½ρv² + ρgh = const**
- Manning-Formel für Fließgeschwindigkeit in Gerinnen

**Konfiguration:**
```axiom
body_of_water river {
  flow_rate: 2.5            // m/s
  depth: 1.2m
  temperature: 12           // °C
  current_direction: SE
  manning_coefficient: 0.035 // Rauheit des Flussbetts
}

// Regen erzeugt Oberflächenabfluss
weather rain {
  intensity: 15              // mm/h
  affects terrain {
    moisture += intensity * dt / 1000
    if moisture > moisture_capacity {
      spawn surface_runoff direction: terrain.slope
    }
  }
}
```

### 2.6 Atmosphärische Physik & Wetter

**Modelle:**
- Windfeld: Vektorfeld über die gesamte Karte, beeinflusst durch Terrain (Düseneffekt in Tälern, Leewellen hinter Bergen)
- Temperaturprofil: Basierend auf Höhe, Tageszeit, Sonneneinstrahlung, Windchill
- Niederschlag: Regen, Schnee, Hagel basierend auf Temperatur und Feuchtigkeit
- Beaufort-Skala für Windeffekte auf Entities und Strukturen

**Windkraft auf Objekte:**
```
F_wind = ½ · ρ_air · v_wind² · C_d · A_frontal
```
Wobei:
- `ρ_air` = 1.225 kg/m³ (Luftdichte auf Meereshöhe)
- `v_wind` = Windgeschwindigkeit relativ zum Objekt
- `C_d` = Widerstandsbeiwert des Objekts (Kugel: 0.47, Zylinder: 1.17, Person: 1.0-1.3)
- `A_frontal` = Angriffsfläche senkrecht zur Windrichtung

**Konfiguration:**
```axiom
weather blizzard {
  wind_speed: 90             // km/h
  wind_direction: NW
  wind_gusts: 120            // km/h (Böenspitzen)
  temperature: -15           // °C
  precipitation: snow
  precipitation_rate: 30     // mm/h Wasseräquivalent
  visibility: 50m
  
  // Windchill nach ISO 11079
  windchill: fn(T, v) = 13.12 + 0.6215*T - 11.37*v^0.16 + 0.3965*T*v^0.16
  
  affects entity {
    if mass < 60kg and wind_speed > 100km/h {
      apply_force(wind_direction, F_wind)  // Kann umgeworfen werden
    }
    movement_speed *= max(0.1, 1.0 - wind_speed/150)
    body_temperature -= windchill_effect * dt
  }
  
  affects structure {
    if integrity < 0.3 and wind_speed > 80km/h {
      trigger collapse
    }
    integrity -= (wind_speed / 200)^2 * dt
  }
}
```

### 2.7 Kollisionserkennung & Response

**Algorithmen:**
- Broad Phase: Spatial Hashing oder AABB-Tree für schnelles Filtern
- Narrow Phase: GJK-Algorithmus für konvexe Formen, SAT für einfache Shapes
- Collision Response: Impulsbasierte Auflösung mit Elastizität und Reibung

**Kollisionstypen:**
```
Entity ↔ Entity:     Impulsaustausch, elastisch/plastisch
Entity ↔ Terrain:    Reibung, Hangabtrieb, Einsinken (Sand/Schnee)
Entity ↔ Structure:  Kraft auf Strukturintegrität, Rückprall
Projectile ↔ *:      Penetration basierend auf Energie und Material
Fluid ↔ Terrain:     Erosion, Überflutung
```

### 2.8 Materialphysik & Strukturmechanik

**Modelle:**
- Elastizität (Hooke): **σ = E · ε** (Spannung proportional zur Dehnung)
- Bruchgrenze: Materialien versagen bei Überschreitung der Zugfestigkeit
- Ermüdung: Wiederholte Belastung unterhalb der Bruchgrenze kann zu Versagen führen
- Brandverhalten: Materialien haben Flammpunkt, Brenndauer, Wärmefreisetzung

**Konfiguration:**
```axiom
material oak_wood {
  density: 700              // kg/m³
  youngs_modulus: 12.5      // GPa
  tensile_strength: 100     // MPa
  compressive_strength: 50  // MPa
  flash_point: 300          // °C
  burn_rate: 0.7            // mm/min
  thermal_conductivity: 0.17
  specific_heat: 2400
}

structure bridge {
  material: oak_wood
  dimensions: 15m x 3m x 0.3m
  supports: [(0, 0), (15, 0)]
  max_load: fn(material, dimensions, supports)  // Berechnet aus Balkenbiegung
  integrity: 1.0
  
  degrades_when {
    load > max_load * 0.8: fatigue += 0.001/tick
    temperature > flash_point: burning = true
    moisture > 0.3: integrity -= 0.0001/tick  // Fäulnis
  }
}
```

### 2.9 Optik & Beleuchtung (vereinfacht)

Kein Raytracing, aber physikalisch motivierte Sichtbarkeit:
- Sichtweite basierend auf Wetter (Nebel, Regen, Blizzard)
- Tag/Nacht-Zyklus mit korrekter Sonnenposition
- Schatten durch Terrain-Elevation (Berge blockieren Licht)
- Lichtquellen (Feuer, Lampen) mit realistischem Abfall: **I = I₀ / r²**

### 2.10 Akustik (optional, spätere Phase)

- Schall breitet sich mit **~343 m/s** aus (temperaturabhängig)
- Lautstärke fällt mit **1/r²** ab
- Terrain und Strukturen blockieren/reflektieren Schall
- Entities können Geräusche "hören" innerhalb ihres Hörradius

---

## 3. AxiomLang — Die Simulationssprache

### 3.1 Design-Prinzipien

1. **Deklarativ** — Man beschreibt WAS existiert, nicht WIE es simuliert wird
2. **Physik-nativ** — Einheiten sind Teil der Syntax (`5kg`, `10m/s`, `25°C`)
3. **Composable** — Materialien, Entities, Weathers sind wiederverwendbare Blöcke
4. **Lesbar** — Ein `.ax`-File liest sich wie eine Weltbeschreibung

### 3.2 Einheiten-System

AxiomLang hat ein eingebautes Einheitensystem mit automatischer Konvertierung:

```axiom
// Masse
5kg, 5000g, 11.02lb

// Länge
10m, 1000cm, 10.94yd, 32.81ft

// Geschwindigkeit
5m/s, 18km/h, 11.18mph

// Kraft
100N, 100kg*m/s²

// Temperatur
20°C, 68°F, 293.15K

// Druck
101325Pa, 1013.25hPa, 1atm

// Energie
1000J, 1kJ, 0.239kcal

// Automatische Konvertierung
let speed = 100km/h in m/s   // → 27.78 m/s
let temp = 72°F in °C         // → 22.22°C
```

### 3.3 Vollständige Syntax-Referenz

```axiom
// ═══════════════════════════════════════════════════
// WORLD DEFINITION
// ═══════════════════════════════════════════════════

world "alpine_simulation" {
  size: 1024 x 1024           // Grid-Auflösung
  cell_size: 1m               // Reale Größe pro Zelle
  dimensions: 2d              // "2d" oder "3d"
  tick_rate: 60hz             // Simulations-Ticks pro Sekunde
  gravity: 9.81m/s²
  gravity_model: local        // "local" | "newtonian"
  air_density: 1.225kg/m³
  air_viscosity: 1.81e-5      // Pa·s
  ambient_temperature: 15°C
  time: 2026-06-15T06:00
  time_scale: 1.0             // 1.0 = Echtzeit, 10.0 = 10x schneller
  seed: 42                    // Für deterministische Simulationen
  
  // Physik-Module aktivieren/deaktivieren
  modules: [
    mechanics,
    thermodynamics,
    fluid_dynamics,
    weather,
    material_physics,
    acoustics
  ]
}

// ═══════════════════════════════════════════════════
// MATERIALS
// ═══════════════════════════════════════════════════

material granite {
  density: 2750kg/m³
  youngs_modulus: 70GPa
  compressive_strength: 200MPa
  tensile_strength: 15MPa
  static_friction: 0.65
  kinetic_friction: 0.55
  thermal_conductivity: 2.5W/(m*K)
  specific_heat: 790J/(kg*K)
  hardness: 7.0               // Mohs
  porosity: 0.01
  color: "#808080"             // Für Rendering
}

material fresh_snow {
  density: 100kg/m³
  static_friction: 0.10
  kinetic_friction: 0.05
  thermal_conductivity: 0.05
  specific_heat: 2090
  melting_point: 0°C
  latent_heat_fusion: 334kJ/kg
  albedo: 0.85                 // Reflektivität (0-1)
  compressible: true
  compression_factor: 0.3      // Verdichtet sich unter Last
}

// Materialien können voneinander erben
material packed_snow extends fresh_snow {
  density: 400kg/m³
  static_friction: 0.20
  kinetic_friction: 0.10
  compression_factor: 0.8
}

// ═══════════════════════════════════════════════════
// TERRAIN
// ═══════════════════════════════════════════════════

terrain meadow {
  base_material: soil
  surface_material: grass
  elevation_range: 400..500m
  moisture: 0.3
  vegetation_density: 0.7
  load_bearing: 200kPa
  
  // Terrain reagiert auf Physik
  on rain {
    moisture += rain.intensity * dt
    if moisture > 0.6 {
      surface_friction *= 0.6    // Matschig
      load_bearing *= 0.5
    }
  }
  
  on temperature < 0°C {
    surface_material = frozen_soil
    surface_friction = 0.15
  }
  
  on temperature < -5°C and moisture > 0.2 {
    surface_material = ice
    spawn frost_layer thickness: moisture * 2cm
  }
}

terrain mountain_slope {
  base_material: granite
  surface_material: gravel
  elevation_range: 800..2500m
  slope_angle: 20..45deg
  
  on rain and slope_angle > 30deg {
    // Erdrutsch-Risiko
    if moisture > 0.7 and duration > 2h {
      trigger landslide {
        volume: moisture * area * 0.1
        direction: slope.downhill
        speed: fn(slope_angle, moisture)
      }
    }
  }
}

// Terrain platzieren
place meadow at (0,0)..(500,500)
place mountain_slope at (500,0)..(1024,400) {
  elevation: heightmap("alps_dem.png")    // Höhenkarte importieren
}

// ═══════════════════════════════════════════════════
// ENTITIES — Personen
// ═══════════════════════════════════════════════════

entity person {
  type: humanoid
  mass: 75kg
  height: 1.78m
  width: 0.45m                 // Für Kollisionsbox & Windangriffsfläche
  drag_coefficient: 1.15
  
  // Physikalische Grenzen
  max_force: 800N              // Maximale Muskelkraft
  max_speed: fn(terrain) {
    base: 5km/h
    * terrain.surface_friction  // Eis = langsamer
    * (1.0 - wind.headwind / 150km/h)  // Wind bremst
    * energy_factor             // Müdigkeit
  }
  
  // Biophysik
  body_temperature: 37°C
  metabolic_rate: 80W          // Grundumsatz Wärmeproduktion
  sweat_rate: 0               // Erhöht sich bei Hitze
  clothing_insulation: 2.0clo
  
  comfort_range: 18..28°C
  hypothermia_threshold: 35°C
  hyperthermia_threshold: 40°C
  
  // Thermoregulation (berechnet pro Tick)
  thermal_balance: fn() {
    heat_produced = metabolic_rate + activity_heat
    heat_lost_convection = h * body_surface_area * (skin_temp - air_temp)
    heat_lost_radiation = emissivity * stefan_boltzmann * body_surface_area * (skin_temp^4 - env_temp^4)
    heat_lost_evaporation = sweat_rate * latent_heat_vaporization
    
    net = heat_produced - heat_lost_convection - heat_lost_radiation - heat_lost_evaporation
    body_temperature += net / (mass * specific_heat_body) * dt
  }
  
  // Inventar mit physikalischen Eigenschaften
  inventory: [] {
    max_carry: 40kg            // Basierend auf Muskelkraft
    affects mass: + sum(item.mass)
    affects drag: + sum(item.frontal_area) * 0.5
  }
  
  // AI Behavior (optional)
  behavior: {
    if body_temperature < hypothermia_threshold -> seek_heat_source()
    if body_temperature > hyperthermia_threshold -> seek_shade(), increase_sweat()
    if on_terrain.is(ice) -> reduce_speed(0.3), careful_movement()
    if wind.speed > 60km/h -> brace(), seek_shelter()
    if hunger > 0.8 -> find_food()
    default -> follow_schedule()
  }
}

// Instanz erzeugen
spawn person "Elena" at (120, 200) {
  mass: 62kg
  height: 1.65m
  clothing_insulation: 3.5clo   // Winterkleidung
  inventory: [backpack(5kg), water_bottle(1kg)]
}

// ═══════════════════════════════════════════════════
// ENTITIES — Gegenstände
// ═══════════════════════════════════════════════════

entity boulder {
  type: rigid_body
  shape: sphere(radius: 0.8m)
  material: granite
  mass: fn(shape, material) = volume * material.density  // Automatisch berechnet
  
  on slope {
    if terrain.slope_angle > material.static_friction.to_angle() {
      start_rolling(direction: slope.downhill)
    }
  }
}

entity wooden_crate {
  type: rigid_body
  shape: box(1m x 1m x 1m)
  material: pine_wood
  mass: 35kg
  
  breakable: true
  break_force: 5000N           // Zerstörungskraft
  
  on impact(force) {
    if force > break_force {
      destroy()
      spawn debris count: 8 {
        mass: self.mass / 8
        velocity: random_scatter(impact.direction, 30deg)
      }
    }
  }
  
  on fire {
    if temperature > material.flash_point {
      burning = true
      heat_output = mass * material.heat_of_combustion * burn_rate
      mass -= burn_rate * dt    // Masse nimmt ab
      if mass < 0.1 * original_mass {
        destroy()
        spawn ash mass: original_mass * 0.05
      }
    }
  }
}

entity projectile {
  type: particle
  shape: sphere(radius: 5mm)
  material: steel
  mass: 8g
  
  // Ballistische Flugbahn mit Luftwiderstand
  physics: {
    gravity: true
    drag: true                  // F_d = ½ρv²CdA
    drag_coefficient: 0.295     // Kugel
    spin: optional              // Magnus-Effekt wenn aktiviert
  }
  
  on impact(target) {
    let kinetic_energy = 0.5 * mass * velocity²
    let penetration = kinetic_energy / (target.material.hardness * impact_area)
    
    if penetration > target.thickness {
      pass_through(energy_loss: target.thickness * target.material.hardness)
    } else {
      embed_in(target)
      target.integrity -= kinetic_energy / target.material.toughness
    }
  }
}

// ═══════════════════════════════════════════════════
// STRUCTURES — Gebäude & Bauwerke
// ═══════════════════════════════════════════════════

structure log_cabin {
  dimensions: 8m x 6m x 3.5m
  material: oak_wood
  wall_thickness: 0.2m
  roof_material: clay_tiles
  roof_angle: 35deg
  
  // Strukturelle Integrität
  integrity: 1.0
  structural_model: simple_beam   // "simple_beam" | "truss" | "frame"
  
  // Wind-Belastung (nach Eurocode-vereinfacht)
  wind_load: fn(wind) {
    pressure = 0.5 * air_density * wind.speed²
    force = pressure * frontal_area * shape_factor
    return force
  }
  
  // Schneelast auf Dach
  snow_load: fn(snow_depth) {
    load = snow_depth * snow.density * roof_area * cos(roof_angle)
    if load > max_roof_load {
      trigger roof_collapse
    }
  }
  
  // Thermische Eigenschaften
  insulation: fn(wall_thickness, material) {
    R_value = wall_thickness / material.thermal_conductivity
    return R_value
  }
  interior_temperature: fn(outside_temp, insulation, heat_sources) {
    // Wärmegleichgewicht innen
  }
  
  provides_shelter: true
  capacity: 8
}

// ═══════════════════════════════════════════════════
// WETTER & ATMOSPHÄRE
// ═══════════════════════════════════════════════════

weather clear {
  cloud_cover: 0.1
  precipitation: none
  wind_speed: 5km/h
  wind_direction: W
  humidity: 0.4
  solar_radiation: fn(time, latitude, cloud_cover)
}

weather thunderstorm {
  cloud_cover: 1.0
  precipitation: rain
  precipitation_rate: 40mm/h
  wind_speed: 60km/h
  wind_gusts: 100km/h
  wind_direction: NW
  humidity: 0.95
  temperature_drop: -8°C       // Relativ zur Basistemperatur
  
  lightning: {
    frequency: every 15..45s
    strike_height_preference: max  // Trifft hohe Punkte bevorzugt
    energy: 1e9J               // Pro Blitz
    fire_chance: 0.3            // 30% Chance auf Brand bei Treffer
    radius: 500m               // Einschlagradius um Stormcenter
  }
  
  hail: optional {
    size: 1..3cm
    density: 900kg/m³
    terminal_velocity: fn(size) = sqrt(2 * mass * g / (Cd * rho_air * A))
    damage: fn(kinetic_energy, target.material)
  }
}

// Wetter-Schedule
schedule clear from 06:00 to 14:00
schedule thunderstorm from 14:00 to 18:00 {
  buildup: 30min              // Langsamer Übergang
  peak: 15:30..16:30
  dissipation: 45min
}

// ═══════════════════════════════════════════════════
// EVENTS & TRIGGERS
// ═══════════════════════════════════════════════════

event avalanche {
  trigger_when {
    terrain.type == mountain_slope
    and snow_depth > 0.8m
    and (
      temperature.rising_rate > 2°C/h
      or vibration > threshold
      or slope_angle > 35deg and new_snow > 0.3m
    )
  }
  
  effect {
    let volume = affected_area * snow_depth * 0.6
    let mass = volume * snow.density
    spawn snow_mass {
      mass: mass
      velocity: slope.downhill * 5m/s
      acceleration: g * sin(slope_angle) - friction
      // Wächst durch Mitreißen
      on_contact(snow) { absorb(snow) }
      on_contact(entity) { apply_force(velocity * mass) }
      on_contact(structure) { apply_force(velocity * mass) }
    }
  }
}

event earthquake {
  magnitude: 5.5               // Richter-Skala
  epicenter: (400, 300)
  depth: 10km
  
  effect {
    // Seismische Wellen
    let intensity_at = fn(distance) {
      magnitude * 10^(-0.5 * distance/100)
    }
    
    for_each structure in range(epicenter, 500m) {
      let local_intensity = intensity_at(distance(structure, epicenter))
      structure.integrity -= local_intensity * vulnerability_factor
      if structure.integrity < 0 { trigger collapse }
    }
    
    for_each terrain.is(mountain_slope) {
      if intensity_at(distance) > landslide_threshold {
        trigger landslide
      }
    }
  }
}

// ═══════════════════════════════════════════════════
// REPL COMMANDS (Terminal-Interaktion)
// ═══════════════════════════════════════════════════

// In der interaktiven Session:
> load "alpine_simulation.ax"
> run

> inspect Elena
> inspect terrain at (120, 200)
> measure force on bridge
> measure temperature at (50, 50)

> spawn boulder at (600, 100) velocity: (0, -5m/s)
> trigger earthquake magnitude: 4.0 at (500, 250)
> set weather thunderstorm

> time.speed 10x
> time.pause
> time.step 1               // 1 Tick vorwärts

> snapshot save "before_storm"
> snapshot load "before_storm"
> snapshot diff "before_storm" "after_storm"

> plot temperature at (120, 200) over 1h
> plot entity.Elena.body_temperature over 30min
> export csv "simulation_data.csv" entities: [Elena] fields: [position, velocity, temperature]
```

---

## 4. Architektur

### 4.1 Entity-Component-System (ECS)

Axiom nutzt eine ECS-Architektur (wie in Game-Engines üblich), weil sie:
- Maximal performant ist (cache-friendly, data-oriented)
- Modular erweiterbar ist (neues System = neues Feature)
- Parallelisierbar ist (unabhängige Systeme laufen parallel)

```
COMPONENTS (Daten):
├── Position          { x: f64, y: f64, z: f64 }
├── Velocity          { vx: f64, vy: f64, vz: f64 }
├── Mass              { value: f64 }
├── Shape             { geometry: Geometry, dimensions: Vec3 }
├── Material          { properties: MaterialProps }
├── Temperature       { value: f64, heat_capacity: f64 }
├── Integrity         { value: f64, max: f64 }
├── Inventory         { items: Vec<EntityId>, max_mass: f64 }
├── AI                { behavior_tree: BehaviorTree, state: AIState }
├── Burnable          { flash_point: f64, burn_rate: f64, is_burning: bool }
├── Terrain           { cell_data: TerrainGrid }
├── WindAffected      { drag_coeff: f64, frontal_area: f64 }
├── Shelter           { capacity: u32, insulation: f64 }
└── ... (erweiterbar durch Plugins)

SYSTEMS (Logik, laufen pro Tick):
├── GravitySystem         — Wendet Gravitationskraft an
├── ForceAccumulatorSystem — Summiert alle Kräfte pro Entity
├── IntegrationSystem     — Berechnet neue Position/Velocity (Verlet/RK4)
├── CollisionBroadPhase   — Spatial Hashing, findet potentielle Kollisionen
├── CollisionNarrowPhase  — Exakte Kollisionserkennung (GJK/SAT)
├── CollisionResponse     — Impulsauflösung, Reibung
├── FrictionSystem        — Terrain-basierte Reibungskräfte
├── WindSystem            — Berechnet Windfeld, wendet Windkraft an
├── WeatherSystem         — Updated Wetter, Niederschlag, Temperatur
├── ThermodynamicsSystem  — Wärmeleitung, Konvektion, Strahlung
├── PhaseTransitionSystem — Gefrieren, Schmelzen, Verdampfen
├── TerrainUpdateSystem   — Bodenfeuchte, Erosion, Frost
├── FluidSystem           — Shallow-Water, Strömung, Überflutung
├── MaterialStressSystem  — Strukturbelastung, Ermüdung, Bruch
├── FireSystem            — Brandausbreitung, Wärmeabgabe
├── AISystem              — Behavior Trees evaluieren
├── EventSystem           — Trigger prüfen, Events auslösen
└── RenderSystem          — Terminal-Output aktualisieren
```

### 4.2 Tick-Pipeline

```
┌─────────────────────────────────────────────────────────────┐
│                    AXIOM TICK PIPELINE                       │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  1. INPUT          Parse REPL-Commands, Events              │
│       ↓                                                     │
│  2. FORCES         Gravity + Wind + Contact + Custom        │
│       ↓                                                     │
│  3. INTEGRATION    Position & Velocity update (RK4/Verlet)  │
│       ↓                                                     │
│  4. COLLISION      Broad Phase → Narrow Phase → Response    │
│       ↓                                                     │
│  5. CONSTRAINTS    Joints, Ropes, Structural connections    │
│       ↓                                                     │
│  6. ENVIRONMENT    Weather → Terrain → Fluid → Thermo       │
│       ↓                                                     │
│  7. MATERIALS      Stress → Fatigue → Phase Transitions     │
│       ↓                                                     │
│  8. EVENTS         Trigger-Evaluation (Avalanche, Fire...)  │
│       ↓                                                     │
│  9. AI             Behavior Trees → Pathfinding → Actions   │
│       ↓                                                     │
│  10. RENDER        Terminal-UI Update (if not headless)      │
│       ↓                                                     │
│  11. RECORD        Metrics, Snapshots, CSV-Export            │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 4.3 Projektstruktur

```
axiom/
├── Cargo.toml
├── LICENSE                   # MIT
├── README.md
├── CONTRIBUTING.md
├── docs/
│   ├── physics/              # Dokumentation aller physikalischen Modelle
│   │   ├── mechanics.md
│   │   ├── thermodynamics.md
│   │   ├── fluids.md
│   │   └── materials.md
│   ├── language/             # AxiomLang Spezifikation
│   │   ├── syntax.md
│   │   ├── units.md
│   │   └── stdlib.md
│   └── architecture/         # Technische Architektur-Docs
│       ├── ecs.md
│       └── tick_pipeline.md
├── crates/
│   ├── axiom-core/           # ECS, Tick-Engine, Event-Bus
│   │   ├── src/
│   │   │   ├── ecs/
│   │   │   │   ├── world.rs
│   │   │   │   ├── entity.rs
│   │   │   │   ├── component.rs
│   │   │   │   └── system.rs
│   │   │   ├── tick.rs
│   │   │   ├── events.rs
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   ├── axiom-lang/           # Parser & Interpreter für AxiomLang
│   │   ├── src/
│   │   │   ├── lexer.rs
│   │   │   ├── parser.rs
│   │   │   ├── ast.rs
│   │   │   ├── units.rs      # Einheiten-System
│   │   │   ├── evaluator.rs
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   ├── axiom-physics/        # Alle Physik-Systeme
│   │   ├── src/
│   │   │   ├── mechanics/
│   │   │   │   ├── forces.rs
│   │   │   │   ├── integration.rs   # Verlet, RK4
│   │   │   │   ├── collision.rs
│   │   │   │   └── friction.rs
│   │   │   ├── thermo/
│   │   │   │   ├── conduction.rs
│   │   │   │   ├── convection.rs
│   │   │   │   ├── radiation.rs
│   │   │   │   └── phase_transitions.rs
│   │   │   ├── fluids/
│   │   │   │   ├── shallow_water.rs
│   │   │   │   ├── groundwater.rs
│   │   │   │   └── erosion.rs
│   │   │   ├── atmosphere/
│   │   │   │   ├── wind.rs
│   │   │   │   ├── weather.rs
│   │   │   │   └── precipitation.rs
│   │   │   ├── materials/
│   │   │   │   ├── stress.rs
│   │   │   │   ├── fracture.rs
│   │   │   │   └── fire.rs
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   ├── axiom-ai/             # Behavior Trees, Pathfinding
│   │   ├── src/
│   │   │   ├── behavior_tree.rs
│   │   │   ├── pathfinding.rs  # A*, Flow Fields
│   │   │   ├── needs.rs        # Hunger, Temperature, Energy
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   ├── axiom-render/         # Terminal-Rendering
│   │   ├── src/
│   │   │   ├── tui.rs         # ratatui-basiert
│   │   │   ├── map_view.rs    # Top-Down-Karte
│   │   │   ├── detail_view.rs # Entity-Details
│   │   │   ├── chart_view.rs  # Echtzeit-Diagramme
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   └── axiom-cli/            # CLI & REPL
│       ├── src/
│       │   ├── main.rs
│       │   ├── repl.rs
│       │   ├── commands.rs
│       │   └── config.rs
│       └── Cargo.toml
├── stdlib/                   # Standard-Bibliothek (.ax Dateien)
│   ├── materials/
│   │   ├── metals.ax
│   │   ├── woods.ax
│   │   ├── stones.ax
│   │   ├── soils.ax
│   │   └── fluids.ax
│   ├── entities/
│   │   ├── humanoid.ax
│   │   ├── animals.ax
│   │   └── vehicles.ax
│   ├── structures/
│   │   ├── buildings.ax
│   │   └── bridges.ax
│   ├── weather/
│   │   ├── temperate.ax
│   │   ├── arctic.ax
│   │   └── tropical.ax
│   └── terrains/
│       ├── european_alpine.ax
│       └── coastal.ax
├── scenarios/                # Fertige Beispiel-Simulationen
│   ├── medieval_village.ax
│   ├── avalanche_study.ax
│   ├── bridge_stress_test.ax
│   ├── forest_fire.ax
│   └── flood_simulation.ax
└── tests/
    ├── physics/              # Unit-Tests für physikalische Korrektheit
    │   ├── test_free_fall.rs
    │   ├── test_friction.rs
    │   ├── test_heat_transfer.rs
    │   └── test_collision.rs
    └── integration/
        ├── test_weather_cycle.rs
        └── test_scenario_load.rs
```

---

## 5. Tech Stack

| Komponente | Technologie | Begründung |
|---|---|---|
| Sprache | **Rust** | Performance, Safety, ideal für Simulationen |
| ECS | **hecs** oder **bevy_ecs** | Bewährt, performant, gut dokumentiert |
| Parser | **pest** | PEG-basiert, ideal für DSLs, Rust-nativ |
| Terminal UI | **ratatui** + **crossterm** | Standard für Rust-TUIs, cross-platform |
| Numerik | **nalgebra** | Lineare Algebra, Vektoren, Matrizen |
| Serialisierung | **serde** + **bincode** | Für Snapshots, Save/Load |
| Parallelisierung | **rayon** | Data-parallel Systems, work-stealing |
| Testing | **proptest** | Property-based Testing für Physik-Korrektheit |
| Logging | **tracing** | Structured Logging für Debug/Profiling |

---

## 6. Step-by-Step Entwicklungsplan

### Phase 0: Foundation (Wochen 1-3)

**Ziel:** Projekt-Setup, ECS-Kern, minimale Tick-Engine

```
□ Cargo Workspace mit allen Crates anlegen
□ CI/CD-Pipeline (GitHub Actions): Build, Test, Clippy, Fmt
□ ECS implementieren oder hecs integrieren
  □ Entity erstellen/löschen
  □ Components hinzufügen/entfernen/abfragen
  □ System-Trait definieren
□ Tick-Engine
  □ Fixed-timestep Loop (Δt = 1/tick_rate)
  □ System-Scheduler (Reihenfolge garantiert)
  □ Pause, Step, Speed-Control
□ Event-Bus
  □ Events registrieren, emittieren, konsumieren
  □ Deferred Events (nächster Tick)
□ Grundlegende Tests

Deliverable: `cargo run` startet eine leere Tick-Engine die im Terminal
"Tick 1... Tick 2..." ausgibt.
```

### Phase 1: AxiomLang Parser — Basics (Wochen 4-6)

**Ziel:** Sprache kann Welten, Materialien und einfache Entities parsen

```
□ Lexer
  □ Keywords: world, material, entity, terrain, spawn, place
  □ Einheiten-Literale: 5kg, 10m/s, 25°C
  □ Zahlen, Strings, Identifier, Operatoren
□ Parser (PEG mit pest)
  □ World-Block parsen → WorldConfig struct
  □ Material-Block parsen → MaterialDef struct
  □ Entity-Block parsen → EntityDef struct
  □ Terrain-Block parsen → TerrainDef struct
  □ Spawn/Place-Commands → SpawnCmd struct
□ Einheiten-System
  □ Alle SI-Einheiten + gebräuchliche Nicht-SI
  □ Automatische Konvertierung in SI-Basiseinheiten intern
  □ Compile-Error bei inkompatiblen Einheiten
□ AST → ECS-Bridging
  □ Geparste Definitionen in ECS-Entities + Components übersetzen

Deliverable: `axiom load test.ax` parst ein .ax-File und erzeugt
Entities im ECS. `axiom inspect entity.Elena` zeigt Daten an.
```

### Phase 2: Grundlegende Mechanik (Wochen 7-10)

**Ziel:** Objekte fallen, gleiten, kollidieren physikalisch korrekt

```
□ Gravitation-System
  □ F_g = m * g auf alle Entities mit Mass + Position
  □ Konfigurierbar (g-Wert, Modell)
□ Kraft-Akkumulator
  □ Sammelt alle Kräfte pro Entity pro Tick
  □ Berechnet Nettokraft → Beschleunigung
□ Integrator
  □ Semi-impliziter Euler als Baseline
  □ Verlet-Integration als Upgrade
  □ RK4 als optionaler High-Accuracy-Modus
□ Reibung
  □ Terrain-basierte Friction-Koeffizienten
  □ Haft- vs. Gleitreibung korrekt
  □ Hangabtriebskraft: F = m*g*sin(θ)
□ Kollisionserkennung
  □ Spatial Hashing (Broad Phase)
  □ AABB-Überlappung
  □ SAT für Boxes, Distanz für Spheres (Narrow Phase)
□ Kollisions-Response
  □ Impulsbasiert: j = -(1+e) * v_rel·n / (1/m1 + 1/m2)
  □ Elastizität konfigurierbar
  □ Reibung bei Kontakt

Deliverable: Man kann einen Boulder auf einem Hang spawnen und
zusehen wie er physikalisch korrekt runterrollt, beschleunigt,
an einer Wand abprallt.

Tests:
  □ Freier Fall: h = ½gt² validieren
  □ Schiefe Ebene: Endgeschwindigkeit validieren
  □ Elastischer Stoß: Impulserhaltung prüfen
  □ Reibung: Objekt bleibt auf flacher Ebene stehen
```

### Phase 3: Terminal-Rendering (Wochen 11-13)

**Ziel:** Welt visuell im Terminal sichtbar

```
□ ratatui-Setup
  □ Full-screen Terminal-App
  □ Keyboard-Input-Handling
□ Map-View
  □ Top-Down-Grid-Rendering
  □ Terrain-Farben (ANSI 256 oder TrueColor)
  □ Entity-Symbole (Unicode)
  □ Kamera-Steuerung (WASD, Zoom)
  □ Elevation-basierte Schattierung
□ Detail-Panel
  □ Entity-Inspektor (rechte Seite)
  □ Position, Velocity, Forces, Temperature
  □ Echtzeit-Update
□ Status-Bar
  □ Tick-Zähler, Simulationszeit, Speed
  □ Aktives Wetter
  □ Entity-Anzahl
□ Minimap (optional)

Deliverable: Man sieht die Welt im Terminal, kann navigieren,
Entities auswählen und ihre Physik-Daten live sehen.
```

### Phase 4: Thermodynamik (Wochen 14-17)

**Ziel:** Temperatur, Wärmeleitung, Phasenübergänge funktionieren

```
□ Temperatur-Component
  □ Jede Entity und Terrainzelle hat Temperatur
  □ Spezifische Wärmekapazität pro Material
□ Wärmeleitung (Fourier)
  □ Zwischen benachbarten Terrain-Zellen
  □ Zwischen Entity und Terrain (Kontakt)
  □ Zwischen Entities bei Kontakt
□ Konvektion
  □ Windchill-Modell
  □ Luft-Entity-Wärmeaustausch
□ Strahlung
  □ Stefan-Boltzmann für heiße Objekte (Feuer, Sonne)
  □ Solar-Einstrahlung (Tag/Nacht)
□ Phasenübergänge
  □ Wasser ↔ Eis (0°C, 334 kJ/kg)
  □ Wasser → Dampf (100°C, 2260 kJ/kg)
  □ Schnee → Wasser
  □ Terrain-Oberfläche ändert sich (nass → gefroren → eis)
□ Feuer-System (Basis)
  □ Materialien mit Flammpunkt
  □ Brandausbreitung basierend auf Wind und Material
  □ Wärmeabgabe
  □ Masse-Verlust beim Brennen

Deliverable: Ein Lagerfeuer wärmt nahestehende Entities.
Regen macht Boden nass. Frost lässt nassen Boden gefrieren.
Eis schmilzt bei Sonneneinstrahlung.

Tests:
  □ Wärmeleitung: T-Ausgleich zwischen zwei Körpern
  □ Phasenübergang: Korrekte latente Wärme
  □ Newtonsches Abkühlungsgesetz validieren
```

### Phase 5: Wetter & Atmosphäre (Wochen 18-21)

**Ziel:** Dynamisches Wetter mit physikalischen Auswirkungen

```
□ Windfeld
  □ Globales Vektorfeld über die Karte
  □ Terrain-Modifikation (Beschleunigung in Tälern, Verwirbelung hinter Bergen)
  □ Böen als stochastische Variation
□ Windkraft auf Entities
  □ F_d = ½ρv²CdA korrekt berechnet
  □ Entities können umgeworfen werden
  □ Leichte Objekte werden weggeweht
□ Niederschlag
  □ Regen: erhöht terrain.moisture, kühlt
  □ Schnee: akkumuliert als Schicht, hat Masse, isoliert
  □ Hagel: Partikel mit kinetischer Energie, beschädigt
□ Blitz
  □ Stochastisch, bevorzugt hohe Punkte
  □ Kann Feuer starten
  □ Energie-Dissipation
□ Sichtbarkeit
  □ Nebel, Regen, Schneefall reduzieren Sichtweite
  □ Affektiert Entity-AI (können nicht so weit sehen)
□ Wetter-Scheduling
  □ schedule/transition Syntax in AxiomLang
  □ Sanfte Übergänge zwischen Wetter-Zuständen
□ Tag/Nacht-Zyklus
  □ Sonnenposition basierend auf Uhrzeit + Breitengrad
  □ Temperatur-Einfluss
  □ Beleuchtungs-Einfluss

Deliverable: Ein kompletter Wetter-Zyklus: Sonniger Morgen →
Wolkenaufzug → Gewitter mit Wind, Regen, Blitz → Aufklaren.
Entities suchen Schutz, Terrain wird nass, Strukturen ächzen im Wind.
```

### Phase 6: Fluid-System (Wochen 22-25)

**Ziel:** Wasser fließt, Boden erodiert, Überflutungen passieren

```
□ Shallow-Water-Equations
  □ Höhenbasiertes Wasserfluss-Modell
  □ Wasser fließt bergab (Gradient-basiert)
  □ Akkumulation in Senken → Teiche/Seen
□ Regen-Abfluss
  □ Niederschlag → Bodenfeuchte → Sättigung → Oberflächenabfluss
  □ Abhängig von Permeabilität des Terrains
□ Flüsse & Bäche
  □ Vordefinierte Flussbetten in AxiomLang
  □ Manning-Formel für Fließgeschwindigkeit
  □ Pegelanstieg bei Regen
□ Erosion
  □ Wasser-Erosion: Terrainoberfläche wird abgetragen
  □ Geschwindigkeit abhängig von Fließgeschwindigkeit & Materialhärte
  □ Sediment-Transport und -Ablagerung
□ Grundwasser (vereinfacht)
  □ Darcy-Gesetz für Perkolation
  □ Grundwasserspiegel beeinflusst Oberfläche

Deliverable: Starkregen erzeugt Oberflächenabfluss der hangabwärts
fließt, sich in einer Senke sammelt, und dabei den Boden erodiert.

Tests:
  □ Massenerhaltung: Gesamtwassermenge bleibt konstant
  □ Fließrichtung: Immer dem Gradienten folgend
  □ Erosionsrate: Proportional zu Fließgeschwindigkeit
```

### Phase 7: Material-Physik & Strukturmechanik (Wochen 26-29)

**Ziel:** Strukturen können belastet werden, sich verformen und brechen

```
□ Spannungs-Dehnungs-Modell
  □ Hooke'sches Gesetz: σ = E·ε
  □ Elastische Verformung → kehrt zurück
  □ Plastische Verformung → permanent
  □ Bruch bei Überschreitung der Festigkeit
□ Balken-Modell (für Brücken, Dächer)
  □ Biegemoment: M = F·L (Einfeldträger)
  □ Durchbiegung: f = F·L³/(48·E·I)
  □ Knicken (Euler): F_crit = π²·E·I / L²
□ Wind-Belastung auf Strukturen
  □ Druckbelastung auf Wände
  □ Sogbelastung auf Dach
  □ Torsion bei asymmetrischer Belastung
□ Schneelast
  □ Masse der Schneeschicht auf Dachfläche
  □ Abhängig von Dachneigung
□ Ermüdung
  □ Wiederholte Belastungszyklen akkumulieren Damage
  □ Wöhler-Kurve (vereinfacht)
□ Einsturz
  □ Wenn integrity < 0 → Struktur kollabiert
  □ Erzeugt Trümmer-Entities mit Physik
  □ Kann benachbarte Strukturen beschädigen (Domino)
□ Zerstörbare Entities
  □ Kisten, Zäune, Bäume → breakable
  □ Break-Force → Trümmer mit Impuls

Deliverable: Eine Holzbrücke mit einer definierten Tragfähigkeit.
Man schickt zu viele Entities drüber → Durchbiegung steigt →
Knarren (Event) → Bruch → Trümmer fallen ins Wasser.
```

### Phase 8: AI-System & Pathfinding (Wochen 30-33)

**Ziel:** Entities verhalten sich intelligent basierend auf Physik-Zuständen

```
□ Behavior-Tree-Engine
  □ Selector, Sequence, Decorator, Leaf-Nodes
  □ Definierbar in AxiomLang
  □ Reagiert auf physikalische Zustände
□ Needs-System
  □ Hunger, Thirst, Energy, Warmth, Safety
  □ Physikalisch motiviert (Warmth = f(body_temperature))
  □ Beeinflusst Behavior-Prioritäten
□ Pathfinding
  □ A* mit physikalischen Kosten
  □ Eis = höhere Kosten (rutschig)
  □ Steile Hänge = höhere Kosten
  □ Wasser = unpassierbar (oder schwimmbar)
  □ Wind gegen Laufrichtung = höhere Kosten
□ Sensoren
  □ Vision: Sichtfeld basierend auf Wetter/Licht
  □ Hearing: Geräusche im Radius (optional)
  □ Temperature: Kälte/Hitze spüren
□ Gruppen-Verhalten
  □ Herden-Dynamik (Boids-Algorithmus)
  □ Gemeinsame Shelter-Suche

Deliverable: 20 Villager in einem Dorf. Tag/Nacht-Rhythmus,
bei Regen suchen sie Schutz, bei Kälte gehen sie zum Feuer,
bei Gefahr (Wolf, Sturm) fliehen sie intelligent.
```

### Phase 9: REPL & CLI Polish (Wochen 34-36)

**Ziel:** Professionelle Terminal-Erfahrung

```
□ REPL
  □ Tab-Completion für Commands und Entity-Names
  □ Syntax-Highlighting für AxiomLang
  □ History (↑/↓)
  □ Multi-Line-Input für komplexe Definitionen
□ CLI-Commands
  □ axiom run <file.ax>         — Simulation starten
  □ axiom check <file.ax>       — Syntax prüfen
  □ axiom inspect <file.ax>     — Welt-Übersicht ohne Start
  □ axiom replay <snapshot>     — Snapshot abspielen
  □ axiom export <format>       — CSV, JSON, Binary
  □ axiom benchmark <file.ax>   — Performance-Messung
□ Echtzeit-Charts
  □ Temperaturverlauf, Geschwindigkeit, Kräfte
  □ In separatem Terminal-Panel
□ Snapshot-System
  □ Save/Load kompletter Simulationszustand
  □ Diff zwischen Snapshots
  □ Branching (Fork einer Simulation)
□ Headless-Modus
  □ Keine UI, nur Tick-Engine
  □ Automatischer Export bei definierten Bedingungen
  □ Für CI/CD, Batch-Simulationen, AI-Training
```

### Phase 10: Stdlib & Scenarios (Wochen 37-40)

**Ziel:** Umfangreiche Standard-Bibliothek für sofortige Nutzung

```
□ Materialien (physikalisch korrekte Werte)
  □ Metalle: Stahl, Aluminium, Kupfer, Eisen
  □ Hölzer: Eiche, Kiefer, Buche, Birke
  □ Steine: Granit, Sandstein, Kalkstein, Basalt
  □ Böden: Lehm, Sand, Kies, Humus, Ton
  □ Fluide: Wasser, Salzwasser, Öl
  □ Schnee/Eis: Frischschnee, Packschnee, Firn, Eis
□ Entity-Vorlagen
  □ Humanoid (Person), mit realistischer Biomechanik
  □ Tiere: Wolf, Reh, Vogel (jeweils mit Masse, Speed, Verhalten)
  □ Fahrzeuge: Karren, Boot (Physik-basiert)
□ Struktur-Vorlagen
  □ Holzhaus, Steinhaus, Brücke, Turm, Mauer
  □ Jeweils mit korrekten Materialeigenschaften
□ Wetter-Presets
  □ Mitteleuropäisch: Frühling, Sommer, Herbst, Winter
  □ Arktisch: Blizzard, Polarnacht
  □ Tropisch: Monsun, Hurrikan
□ Beispiel-Szenarien
  □ "medieval_village" — Dorf mit Bewohnern, Jahreszeiten
  □ "avalanche_study" — Lawinensimulation für verschiedene Parameter
  □ "bridge_stress_test" — Brückenbelastung bis zum Bruch
  □ "forest_fire" — Waldbrand-Ausbreitung bei verschiedenem Wind
  □ "flood_simulation" — Überflutung eines Flusstals
```

### Phase 11: Plugin-System & Community (Wochen 41-44)

**Ziel:** Framework ist erweiterbar durch die Community

```
□ Plugin-API
  □ Custom Components registrieren
  □ Custom Systems registrieren
  □ Custom AxiomLang-Blöcke definieren
  □ Rust-basierte Plugins (Dynamic Loading oder WASM)
□ Package-Manager (axiom-pkg oder Integration mit crates.io)
  □ axiom install <plugin>
  □ axiom publish <plugin>
□ Dokumentation
  □ Vollständige API-Docs (docs.rs)
  □ Tutorial-Serie: "Build your first simulation"
  □ Physics Reference: Alle Formeln dokumentiert
  □ AxiomLang Specification
□ Community
  □ GitHub Discussions aktivieren
  □ CONTRIBUTING.md mit klarem Prozess
  □ Issue-Templates für Bugs, Features, Physik-Modelle
  □ Discord/Matrix für Echtzeit-Diskussion
□ Website
  □ Landing Page mit Demos (WASM-compiled Terminal im Browser)
  □ Online-Dokumentation
  □ Scenario-Gallery
```

---

## 7. Physik-Validierung & Testing-Strategie

Jedes physikalische System wird gegen bekannte analytische Lösungen validiert:

| Test | Erwartetes Ergebnis | Formel |
|---|---|---|
| Freier Fall 10m | t ≈ 1.43s, v ≈ 14.0 m/s | h = ½gt², v = gt |
| Schiefe Ebene 30° (reibungsfrei) | a = 4.905 m/s² | a = g·sin(θ) |
| Schiefe Ebene 30° (μ=0.3) | a = 2.36 m/s² | a = g(sin(θ) - μcos(θ)) |
| Elastischer Stoß gleiche Masse | Geschwindigkeitstausch | Impulserhaltung |
| Wärmeleitung Stab | Exponentieller Ausgleich | Fourier-Lösung |
| Wasser gefriert | Bei 0°C, Plateau durch latente Wärme | Q = m·L |
| Windkraft bei 100km/h auf 1m² | ≈ 472 N | F = ½ρv²CdA |
| Projektil 45° | Max. Reichweite: v²/g | Ballistische Kurve |
| Pendel | T = 2π√(L/g) | Harmonische Schwingung |

**Testing-Framework:**
```rust
#[test]
fn test_free_fall() {
    let world = World::new(WorldConfig { gravity: 9.81, .. });
    let ball = world.spawn((
        Position(0.0, 100.0, 0.0),
        Velocity::zero(),
        Mass(1.0),
    ));
    
    // 100 Ticks bei 60Hz = 1.667 Sekunden
    world.advance_ticks(100);
    
    let pos = world.get::<Position>(ball);
    let expected_y = 100.0 - 0.5 * 9.81 * (100.0/60.0).powi(2);
    assert_relative_eq!(pos.y, expected_y, epsilon = 0.01);
}
```

---

## 8. Open-Source-Strategie

### Lizenz
**MIT** — maximal permissiv, ermöglicht kommerzielle Nutzung, fördert Adoption.

### Repository-Setup
```
□ README.md — Elevator Pitch, Quick Start, Screenshots/GIFs
□ CONTRIBUTING.md — How to contribute, Code Style, PR Process
□ CODE_OF_CONDUCT.md — Contributor Covenant
□ CHANGELOG.md — Semantic Versioning
□ .github/
  □ ISSUE_TEMPLATE/ — Bug, Feature, Physics Model Request
  □ PULL_REQUEST_TEMPLATE.md
  □ workflows/ — CI (build, test, clippy, benchmark)
```

### Release-Strategie
```
v0.1.0  — Phase 2 abgeschlossen (Grundmechanik)
v0.2.0  — Phase 3 (Terminal-Rendering)
v0.3.0  — Phase 4 (Thermodynamik)
v0.4.0  — Phase 5 (Wetter)
v0.5.0  — Phase 6 (Fluide)
v0.6.0  — Phase 7 (Materialphysik)
v0.7.0  — Phase 8 (AI)
v0.8.0  — Phase 9 (CLI Polish)
v0.9.0  — Phase 10 (Stdlib)
v1.0.0  — Phase 11 (Plugin-System, stabile API)
```

### Metriken für Erfolg
- GitHub Stars als Indikator für Interesse
- Anzahl Community-Scenarios auf GitHub
- Anzahl Plugins/Erweiterungen
- Zitierungen in akademischen Papers (falls Physik-Validierung gut genug)
- Downloads via `cargo install axiom`

---

## 9. Optionale Zukunfts-Features (Post-v1.0)

- **3D-Modus** mit voxelbasiertem Rendering
- **WASM-Export** → Simulationen im Browser
- **LLM-Integration** → Entities mit natürlichsprachlichem Verhalten
- **Multiplayer** → Mehrere User interagieren in derselben Simulation
- **GPU-beschleunigt** → CUDA/OpenCL für große Welten (>10K Entities)
- **Replay-Video-Export** → Simulation als MP4/GIF exportieren
- **Python-Bindings** (PyO3) → Für Wissenschaftler die kein Rust können
- **REST-API** → Headless-Server mit HTTP-Interface
- **Procedural Generation** → Welten automatisch generieren (Perlin Noise, L-Systems)
