package main

import (
	"flag"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
)

var path = flag.String("path", os.Getenv("PWD"), "give it a path to watch")

// FileInfo ...
type FileInfo struct {
	fs.FileInfo
}

func main() {
	flag.Parse()

	if path != nil {
		var err = filepath.WalkDir(".", func(path string, d fs.DirEntry, err error) error {
			fmt.Println(path)
			fmt.Println(d.Name())
			fmt.Println(d.IsDir())
			fmt.Println(d.Type())
			fmt.Println(d.Info())
			return err
		})
		if err != nil {
			fmt.Println("test")
		}
	} else {
		flag.PrintDefaults()
	}
}
