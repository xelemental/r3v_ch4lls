//Challenge Name :- StringCrack 
//Concept - Crack the program through multiple breakpoints and push the string to get the flag and wrong function will lead to wrong push and wrong flag will get printed.
//Challenge Points - 150


pub fn crack1()

{
	let mut a = "getid".to_string();
	let mut b = "0x000".to_string();

	if b.len() == 4	 
		{
//			use std::io::stdin;
//			println!("Either you crack it or get the wrong flag");
			let mut s = 2 ;
//			let mut s =String::new();
//			println!("Enter the magic string : ");
			let check = "getuid".to_string();
		//	let _= stdout ().flush();
//			stdin(). read_line(&mut s).expect("Is this what you enter");
				if s  == 1 
					{
						let mut isflag = String::from("r00t");
						isflag.insert_str(0, "n00b");
						isflag.insert_str(2, "boot");
						//isflag.insert_str(0 ,"{y0u");
						//isflag.insert_str(2 ,"4r3th3");
						//isflag.insert_str(3 ,"r00t}");
				          println!("The flag is {}" , isflag);
					}
			else
				{
					
					let mut flag = String::from("fake");
					flag.insert_str(1 ,"flag");
					println!("The flag which is not correct is as follows {}",flag);
				}
	}
}

	
pub fn false1 ()
	{

			 loop{
				println!("Go , Get the flag") ; 
				
				
			         }

 		}
fn main()
{
	let key = 1 ;
	if key != 1
 	
	{
	crack1();
	}
       else
	{
		println!("RAX is surely your piece of cake");
		false1();
	}
}
