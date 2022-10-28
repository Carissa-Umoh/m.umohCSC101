fn main() {
   let t:f64 = 450000.0;
   let m:f64 = 1500000.0;
   let h:f64 = 750000.0;
   let d:f64 = 2850000.0;
   let a:f64 = 250000.0;
   let q:f64 = 2.0;
   let w:f64 = 1.0;	
   let e:f64 = 3.0;

   //sum and average
   let sum = (t * q) + (m * w) + (h * e) + (d * e) + (a * w);
   println!("sum is {}", sum);
   let average = sum / 5.0;
   println!("average is {}", average);
    }