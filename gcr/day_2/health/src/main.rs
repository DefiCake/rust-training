use std::convert::TryFrom;

pub struct User {
  name: String,
  age: u32,
  height: f32,
  visit_count: usize,
  last_blood_pressure: Option<(u32, u32)>,
}

pub struct Measurements {
  height: f32,
  blood_pressure: (u32, u32),
}

#[derive(Debug)]
pub struct HealthReport<'a> {
  patient_name: &'a str,
  visit_count: u32,
  height_change: f32,
  blood_pressure_change: Option<(i32, i32)>,
}

impl Default for User {
  fn default() -> Self {
    User { name: String::from(""), age: 0, height: 0.0, visit_count: 0, last_blood_pressure: None }
  }
}

impl Measurements {
  pub fn blood_pressure_diff(&self, blood_pressure: (u32, u32)) -> (i32, i32) {
    let p1: i32 = i32::try_from(self.blood_pressure.0).unwrap() - i32::try_from(blood_pressure.0).unwrap();
    let p2: i32 = i32::try_from(self.blood_pressure.1).unwrap() - i32::try_from(blood_pressure.1).unwrap();
    (p1, p2)
  }
}

impl User {
  pub fn new(name: String, age: u32, height: f32) -> Self {
    User { name, age, height, ..Default::default() }
  }

  pub fn name(&self) -> &str {
    unimplemented!()
  }

  pub fn age(&self) -> u32 {
    self.age
  }

  pub fn height(&self) -> f32 {
    self.height
  }

  pub fn doctor_visits(&self) -> u32 {
    u32::try_from(self.visit_count).unwrap()
  }

  pub fn set_age(&mut self, new_age: u32) {
    self.age = new_age;
  }

  pub fn set_height(&mut self, new_height: f32) {
    self.height = new_height;
  }

  pub fn visit_doctor(&mut self, measurements: Measurements) -> HealthReport {
    self.visit_count += 1;

    let height_change = self.height - &measurements.height;
    let blood_pressure_change: Option<(i32, i32)> = match self.last_blood_pressure {
      None => None,
      Some(last_blood_pressure) => Some(measurements.blood_pressure_diff(last_blood_pressure)),
    };

    self.height = measurements.height;
    self.last_blood_pressure = Some(measurements.blood_pressure);

    HealthReport {
      patient_name: self.name.as_str(),
      visit_count: u32::try_from(self.visit_count).unwrap(),
      height_change,
      blood_pressure_change,
    }
  }
}

fn main() {
  let mut bob = User::new(String::from("Bob"), 32, 155.2);
  println!("I'm {} and my age is {}", bob.name(), bob.age());

  let report = bob.visit_doctor(Measurements {
    height: 156.1,
    blood_pressure: (120, 80),
  });

  println!("Report:");
  println!("Patient name: {}", report.patient_name);
  println!("Visit count: {}", report.visit_count);
  println!("Height change: {}", report.height_change);
  println!("Blood press. change: {:?}", report.blood_pressure_change.unwrap_or((0, 0)));
}

#[test]
fn test_height() {
  let bob = User::new(String::from("Bob"), 32, 155.2);
  assert_eq!(bob.height(), 155.2);
}

#[test]
fn test_set_age() {
  let mut bob = User::new(String::from("Bob"), 32, 155.2);
  assert_eq!(bob.age(), 32);
  bob.set_age(33);
  assert_eq!(bob.age(), 33);
}

#[test]
fn test_visit() {
  let mut bob = User::new(String::from("Bob"), 32, 155.2);
  assert_eq!(bob.doctor_visits(), 0);
  let report = bob.visit_doctor(Measurements {
    height: 156.1,
    blood_pressure: (120, 80),
  });
  assert_eq!(report.patient_name, "Bob");
  assert_eq!(report.visit_count, 1);
  assert_eq!(report.blood_pressure_change, None);

  let report = bob.visit_doctor(Measurements {
    height: 156.1,
    blood_pressure: (115, 76),
  });

  assert_eq!(report.visit_count, 2);
  assert_eq!(report.blood_pressure_change, Some((-5, -4)));
}
