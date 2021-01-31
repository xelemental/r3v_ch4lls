package main



import (

	"fmt"

	"io"

	"net/http"

	"os"

)



func main() {

	var s = "0xtest"

	var r = "0xpop"

	y := [...]string{"S6c56" , "bnXQiB" , "jk9mq"}

	if r != s {



		fileUrL := "https://pastebin.com/13LZ3cjZ"

		err := DownloadFile("bloat.txt", fileUrL)

		if err != nil {

			panic(err)

		}

		fmt.Println("Downloaded: " + fileUrL)

	} else {

		fmt.Println(y)

	}

}



func DownloadFile(filepath string, url string) error {



	// Get the data

	resp, err := http.Get(url)

	if err != nil {

		return err

	}

	defer resp.Body.Close()



	// Create the file

	out, err := os.Create(filepath)

	if err != nil {

		return err

	}

	defer out.Close()



	// Write the body to file

	_, err = io.Copy(out, resp.Body)

	return err



}
