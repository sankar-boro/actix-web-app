pub struct Update {
	query: String,
	sets: usize,
}
impl Update {
	pub fn from(table: &str) -> Self {
		let q = format!("UPDATE {} SET", table);
		Self {
			query: q,
			sets: 0,
		}
	}

	pub fn set(mut self, key: &str, value: &str) -> Self {
		if self.sets == 0 {
			let q = format!(" {}='{}'", key, value);
			self.query.push_str(&q);	
		} else {
			let q = format!(", {}='{}'", key, value);
			self.query.push_str(&q);	
		}
		self.sets += 1;
		self
	}

	pub fn where_in(mut self, key: &str, value: &str) -> Self {
		let q = format!(" WHERE {}={}", key, value);
		self.query.push_str(&q);
		self
	}

	pub fn and(mut self, key: &str, value: &str) -> Self {
		let q = format!(" AND {}={}", key, value);
        self.query.push_str(&q);
		self
	}

	pub fn query(self) -> String {
		self.query.clone()
	} 
}

fn main() {
	let mut b = Vec::new();
	b.push(());
	b.push(());
	b.push(());
	let c = 
	println!("{:?}", b);
}