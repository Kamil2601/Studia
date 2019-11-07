fn my_first_interpreter(code: &str) -> String {
    let mut result = String::new();
    let mut n : u8 = 0;

    for c in code.chars(){
        match c{
            '+' => n = n.wrapping_add(1),
            '.' => result = format!("{}{}",result,n as char),
            _ => ()
        }
    }

    return result;
}

fn main() {
    println!("{}",my_first_interpreter("+++++++++++++++++++++++++++++++++++++++++++++++++......."));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(my_first_interpreter("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+."), "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }

    #[test]
    fn test2() {
        assert_eq!(my_first_interpreter("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+++++++++++++++++++++++++++++.+++++++..+++.+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+++++++++++++++++++++++++++++++++++++++++++++++++++++++.++++++++++++++++++++++++.+++.++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++."), "Hello, World!");
    }

    #[test]
    fn test3() {
        assert_eq!(my_first_interpreter("++++++++++++++.+++.+++.+++.+++.+++.+.+.+.+.+.+.+.+.+.+.+."), " !\"#$%&'(")
    }

    #[test]
    fn test4() {
        assert_eq!(my_first_interpreter("++++++++++++++++++++++++++++++++++++++++++++++++++......."),"2222222")
    }

    #[test]
    fn test5() {
        assert_eq!(my_first_interpreter("+++++++++++++++++++++++++++++++++++++++++++++++++......."),"1111111")
    }

    #[test]
    fn test6() {
         assert_eq!(my_first_interpreter("+++++++++++++++++++++++++++++++++++++++++++++++++.......++.++."),"111111135")
    }

    #[test]
    fn test7() {
        assert_eq!(my_first_interpreter("+++++++++++++++++++++++++++++++++++++++++++++++++.......++..++."),"1111111335")
    }

    #[test]
    fn test8() {
        assert_eq!(my_first_interpreter("+++++++++++++++++++++++++++++++++++++++++++++++++.......++..++.+.."),"111111133566")
    }

    #[test]
    fn test9() {
        assert_eq!(my_first_interpreter("+++++++++++++++++++++++++++++++++++++++++++++++++.......++..++.+..++..."),"111111133566888")
    }

    #[test]
    fn test10() {
        assert_eq!(my_first_interpreter("+++++++++++++++++++++++++++++++++++++++++++++++++.......++..++.+..++.....+++++++"),"11111113356688888")
    }
}