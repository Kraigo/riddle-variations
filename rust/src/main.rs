struct Variants {
    arr: Vec<f32>,
    basis: usize,

    v_size: usize,
    v_count: usize,
    convert: Box<Fn(i32) -> Vec<i32>>
}

impl Variants {
    pub fn converter(&self, _from: usize, to: usize) -> Box<Fn(i32) -> Vec<i32>> {
        let basis = self.basis;
        Box::new(move |num: i32| {
            let mut cur = num.clone();
            let mut result: Vec<i32> = vec![0; basis];
            let mut count = result.len();
            while cur != 0i32 {
                let res = cur % to as i32;
                let next = cur as f32 / to as f32;
                cur = next.floor() as i32;
                result[count - 1] = res;
                count -= 1;
            }
            result
        })
    }


    pub fn new() -> Variants {
        Variants {
            arr: Vec::new(),
            basis: 1,
            v_size: 0,
            v_count: 0,
            convert: Box::new(|_num: i32| vec![])
        }
    }

    pub fn arr(&mut self, val: Vec<f32>) -> &mut Variants {
        self.arr = val;
        self
    }

    pub fn basis(&mut self, val: usize) -> &mut Variants {
        self.basis = val;
        self
    }

    pub fn finalize(&self) -> Variants {
        Variants {
            arr: self.arr.clone(),
            basis: self.basis,
            v_size: self.arr.len().pow(self.basis as u32),
            v_count: self.v_count,
            convert: self.converter(10usize, self.arr.len() as usize)
        }
    }
}

impl Iterator for Variants {
    type Item = Vec<f32>;
    fn next(&mut self) -> Option<Vec<f32>> {
        if self.v_count < self.v_size {
            let conv = &self.convert;
            let result = conv(self.v_count as i32)
                .iter()
                .map(|&k| self.arr[k as usize])
                .collect();
                
            self.v_count += 1;
            return Some(result)
        }
        else {
            None
        }
        
    }
}

fn main() {
    let mut source: Vec<f32> = vec![1.0, 3.0, 5.0, 7.0, 9.0, 11.0, 13.0, 15.0];
    let mut source_modified = vec![];
    let target = 30.0;
    let mut result = vec![];
    let after_point = 1;
    let source_len = source.len() as i32;

    println!("== Source: {:?}", source);
    println!("== Target: {}", target);
    println!("== Point: {}", after_point);
    
    for i in 0..source_len {
        for n in 0..source_len {
            
            let two_digit = format!("{:.0}{:.0}", source[i as usize], source[n as usize]);
            match two_digit.parse::<f32>()
            {
                Ok(r) => source_modified.push(r),
                _ => {}
            };
        }
        for p in 0..after_point {
            let variants = Variants::new()
                .arr(source.clone())
                .basis(p + 1)
                .finalize();

            for v in variants {
                let point = v.iter().map(|y| format!("{:.0}", y)).collect::<Vec<String>>().join("");
                let with_point = format!("{:.0}.{}", source[i as usize], point);
                match with_point.parse::<f32>()
                {
                    Ok(r) => source_modified.push(r),
                    _ => {}
                };
            }
        }
    }
    
    source.extend(&source_modified);
    source.retain(|&v| v < target);
    source.sort_by(|a, b| a.partial_cmp(b).unwrap());
    source.dedup();

    println!("Variants: {}", source.len());
    let variants = Variants::new()
        .arr(source)
        .basis(3)
        .finalize();

    println!("Combitaions: {}", variants.v_size);
    println!("...");
    print!("Current iter:");
    let mut var_count = 0.0;
    let var_size = variants.v_size.clone() as f32;
    for v in variants {
        var_count += 1.0;
        let sum = v.iter().fold(0f32, |mut sum, &val| {sum += val; sum});

        if var_count % 5000.0 == 0.0 {
            let percent = (&var_count / &var_size) * 100.0;            
            print!("\rCurrent iter: {:.2}%", percent);
        }

        if sum == target {
            result.push(v);
        }
    }
    print!("\rCurrent iter: done        ",);
    println!("\n...Sort variants");
    for r in &mut result {
        r.sort_by(|a, b| a.partial_cmp(b).unwrap())
    }

    println!("...Sort result");
    result.sort_by(|a,b| {
      let first = format!("{:?}", a);
      let second = format!("{:?}", b);
      first.partial_cmp(&second).unwrap()
    });
    
    println!("...Dedup result");
    result.dedup_by(|a,b| {
      let first = format!("{:?}", a);
      let second = format!("{:?}", b);
      first.eq_ignore_ascii_case(&second)
    });

    println!("Available {} variants", result.len());
}
