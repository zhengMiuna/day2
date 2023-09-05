use std::io;

//Buffer
struct Buffer<T> {
    pub x: Vec<T>,
}

impl <T> Buffer<T>
where T: std::ops::Add<Output=T> +Clone +Default,
{
    pub fn sum(&self) -> T {
        let mut sum = T::default();
        for i in &self.x {
            sum = sum + i.clone();
        }
        sum
    }
}

//Stringcompare

fn compare_string(x: &str, y: &str) -> bool {
    let chars1: Vec<char> = x.chars().collect();
    let chars2: Vec<char> = y.chars().collect();
    for i in 0..chars1.len() {
        if i == chars2.len() {
            return true;
        }
        if chars1[i] > chars2[i] {
            return true;
        }else if chars1[i] < chars2[i] {
            return false;
        }
    }
    false
}

//closure

fn fun(v: &Vec<char>) -> Vec<char> {
    let res = v.iter().map(|x| std::char::from_u32((*x as u32) + 1).unwrap()).collect();
    res
}



fn main() {

    //test for buffer
    println!("Buffer:");
    let a=Buffer{x:vec![1,2,3]};
    let b= Buffer{x:vec![1.1,2.2,3.3]};
    println!("1 + 2 + 3 = {}",a.sum());
    println!("1.1 + 2.2 + 3.3 = {}",b.sum());

    //test for stringcompare
    println!("compareString:");
    let mut a =String::new();
    let mut b =String::new();
    println!("Please enter the first string:");
    io::stdin().read_line(&mut a);
    println!("Please enter the second string:");
    io::stdin().read_line(&mut b);
    let res =compare_string(&a[..], &b[..]);
    println!("{} > {} is {}",a,b,res);

    //test for closure
    println!("closure:");
    let a =vec!['a','b','c','d','e'];
    println!("before: {:?}",a);
    println!("after: {:?}",fun(&a));

}
