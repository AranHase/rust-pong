
pub struct GameState {
	pub p1_pady: f32,
	pub p2_pady: f32,
}

impl GameState {
	pub fn new() -> GameState { GameState { p1_pady: 0.5, p2_pady: 0.5 } }

	pub fn update(&mut self, delta_time: f32) {
		self.p1_pady += 0.1*delta_time;
		if self.p1_pady > 1. { self.p1_pady = 0.; }
		self.p2_pady += 0.3*delta_time;
		if self.p2_pady > 1. { self.p2_pady = 0.; }
	}
}

