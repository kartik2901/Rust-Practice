// use std::io;

// fn getinput()-> io::Result<i32>{
//     let mut tmp = i32::new();
//     io::stdin().read_line(&tmp)?;
//     Ok(tmp.trim().to_owned())

// }

// fn main(){
// let mut check = getinput();
// println!("Here is your output {:?}" , check ); 
// }

// use std::io;

// fn main(){

//     println!("Enter any number");
//     let mut number = String::new();
//     io::stdin()
//         .read_line(&mut number)
//         .expect("Failed to read input");

//     println!("Entered number is {}", number);

// }

// use std::io;

// fn main() {
// let mut input_text = String::new();
//     io::stdin()
//         .read_line(&mut input_text)
//         .expect("failed to read from stdin");

//     let trimmed = input_text.trim();
//     match trimmed.parse::<u32>() {
//         Ok(i) => println!("your integer input: {}", i),
//         Err(..) => println!("this was not an integer: {}", trimmed),
//     };
    
// }


//Palimdrome Almost final 

// fn checkprime(a : i32)
// {
//     let m = a/2;
//     // let mut flag = 0; 
//     if a == 0 || a == 1 
//         {
//             println!("It's not a prime") 

//          }
//     else  
//     {
//         for i in 1..m 
//         {
//             if a%i == 0 
//             {
//                  println!("{:?} is not  Prime " , a);
//                 break;
                 
//             }
//              else{
//                 println!("{:?} is a Prime Number " , a);
//                    break; 
                 
//                  }
//         }

//     }

// }

// fn main(){
// let _number = checkprime(11);

// }

// Palndrome alsmot final - end 

//'''''''''''''''''''''''''''''''''''''''''''''' 

// fn check_palindrome(a : i64){
//   let mut r = 0 ;
// //   println!("Palindrome Number ");
//   let mut n = 0;
// //   println!("Palindrome Number ");
//   let mut sum = 0 ; 
// //   println!("Palindrome Number ");
  
//     while n>0{
//         // println!("Palindrome Number ");
       
//         r = r%a;
       
//         sum = (sum*10) + r ;
       
//         n = n / 10 ;

       
//     }

//     println!("{:?}" , sum);
//  if a == sum  {
//      println!("Palindrome Number ");
//  }else {
//      println!("Not a palindrome");
//  }
// }

// fn main(){

//     check_palindrome(1111);

// }

//'''''''''''''''''''''''''''''''''''''''''''''' 


fn check_palindrome( x : i32)->bool{
    if x<0{
        false
    }else {
        let mut rev = 0;
        let mut num = x;
        while num > 0{
            rev = rev*10 + num  % 10 ; 
            num = num / 10; 
        } 
        
       if rev == num 
        //    println!("Palindrome");
       
    }
        
    
    
    fn main(){
        let output = check_palindrome(1221);
        if output == true {
            println!(" It is a plaindrme");
        }else {
        }
    }