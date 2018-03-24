fn main() {
    let mut sum: u64 = 0;

    for point in 0..1000 {
        let mut temp: i16 = point;
        while point > 0 {
            if temp == 0 {
                sum += temp as u64;
                break;
            }
            if temp < 0 {
                break;
            }
            temp -= 3 as i16;
        }
        let mut temp: i16 = point;
        while point > 0 {
            if temp == 0 {
                sum += temp as u64;
                break;
            }
            if temp < 0 {
                break;
            }
            temp -= 5 as i16;
        }
    }

    println!("{}", sum);
}
