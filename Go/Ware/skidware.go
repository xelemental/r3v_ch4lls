package main

import (

	"crypto/aes"
	"encoding/hex"
	"fmt"
	"os"
)

func main() {
	Encryptfinal()
	f, err := os.Create("encryptedmessage.txt")
	if err != nil {
		fmt.Println(err)
		return
	}
	l, err := f.WriteString("3d840a47b5ce38be626fa13500c53b5b")
	if err != nil {
		fmt.Println(err)
		f.Close()
		return
     	}
	fmt.Println(l, "The message has been encrypted and written")
	err = f.Close()
	if err != nil {
		fmt.Println(err)
		return
	}
}

	func Encryptfinal(){

       key := "thisis32bitlongpassphraseimusing"

       pt :=  "321174068998067 98980909"

       c := EncryptAES([]byte(key), pt)

       fmt.Println("This is the only message--------> "+ c)
 }


func EncryptAES(key []byte, plaintext string) string {


	c, err := aes.NewCipher(key)
	CheckError(err)

        out := make([]byte, len(plaintext))

	c.Encrypt(out, []byte(plaintext))

	return hex.EncodeToString(out)
}

func CheckError(err error) {
	if err != nil {
		panic(err)
	}
}



