fn main(){

test1 := 10
test2 := 20
test3 := 30
test4 := 40
test5 := 50
test6 := 60

if test1 > test2 {

	one()
} else {
	 println('Program has crashed at ..... *one')
} 
if test3 > test4{
	 two()
}else {
	println('Program has crashed at .... *two')
}
if test5 > test6{
	three()
}else {
	println('Program has crashed at ..... *three')
}
}
fn one(){

	 nums := [91,92,93,94,95]  //Look at the arrays mate
	one:= [ `0`, `x` , `b` ,`D` ,`6`, `2` ,`3` ,`E`, `3` ,`3` ,`c` ,`5`,`b`, `b`]
	even := nums.filter( it %100 == 0)
//	println(even)
	even_fn := nums.filter(fn (x int ) bool {
		return x % 2 == 0
})
a := 10
b := 10
if a < b{
v := int(`b`)
changed := one.map(int(it) ^ v)
println(changed) // You gotta find the key this was just xored with the key
}else{ 
	changedtwo := nums.map(it ^ 9)
	println(changedtwo)
}
}

fn two(){
	numstwo := [ 80, 81, 82, 83, 84, 85 ]
	two := [ `6`, `0`,`6`,`5`,`5`, `B`, `3`, `6`, `d`, `6`, `6`, `6`, `6`, `a`]
	eventwo := numstwo.filter( it % 100 == 0)
	//println(eventwo)
	eventwo_fn := numstwo.filter(fn (x int) bool {
		return x % 2 == 0
})

c := 20
d := 20
if c < d{
w := int(`b`)
changed2 := two.map(int(it) ^ w)
println(changed2)
}else{
	changedthree := numstwo.map(it ^ 2)
	println(changedthree)
}
}

fn three(){

	numsthree := [70, 71, 72, 73, 74]
	three:= [`8`,`0`,`5`,`6`,`D`, `B`, `a`, `6`, `4`, `2`, `9`, `9`,`E`, `B`]
	eventhree := numsthree.filter(it % 100 == 0)
	eventhree_fn := numsthree.filter(fn (x int)bool{
		return x % 2 == 0
	})
	
	e := 30
	f := 30
	if e < f{
		x := int(`b`)
		changed3 := numsthree.map(it ^ x)
		println(changed3)
		}else{
			changedfour :=numsthree.map(it ^ 3)
			println(changedfour)
		}
	}
