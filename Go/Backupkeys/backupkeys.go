//Challenge Description : Can you recover my backup keys to get the flag , they probably are hardcoded ? 


package main

import (
	"fmt"
	"encoding/hex"
	"bytes"
)

func check1() (string) {
	x := "Jack"
	return x
}

func check2() (string) {
	y := "owns"
	return y
}
func check3() (string) {
	z := "the"
	return z
}
func check4() (string) {
	k := "maze"
	return k
}

func main() {
	var key1 [4]string
	key1[0] = "Jack"
	key1[1] = "owns"
	key1[2] = "the"
	key1[3] = "maze"
	//fmt.Println(key1[0], key1[1])
	//fmt.Println(key1)
	ischeck := bytes.Equal([]byte(check1()), []byte(key1[0]))
	//fmt.Println(ischeck)
	ischeck1 := bytes.Equal([]byte(check2()), []byte(key1[1]))
	//fmt.Println(ischeck1)
	//ischeck2 := bytes.Equal([]byte(check3()), []byte(key1[2]))
	//fmt.Println(ischeck2)	
	//ischeck3 := bytes.Equal([]byte(check4()), []byte(key1[3]))
	//fmt.Println(ischeck3)	
	if ischeck == true{
		for i:= 0 ; i<2; i++ {
			var magicstring string
			fmt.Println("Enter the magic code")
			fmt.Scanln(&magicstring)
			fmt.Println("The magic string you entered "+ magicstring+ " just a corrupted illusion :)")
			

		}
	}
		if ischeck == false {
                  str :=  "Jack"   
                  hx := hex.EncodeToString([]byte(str))  
                  fmt.Println("Might be helpful ;) ====" + hx)    
                  fmt.Println("You have the original magic string :)")
			}
			if ischeck1 == false{

			var p string 
			fmt.Println("Enter the magic code you have just received to get the keys : ")
			fmt.Scanln(&p)
			if p == "00s0adnssn"{
					
							fmt.Printf("%q\n", bytes.SplitAfter([]byte("Try, Harder"), []byte(",")))

					};if p == "4a61636b"{  
					fmt.Println("Phew Phew collect the keys below , don't forget to put them in flag{} format")
			         fmt.Printf("%q\n", bytes.SplitAfter([]byte("Hardcoded,passwords,are,useless"), []byte(",")))
				}else{
					fmt.Println("Try Harder")
				}	
}

}
