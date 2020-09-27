
//Author : Elemental X 
//Category : Reversing


package main

import (
        "crypto/md5"
        "fmt"
        "encoding/hex"
    )


        func notmain() {

                fmt.Println("Enter the secretcode ==>  ")
                var check string
                fmt.Scanln(&check)
                var keys = []byte{101 ,108 ,105 ,116 ,101 ,104, 111 ,111 ,100}
                notstr := string(keys)
                notstr2 := (notstr)
                enc := hex.EncodeToString([]byte(notstr2))
                if enc == check {
                                        var flag = []byte{114 ,101 ,118 ,112 ,119 ,110}
                                        finflag := string(flag)
                                        finflag2 := (finflag)
                                         enc2    :=hex.EncodeToString([]byte(finflag2))
                                        fmt.Println("Here's the flag ==> ",enc2)
                                }else {
                                        data := []byte(enc)
                                        fmt.Printf("You are lost , please go back ==> %x", md5.Sum(data))
                                }
}

        func main(){

            fmt.Println("Enter the key : ")
            fmt.Println()
            var finalcheck string = "9uyh897"
            var input string 
            fmt.Scanln(&input)
            if input == finalcheck{
            notmain();
           } else {
                     fmt.Println("Enter the correct key")
    }
}
