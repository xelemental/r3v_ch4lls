//Author: Elemental X
//Category : Reverse Engineering

package main

import (
        "fmt"
        "encoding/hex"
)
 
func ezdecode() {

        var chars = []byte{110 ,101 ,111 ,112 ,104 ,121 ,116 ,101}
        str1 := string(chars)
        str2 := (str1)
        hex1 := hex.EncodeToString([]byte(str2))
		fmt.Println("Enter that magic string : ")
		var check2 string
		fmt.Scanln(&check2)
		if hex1 == check2 {
			
		var chars = []byte{115 ,116 ,114 ,105 ,110, 103}
        str3 := string(chars)
        str4 := (str3)
        hex2 := hex.EncodeToString([]byte(str4))
        fmt.Println("Here's the final magic hash ==> " , hex2)
		} else {
				fmt.Println("Definitely not the key")
				}
		
}
		
		
		
func encode() {
     var(
	  varC = "nullv0id"
	  varD = "strcomp"
	  )
	  varE := (len(varC))
	  varF := (len(varD))
	  if varE == varF {
	  ezdecode();
	 } else {
	 	fmt.Println("Not the correct door , please go back")
	} 
	}
	
 func main() {
            fmt.Println("Enter the key : ")
			fmt.Println()
 			var finalcheck string = "676f6f66"
			var input string 
			fmt.Scanln(&input)
			if input == finalcheck{
           encode();
		   } else {
		     		fmt.Println("Enter the correct key")
	}
}	

  

  
