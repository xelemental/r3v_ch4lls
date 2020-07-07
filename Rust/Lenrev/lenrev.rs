//Challenge Name :- Lenrev
//Challenge Points :- 200
//Challenge Theory :- At first bypass a crack then compare string length land and enter a function and then crack a simple bypass which inserts a flag .



pub fn crack()

{
	let mut s = "checklen";
	let mut d = "variable";

	println!("Is it the important function ?");
	if s == "variable"
		{
			let mut s = String::with_capacity(10);
			
			s.insert(0, 'b');
			s.insert(1, 'u');
			s.insert(2, 'f');
			s.insert(3, 'f');
			s.insert(4, 'e');
			s.insert(5, 'r');
			s.insert(6, 'r');
			s.insert(7, '3');
			s.insert(8, '4');
			s.insert(9, 'd');
	      println!("{}" , s );

	}
 	if s == "checklen"
		{
			println!("Wrong Place my friend");
		}
}

pub fn lencheck()

{
	let panic = "into_bytes".to_string();
	let paniczero = "reserve_up".to_string();
	let strlen_panic = panic.len();
	let strlen_paniczero = paniczero.len();
	if strlen_panic == 9
			{
				crack();
			}
	if strlen_paniczero == 11
			{
				println!("Check out for the perfect function");
			}
      else
		{
			println!("I would suggest you to go to the perfect re-write");
		}
}

fn main()

{
	let push = 0 ;
	let pop = 1;
		if pop != 1 
			{
				lencheck();
			}
	       if pop == 1 
			{
				println!("Crackmes Just got re defined");
			}
        else
		{
			println!(" Are you the one for whom the  flag is waiting for ?");
		}
}
