//Author: Elemental X 
//Category : Reversing


package main

import (
		"fmt"
		"encoding/hex"
		)
  func one() {
  		fmt.Println("Enter the key --> : ")
		var str string
		fmt.Scanln(&str)
	   runes := []rune(str)

	    var result []int

	for i := 0; i < len(runes); i++ {
		result = append(result, int(runes[i]))
	}

	//fmt.Println(result)
	var sum int
	x := (result)
	for i := 0; i < len(x); i++ {
        fmt.Scan(&x)
        sum += x[i]
	//fmt.Println(sum)
}
     if sum == 405 {
	 		 var flag = []byte{114 ,101 ,118 ,112 ,114 ,111}
                                        finflag := string(flag)
                                        finflag2 := (finflag)
                                              enc2:=hex.EncodeToString([]byte(finflag2))
                                        fmt.Println("Here's the flag ==> ",enc2)
	 	
}else {
			fmt.Println("Go back")
		}	
	
}
		



func main() {

	ascii :=
		`


  ________               _____          __________________       
 /  _____/  ____   _____/ ____\__.__.   \______   \_____  \__  __
/   \  ___ /  _ \ /  _ \   __<   |  |    |       _/ _(__  <  \/ /
\    \_\  (  <_> |  <_> )  |  \___  |    |    |   \/       \   / 
 \______  /\____/ \____/|__|  / ____|____|____|_  /______  /\_/  
        \/                    \/   /_____/      \/       \/      

`
    fmt.Println(ascii)
	var x string = "0x0y7tyb" 


	var y *string


	y = &x 


	var  compare = "string"
	var setone  *string


	setone = &compare
	    

		if setone == y{
			
				one();
			}else
                                               
		{
        fmt.Println("Make sure to have a look at the compare statements :) ")
    }
}

