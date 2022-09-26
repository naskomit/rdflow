#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct SystemSize {
    r_param: usize,
    b_state: usize,
    r_out: usize,
    b_out: usize,
}

#[allow(dead_code)]
impl SystemSize {
    pub fn new() -> SystemSize {
        SystemSize::default()
    }

    pub fn r_param(&mut self, num: usize) -> &mut SystemSize {
        self.r_param = num;
        self
    }
    pub fn b_state(&mut self, num: usize) -> &mut SystemSize {
        self.b_state = num;
        self
    }
    pub fn r_out(&mut self, num: usize) -> &mut SystemSize {
        self.r_out = num;
        self
    }
    pub fn b_out(&mut self, num: usize) -> &mut SystemSize {
        self.b_out = num;
        self
    }
}

#[derive(Debug, Default)]
pub struct System {
    // Parameters
    r_param: Vec<f64>,
    // States
    b_state: Vec<bool>,
    // Outputs
    r_out: Vec<f64>,
    b_out: Vec<bool>
}

impl System {
  pub fn new(ss: SystemSize) -> System {
    System {
      r_param: vec![0.0; ss.r_param],
      b_state: vec![false; ss.b_state],
      r_out: vec![0.0; ss.r_out],
      b_out: vec![false; ss.b_out],
    }
  }
    
}