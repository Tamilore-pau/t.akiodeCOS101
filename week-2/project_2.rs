fn main () {
	let t_amount:f64 = 450_000.00;   let t_quantity:f64 = 2.00;
    let m_amount:f64 = 1_500_000.00; let m_quantity:f64 = 1.00;
    let h_amount:f64 = 750_000.00;  let h_quantity:f64 = 3.00;
    let d_amount:f64 = 2_850_000.00; let d_quantity:f64 = 3.00;
    let a_amount:f64 = 250_000.00;   let a_quantity:f64 =1.00;

    let t_total = t_amount * t_quantity;
    let m_total = m_amount * m_quantity;
    let h_total = h_amount * h_quantity;
    let d_total = d_amount * d_quantity;
    let a_total = a_amount * a_quantity;
    
    let sum = t_total + m_total + h_total + d_total + a_total;
    println!("sum is {}", sum);

    let average = sum / 10.00;
    println!("average is {}", average);

}