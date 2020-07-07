//Name :- ChangeBreak 
//Challenge Points :- 150
//Bypass 2 string length checks and get the flag 
//Programming Language Used :- Rust 

pub fn flag ()

{
	let mut key = 1 ;
	let mut key1 = 0;
		if key1 == 1 //Once you change the value during run time the flag will be printed 
			{
				let mut s = String::with_capacity(10);
					
				s.insert(0, 'c');
				s.insert(1 , 'h');
				s.insert(2, '4');
				s.insert(3, 'n');
				s.insert(4, 'g');
				s.insert(5, '3');
				s.insert(6, '7');
				s.insert(7 , 'e');
				s.insert(8, 'l');
				s.insert(9 , 'n');
				println!("{}" , s);
			}

		else
			{
				println!("Nothing here");
			}
}

pub fn check()

{
		let len0 = "notempty";
		let len1 = "hardlyempty";
		let len_check = len0.len(); //String length of len0 variable stored in len__check 
		let len_check1 = len1.len();//Strung lentgth of len1 variable stored in len_check1
			if len_check == 7   //Once you change the values during run time you have access to the flag function .
				{
					 
					flag();
				}
			else
				{
					loop
						{
							println!("Nothing here");
						}
				}
}

fn main()
		
	{
		let mut key1 = "is_equal";
		let mut key2 = "not_equal";
			if key1 == "not_equal" //String Compare will lead you to the check function .
				{
					check();
				}
			else
				{
					println!("This is not the place you are are looking for the flag");
				}
	}

