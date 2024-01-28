pub fn hash(step: &str) -> u8 {
    let mut out: u16 = 0;
    for c in step.chars() {
        let ascii = out + (c as u8) as u16;
        out = ascii as u16 * 17 % 256 ;
    }
    out as u8
}


pub fn process_part1(input: &str) -> u64 {
   input
        .split(',')
        .map(|step|
            {
               hash(step) as u64
            }
        ).sum()
}



////////////////////////////////////////


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let result = process_part1(&input);
        assert_eq!(result, 1320);
    }


}