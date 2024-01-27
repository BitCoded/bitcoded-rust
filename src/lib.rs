pub fn add(a: usize, b: usize) -> usize
{
	a + b
}

pub fn sub(a: usize, b: usize) -> usize
{
	a - b
}

pub fn mul(a: usize, b: usize) -> usize
{
	a * b
}

pub fn div(a: usize, b: usize) -> usize
{
	a / b
}





#[cfg(test)]
mod tests 
{
	use super::*;

	#[test]
	fn test_add()
	{
		let result = add(7312543, 41629);
		let expect = 7354172;
		assert_eq!(result, expect);
	}

	#[test]
	fn test_sub()
	{
		let result = sub(830, 75);
		let expect = 755;
		assert_eq!(result, expect);
	}

	#[test]
	fn test_mul()
	{
		let result = mul(60, 25);
		let expect = 1500;
		assert_eq!(result, expect);
	}

	#[test]
	fn test_div()
	{
		let result = div(720, 18);
		let expect = 40;
		assert_eq!(result, expect);
	}
}
