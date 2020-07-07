//Challenge Name:- Overflow
//Challenge Points :- 300
//Challenge Concept :- Custom Encoding scheme which pulls off the flag with correct entry , and overflows if wrong .
//Challenge Flag :- dc9111{ru$tis_awesome}


pub fn partone ()

{
   let mut line = String::new();
   println!("Enter your name :");
   let b1 = std::io::stdin().read_line(&mut line).unwrap();
   println!("Hello , {}", line);
   let istocompare = "cracksareforfun".to_string();
//   let istocompare1 = istocompare.len();
 println!("The number of bytes you entered is , {}", b1);
		if b1 == istocompare.len()
			{
				let mut line1 = String::new();
                                println!("Enter your name :");
                                let b1 = std::io::stdin().read_line(&mut line).unwrap();
                                println!("Hello , {}", line);
                                let istocompare2 = "phaseone".to_string();
                                //   let istocompare1 = istocompare.len();
                                println!("The number of bytes you entered is , {}", b1);
					if b1 == istocompare2.len()
						{
							let mut flag = String::with_capacity(10);
							
							flag.insert(0 , 'd');
							flag.insert(1 , 'c');
							flag.insert(2 , '9');
							flag.insert(3 , '1');
							flag.insert(4 , '1');
							flag.insert(5 , 'r');
							flag.insert(6 , 'u');
							flag.insert(7 , '$');
							flag.insert(8, 't');
							flag.insert(9 , 'i');
						println!("Here's your first part of the flag {}", flag);
						parttwo();
						}
						
				else
					{
						loop{
							println!("RAM : AM I A JOKE TO YOU");
						   }
					
                  			}
  		
			}
		else
			{
				println!("RAM : am I a Joke To you");
			}
}


pub fn parttwo()

	{
		let mut name = String::new();
   println!("Enter your string :");
   let ba = std::io::stdin().read_line(&mut name).unwrap();
   println!("Entered String is {}", name);
   let istocompare3 = "cracksareforfun2".to_string();
//   let istocompare1 = istocompare.len();
 println!("The number of bytes you entered is , {}", ba);
                if ba == istocompare3.len()
                        {
                                let mut line2 = String::new();
                                println!("Enter your name :");
                                let b1 = std::io::stdin().read_line(&mut line2).unwrap();
                                println!("Hello , {}", line2);
                                let istocompare4 = "phaseone".to_string();
                                //   let istocompare1 = istocompare.len();
                                println!("The number of bytes you entered is , {}", b1);
                                        if b1 == istocompare4.len()
                                                {
                                                        let mut flag2 = String::with_capacity(9);

                                                        flag2.insert(0 , 's');
                                                        flag2.insert(1 , '_');
                                                        flag2.insert(2 , 'a');
                                                        flag2.insert(3 , 'w');
                                                        flag2.insert(4 , 'e');
                                                        flag2.insert(5 , 's');
                                                        flag2.insert(6 , 'o');
                                                        flag2.insert(7 , 'm');
                                                        flag2.insert(8, 'e');
							
                                                        
                                                println!("Here's your last part of the flag , dn't forhet to merge both of them {}", flag2)
                                                }

                                else
                                        {
                                                loop{
                                                        println!("RAM : AM I A JOKE TO YOU");
                                                   }
                                        
                        }

                        }
                else
                        {
                                println!("RAM : am I a Joke To you");
                        }

}            
  

fn main()
{
	let mut key1 = 0;
	let mut key_def = 1;
	if key_def == 0
{
	partone();
}
else
	{
		println!("Best of Luck");
	}
}

