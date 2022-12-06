package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func read_input_lines()
f, err := os.Open("/tmp/dat")
check(err)

