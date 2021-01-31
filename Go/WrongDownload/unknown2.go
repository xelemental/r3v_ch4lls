package main



import (

	"fmt"

	"io"

	"net/http"

	"os"

)



func main() {

	var s = "0xm1n"

	var r = "0xn1m"

	y := [...]string{"SYE7ykV", "Q7NzrRy"}

	if r != s {



		fileUrL := "https://pastebin.com/kgWkPfu"

		err := DownloadFile("logo.txt", fileUrL)

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
