fn main() {
	let toshiba = 450_000.00 * 3.00;
	let mac = 1_500_000.00 * 3.00;
	let hp = 750_000.00 * 3.00;
	let dell = 2_850_000.00 * 3.00;
	let acer = 250_00.00 *1.00;
	
	let sum = toshiba + mac + hp + dell + acer;
	let avg = sum/10.00;
	println!("sum is{}", sum);
    println!("average {} ", avg);
}