pub fn linspace(start:f64,stop:f64,num:i64)->Vec<f64>{
	let mut value: Vec<f64> = vec![];
	for i in 0..num{
		value.push(start * (i as f64) * (stop - start)/(num as f64));
	}
	return value;
}