package main 

import (
    "fmt"
)

func one(){
    var a []byte
    key := "p0s1x4"
//    var b [6]byte
    copy(a[:], key)
//    c := []byte(key)
    d := byte('e')
    xorer := make([]byte,6)
    addup := make([]byte,6)
    for i := 0; i < 6; i++ {

        xorer[i] = key[i] ^ d
        
        for i := 0; i < 6; i++ {
                    addup[i] += xorer[i]

            if addup[0] == 25{
                var twbytes [1]string
                twbytes[0] = "0x78GH8I02UY20" //The static blockchain address's first 14 chars to be replaced later
                fmt.Println(twbytes[0])

         }
            if addup[1] == 3{
                var twbytes1 [1]string
                twbytes1[0] = "D4CE5F23aa7e48" //To be replaced
                fmt.Println(twbytes1[0])
    }
            if addup[3] == 45{
                var twbytes2 [1]string
                twbytes2[0] = "6228c46C4D99a4"  //To be replaced
                fmt.Println(twbytes2[0])
                
            }
    

        }
    }
}


func main(){
    fmt.Println("Welcome to the corrupted battle.......Enter 1 to continue..")
    var first int 
    fmt.Scanln(&first)
    z := 1
    if first == z{
    fmt.Println("The program has crashed , please fix to gain the address")    
    var theArray [3]string
    theArray[0] = "0x00s4r"  // Assign a value to the first element
    theArray[1] = "0x00s3r" // Assign a value to the second element
    theArray[2] = "0x00s2r"
    
        if theArray[0] == "bl0ck"{
            one();
        }
    }    
}
