package main

import (
	"flag"
	"fmt"
	"io/fs"
	"log"
	"os"
	"path/filepath"
	"time"
)

const cmdName = "expose"
const cmdVersion = "0.0.1"

var walkRoot = flag.String("walkroot", ".", "the root for the walkfunk")
var path = flag.String("path", os.Getenv("PWD"), "give/file it a path to watch")

// FileNode is a single file as a node in the files tree
type FileNode struct {
	// Path and Name
	Path string
	Name string

	// Mode Data
	Mode    fs.FileMode
	ModTime time.Time

	// File Meta Data
	IsDir  bool
	IsFile bool
	IsLink bool
	Size/*u*/ int64
}

// FileNodes is the tree struct of file nodes of a given path
type FileNodes struct {
	// the file Tree
	// All FileNodes as a map
	// of [*string]=path
	// and [..]*FileNode=FileInfo and Meta Data
	Nodes map[string]*FileNode

	// Meta Data about the Tree
	NodesCount uint64
	NodesSize  uint64 // byte/1024/1024
}

func NewNodes() *FileNodes {
	var f FileNodes
	f.Nodes = make(map[string]*FileNode)
	return &f

}

// Create creates a FileNode Tree
func (f *FileNodes) Create(path string) (*FileNodes, error) {
	var walkErr = filepath.WalkDir(
		*walkRoot,
		func(path string, entry fs.DirEntry, err error) error {
			info, _ := entry.Info()
			mode := info.Mode()
			f.Nodes[path] = &FileNode{
				Path:    path,
				Name:    info.Name(),
				Mode:    info.Mode(),
				ModTime: info.ModTime(),
				IsDir:   mode.IsDir(),
				IsFile:  mode.IsRegular(),
				IsLink:  info.Mode()&os.ModeSymlink != 0,
				Size:    info.Size(),
			}
			return err
		})
	return f, walkErr
}

func main() {
	flag.Parse()

	// 	flag.Usage = func() {
	// 		fmt.Fprintf(os.Stderr, "Usage: \t")
	// 		fmt.Fprintf(os.Stderr, "\t%s:\n", cmdName)
	// 		flag.VisitAll(func(f *flag.Flag) {
	// 			fmt.Fprintf(os.Stderr, "TEST: %v\n", f.Usage) // f.Name, f.Value
	// 		})
	// 	}

	// 	flagset := make(map[string]bool)
	// 	if flagset["path"] && flagset["walkRoot"] {
	if path != nil {
		nodes := &FileNodes{}
		nodes.Nodes = make(map[string]*FileNode)
		n, err := nodes.Create(*path)
		if err != nil {
			log.Fatalf("could not create file tree from path: %s Err: %v\n", *path, err)
			return
		}

		for k, v := range n.Nodes {
			fmt.Printf("K: %v | V: %v", k, v)
		}

		//{ K: src/watch.go
		//| V: &{src/watch.go
		//			 watch.go
		//			-rw-r--r--
		//			2021-07-10 14:44:54.50443613 +0200 CEST
		//			false
		//			true
		//			false
		//			0
		//	}

		// fmt.Printf("Nodes for %s: \n\n%v\n", *path, n)
	}
	// 	} else {
	// 		// flag.Usage()
	// 		fmt.Fprintf(os.Stderr, "Usage: \t")
	// 		fmt.Fprintf(os.Stderr, "\t%s:\n", cmdName)
	// 		flag.VisitAll(func(f *flag.Flag) {
	// 			fmt.Fprintf(os.Stderr, "TEST: %v\n", f.Usage) // f.Name, f.Value
	// 		})
	// 	}
	//
}
