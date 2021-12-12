use numext_fixed_uint::U1024;
use linked_hash_map::LinkedHashMap;
use std::fmt;

pub fn get_primes(n:usize) -> Vec<u64>{
    let mut primes = vec![2];
    let mut num = 3;
    while primes.len() < n {
        let mut add = true;
        for n in primes.iter() {
            if num % n == 0 {
                add = false;
                break;
            }
        }
        if add {
            primes.push(num);
        }
        num += 1;
    }
    primes
}

#[derive(Debug,Clone)]
enum Operator {
    Empty,
    Multiplication,
    Exponential,
    Addition,
}

#[derive(Debug,Clone)]
pub struct Equation {
    op: Operator,
    num: Option<u8>,
    exp: Box<Option<Equation>>,
}

impl Equation {
    pub fn new_from_num(num: U1024, primes: &[u64]) -> Option<Equation> {
        // let mut factors = LinkedHashMap::new(); 

        let mut lowest = (U1024::max_value(),0,0);


        for i in 0..256 {
            let (quo,rem) = num.complete_div(&U1024::from(primes[i]));
            if rem.is_zero() && quo < lowest.0{
                lowest.0 = quo;
                lowest.1 = i;
                lowest.2 = -1;
            }
            let mut e: i32 = 1024/2;
            let mut c = e;
            while c > 0 {
                c = c/2;
                if e <= 0 || e > 1024 {
                    break;
                }
                let exp = U1024::from((i+1) as u16).checked_pow((e+1) as u32);
                match exp {
                    Some(res) => {
                        if i == 1 && e >= 1022 {
                            println!("{} {} {}",res,num,e);
                        }
                        if res <= num {
                            // println!("{} {}",i,e);
                            let sub = num.checked_sub(&res).unwrap();
                            if i == 1 && e >= 1022 {
                            }
                            if sub < lowest.0 {
                                lowest.0 = sub;
                                lowest.1 = i;
                                lowest.2 = e;
                            }

                            e += c;
                        } else {
                            e -= c;
                        }
                    }
                    None => {
                        e-=c;
                    }
                }
            }
        }        
        let new_eq;
        if !lowest.0.is_zero() {
            // println!("{} {} {}",lowest.0,lowest.1,lowest.2);
            new_eq = Equation::new_from_num(lowest.0, primes);
        }else{
            new_eq = None
        }
        if lowest.2 != -1 {
            return Some(Equation {
                op: Operator::Addition,
                num: Some(lowest.1 as u8),
                exp: Box::new(Some(Equation{
                    op: Operator::Exponential,
                    num: Some(lowest.2 as u8),
                    exp: Box::new(new_eq),
                })),
            })
        } 
        return Some(Equation {
            op: Operator::Multiplication,
            num: Some(lowest.1 as u8),
            exp: Box::new(new_eq),
        })

        // for (i,p) in primes.iter().enumerate() {
        //     let prime = U1024::from(*p);
        //     let (quo,rem) = num.complete_div(&prime);

        //     if rem.is_zero() {
        //         // println!("mult {} {}",prime, i);
        //         factors.insert(i as u8,quo);
        //     }
        // }


        // for (i,quo) in factors.iter().rev() {
        //     return Some(Equation {
        //         op: Operator::Multiplication,
        //         num: Some(*i),
        //         exp: Box::new(Equation::new_from_num(quo.clone(), primes)),
        //     })
        // }

    }

    fn get_sum(&self) -> u32 {
        let mut sum = 11;
        match &*self.exp {
            Some(eq) => sum += eq.get_sum(),
            None => return 0,
        };
        sum
    }

}

impl fmt::Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match &self.op {
            Operator::Multiplication => "*",
            Operator::Exponential => "^",
            Operator::Addition => "+",
            Operator::Empty => "",
        };
        let exp = match &*self.exp {
            Some(eq) => format!("{}",eq),
            None => String::from(""),
        };

        write!(f,"{} {} {}",op,self.num.unwrap_or_default(),exp)

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use numext_fixed_uint::u1024;

    #[test]
    fn check_primes() {
        
        let primes : Vec<u64> = vec![2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97,101,103,107,109,113,127,131,137,139,149,151,157,163,167,173,179,181,191,193,197,199,211,223,227,229];

        assert_eq!(get_primes(50),primes);
    }

    #[test]
    fn check_equation() {
        let primes = get_primes(1024);

        // let eq = Equation {
        //     op: Operator::Addition,
        //     num: Some(200 as u8),
        //     exp: Box::new(Some(Equation{
        //         op: Operator::Exponential,
        //         num: Some(10),
        //         exp: Box::new(
        //             None
        //         ),
        //     })),
        // };

        let num = u1024!("0b1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
        let r = Equation::new_from_num(num,&primes).unwrap();
        panic!("{} {}",r,r.get_sum());

    }

}