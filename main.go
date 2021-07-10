package main

import (
	"flag"
	"fmt"
	"io/fs"
	"log"
	"os"
	"path/filepath"
	"time"

	"github.com/fsnotify/fsnotify"
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
	Size   int64
}

// FileNodes is the tree struct of file nodes of a given path
type FileNodes struct {
	// the file Tree
	// All FileNodes as a map
	// of [*string]=path
	// and [..]*FileNode=FileInfo and Meta Data
	Nodes map[string]*FileNode

	// Meta Data about the Tree
	// NodesCount uint64
	// NodesSize  uint64 // byte/1024/1024
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
	if path != nil {
		nodes := &FileNodes{}
		nodes.Nodes = make(map[string]*FileNode)
		n, err := nodes.Create(*path)
		if err != nil {
			log.Fatalf("could not create file tree from path: %s Err: %v\n", *path, err)
			return
		}

		watcher, err := fsnotify.NewWatcher()
		if err != nil {
			fmt.Println("ERROR", err)
		}

		for path, filedata := range n.Nodes {

		}
	}
}
