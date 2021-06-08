use std::io;
use std::io::Write;

fn notmainlol(){

    let mut _user_check = "0x7aab2";
    let  (mut _a, mut _b, mut _c , mut _d) : (u64, u64, u64, u64); //four variables are declared
    println!("Welcome to level 0, Please enter the key: "); //Waits for the input from user

    let mut user_guess = String::new(); 
    io::stdin().read_line(&mut user_guess).expect("failed to read the line");
    let user_guess = user_guess.trim(); //Trims the string and avoids  new line


    if user_guess == _user_check{
        let mut _user_check_two = "0x66ab";
        let s = String::from(" ❤️");
         println!("Dear User, Level 0 has been patched, Welcome to level 1 {} \n Please enter the next  key : ",s);
        let mut user_guess_two = String::new();
        io::stdin().read_line(&mut user_guess_two).expect("Failed to read line");
        let user_guess_two = user_guess_two.trim(); //Trims the string to avoid a new line character
                if _user_check_two == user_guess_two{
                     let mut pointer = 0x98de;
                        for _i in 0..2{

                    
                      pointer += 0xaab1;
    
                 }
                      let _a = pointer;
                      println!("Dear User, Level 1 has been patched, Welcome to level 2 {} \n Do you still want to continue :?","❤️" );
                      let mut user_guess_three = String::new();
                      io::stdin().read_line(&mut user_guess_three).expect("Failed to read line");
                      let user_guess_three = user_guess_three.trim();
                      let _user_check_three = "Yes";
                      if user_guess_three == _user_check_three{
                          println!("Welcome to level 2, Let us continue the search of Key!!!!");
                          for _j in 0..1{
                              pointer += 0x76a01;
                              let _y = 90;
                              if _y > 20{
                              match pointer{
                                  0x95841 => {
                                     println!("Time to learn tables {}", "😃");
                                     for _z in 0..11{
                                         for _k in 0..2{
                                             println!("The tables are : {} * {} = {}", _k, _z, _k *_z);
                                         }
                                     }




                                  },
                                  _ => println!("Lol Ok")
                              };
                            }
                              else{
                              println!("Oops you forgot to patch the instruction :(");
                              }
                              pointer += 0x76a01;
                              let x = 0x666ab;
                              if x > pointer{ //
                                  match pointer{
                                       _ => {
                                           pointer += 0x945ab3;
                                           for _m in 0..5{
                                               pointer += 0x678ab;
                                           }
                                           let f1ag = &pointer;
                                          let convert : u8 = *f1ag as u8;
                                           println!("{}", convert);
                                          let mut file = std::fs::File::create("data.txt").expect("create failed");
                                          file.write_all(&[convert]).expect("write failed");
                                       }
                                  }
                              }
                          }
                      }
                      else{
                          println!("Goodbye ! It is sad to see you go :(");
                      }
                      
                      




                                                  }else{
                                                      println!("Oops you entered the wrong Key {}", "😕");
                                                  }
                                  }
     else
        {
            println!("Oops you entered the wrong Key {}", "😕");
   
        }


}

fn main(){



let  def_user_choice = "Yes";
println!("Welcome to the game, do you want to continue( Yes/No) : ?");
let mut user_original_choice= String::new();
io::stdin().read_line(&mut user_original_choice).expect("Failed to read line");
let user_original_choice = user_original_choice.trim();
if user_original_choice == def_user_choice{
notmainlol();
}
else{
    println!("Good Bye");
}


}